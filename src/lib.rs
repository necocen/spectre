use mikage::dpi::PhysicalSize;
use mikage::wgpu;
use mikage::{
    App, Camera2d, GpuContext, InstanceRenderer, InstanceRendererConfig, RedrawPolicy,
    RenderContext, RenderTargetConfig, RenderUpdateContext, RunConfig, RunError, SceneBinding,
    ShaderProcessor, SimulationPolicy,
};

mod camera;
mod controller;
pub mod tiles;
pub mod utils;

pub use controller::{SpectreInstance, TilesController};

struct SpectreApp {
    renderer: InstanceRenderer<SpectreInstance>,
    scene: SceneBinding,
    controller: TilesController,
}

impl SpectreApp {
    fn new(gpu: &GpuContext, target: RenderTargetConfig, _size: PhysicalSize<u32>) -> Self {
        let scene = SceneBinding::new(&gpu.device);

        // シェーダーを解決
        let sp = ShaderProcessor::new();
        let shader_src = include_str!("instancing.wgsl");
        let resolved = sp.resolve(shader_src).expect("failed to resolve shader");

        // Spectreタイルのメッシュを生成
        let (positions, normals, indices) = controller::create_spectre_mesh();

        // InstanceRendererを作成
        let config = InstanceRendererConfig {
            vertex_entry: "vertex",
            fragment_entry: "fragment",
            depth: false,
            storage_binding: false,
        };
        let renderer = InstanceRenderer::<SpectreInstance>::with_shader(
            gpu,
            target,
            scene.layout(),
            &positions,
            &normals,
            &indices,
            &resolved,
            config,
        );

        Self {
            renderer,
            scene,
            controller: TilesController::new(),
        }
    }
}

impl App for SpectreApp {
    type Camera = Camera2d;

    fn prepare_render(&mut self, ctx: &mut RenderUpdateContext<Camera2d>) {
        let target_size = ctx.target_size;

        // シーンユニフォーム更新
        let aspect = target_size.width as f32 / target_size.height.max(1) as f32;
        self.scene
            .update_from_camera(&ctx.gpu.queue, ctx.camera, aspect);

        // カメラのビューに基づいてbboxを計算
        let (vp_min, vp_max) = ctx.camera.viewport_bounds(aspect);
        let viewport = crate::utils::Aabb::from_min_max(vp_min, vp_max);
        if let Some(instances) = self.controller.update_view(&viewport) {
            self.renderer.update_instances(ctx.gpu, instances);
        }
    }

    fn render(&mut self, ctx: &mut RenderContext<Camera2d>) {
        let mut pass = ctx.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("spectre_pass"),
            color_attachments: &[Some(ctx.color_attachment(wgpu::Operations {
                load: wgpu::LoadOp::Clear(wgpu::Color {
                    r: 1.0,
                    g: 1.0,
                    b: 1.0,
                    a: 1.0,
                }),
                store: wgpu::StoreOp::Store,
            }))],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });

        pass.set_bind_group(0, self.scene.bind_group(), &[]);
        self.renderer.render(&mut pass);
    }
}

pub fn run() -> Result<(), RunError> {
    let mut config = RunConfig::new("Infinite Spectres")
        .with_camera(camera::new())
        .with_redraw_policy(RedrawPolicy::Reactive)
        // Tiles are updated during rendering; there is no simulation to tick.
        .with_simulation_policy(SimulationPolicy::Manual);
    config.sample_count = 4;
    mikage::run(SpectreApp::new, config)
}
