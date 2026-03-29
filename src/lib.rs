use glam::Vec2;
use mikage::wgpu;
use mikage::winit::dpi::PhysicalSize;
use mikage::{
    App, Camera2d, FrameContext, GpuContext, InstanceRenderer, InstanceRendererConfig, RunConfig,
    SceneBinding, ShaderProcessor, UpdateContext,
};

mod controller;
pub mod tiles;
pub mod utils;

use controller::{LastViewState, SpectreInstance, TilesController};

struct SpectreApp {
    renderer: InstanceRenderer<SpectreInstance>,
    scene: SceneBinding,
    controller: TilesController,
    last_view: LastViewState,
}

impl SpectreApp {
    fn new(gpu: &GpuContext, _size: PhysicalSize<u32>) -> Self {
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
            last_view: LastViewState::default(),
        }
    }
}

impl App for SpectreApp {
    type Camera = Camera2d;

    fn update(&mut self, ctx: &mut UpdateContext<Camera2d>) {
        let window_size = (ctx.window_size.width, ctx.window_size.height);

        // シーンユニフォーム更新
        let aspect = window_size.0 as f32 / window_size.1.max(1) as f32;
        self.scene
            .update_from_camera(&ctx.gpu.queue, ctx.camera, aspect);

        // カメラのビューに基づいてbboxを計算
        let (vp_min, vp_max) = ctx.camera.viewport_bounds(aspect);
        let half_size = (vp_max - vp_min) * 0.5 * 1.5; // 1.5倍のマージン
        let center = (vp_min + vp_max) * 0.5;
        const MIN_SIZE: f32 = 15.0;
        let half_size = Vec2::new(half_size.x.max(MIN_SIZE), half_size.y.max(MIN_SIZE));
        let bbox = crate::utils::Aabb::from_min_max(center - half_size, center + half_size);

        // タイル更新（expand が発生した場合は同一フレーム内で再計算、最大3回）
        for _ in 0..3 {
            match controller::update_tiles(&mut self.controller, &mut self.last_view, &bbox) {
                Some(instances) => {
                    self.renderer.update_instances(ctx.gpu, &instances);
                    if !self.last_view.expanded {
                        break;
                    }
                }
                None => break,
            }
        }
    }

    fn encode(&mut self, ctx: &mut FrameContext<Camera2d>) {
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
        });

        pass.set_bind_group(0, self.scene.bind_group(), &[]);
        self.renderer.render(&mut pass);
    }
}

pub fn run() {
    let mut camera = Camera2d::default();
    camera.zoom = 0.028;
    camera.damping = 0.95;
    camera.min_zoom = 0.003;
    camera.max_zoom = 0.12;
    camera.zoom_speed = 0.2;
    camera.zoom_smoothing = 0.2;

    let mut config = RunConfig::new("Infinite Spectres").with_camera(camera);
    config.sample_count = 4;
    mikage::run(
        |gpu: &GpuContext, size: PhysicalSize<u32>| SpectreApp::new(gpu, size),
        config,
    );
}
