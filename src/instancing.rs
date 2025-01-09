use bevy::{
    asset::embedded_asset,
    core_pipeline::core_2d::Transparent2d,
    ecs::{
        query::QueryItem,
        system::{
            lifetimeless::{Read, SRes},
            SystemParamItem,
        },
    },
    math::FloatOrd,
    prelude::*,
    render::{
        extract_component::{ExtractComponent, ExtractComponentPlugin},
        mesh::{
            allocator::MeshAllocator, MeshVertexBufferLayoutRef, RenderMesh, RenderMeshBufferInfo,
            VertexBufferLayout,
        },
        render_asset::RenderAssets,
        render_phase::{
            AddRenderCommand as _, DrawFunctions, PhaseItem, PhaseItemExtraIndex, RenderCommand,
            RenderCommandResult, SetItemPipeline, TrackedRenderPass, ViewSortedRenderPhases,
        },
        render_resource::{
            Buffer, BufferInitDescriptor, BufferUsages, ColorTargetState, ColorWrites,
            PipelineCache, RenderPipelineDescriptor, SpecializedMeshPipeline,
            SpecializedMeshPipelineError, SpecializedMeshPipelines, TextureFormat, VertexAttribute,
            VertexFormat, VertexStepMode,
        },
        renderer::RenderDevice,
        view::{ExtractedView, RenderVisibleEntities},
        Render, RenderApp, RenderSet,
    },
    sprite::{
        Mesh2dPipeline, Mesh2dPipelineKey, RenderMesh2dInstances, SetMesh2dBindGroup,
        SetMesh2dViewBindGroup,
    },
};

const SHADER_ASSET_PATH: &str = "embedded://spectre/instancing.wgsl";

pub struct CustomMaterialPlugin;

impl Plugin for CustomMaterialPlugin {
    fn build(&self, app: &mut App) {
        embedded_asset!(app, "./instancing.wgsl");
        app.add_plugins(ExtractComponentPlugin::<InstanceMaterialData>::default());
        app.sub_app_mut(RenderApp)
            .add_render_command::<Transparent2d, DrawCustom>()
            .init_resource::<SpecializedMeshPipelines<CustomPipeline>>()
            .add_systems(
                Render,
                (
                    queue_custom.in_set(RenderSet::QueueMeshes),
                    prepare_instance_buffers.in_set(RenderSet::PrepareResources),
                ),
            );
    }

    fn finish(&self, app: &mut App) {
        app.sub_app_mut(RenderApp).init_resource::<CustomPipeline>();
    }
}

#[derive(Component, Deref)]
pub struct InstanceMaterialData(pub Vec<InstanceData>);

impl ExtractComponent for InstanceMaterialData {
    type QueryData = &'static InstanceMaterialData;
    type QueryFilter = ();
    type Out = Self;

    fn extract_component(item: QueryItem<'_, Self::QueryData>) -> Option<Self> {
        Some(InstanceMaterialData(item.0.clone()))
    }
}

#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct InstanceData {
    pub position: Vec3,
    pub angle: f32,
}

#[allow(clippy::too_many_arguments)]
fn queue_custom(
    transparent_2d_draw_functions: Res<DrawFunctions<Transparent2d>>,
    custom_pipeline: Res<CustomPipeline>,
    mut pipelines: ResMut<SpecializedMeshPipelines<CustomPipeline>>,
    pipeline_cache: Res<PipelineCache>,
    meshes: Res<RenderAssets<RenderMesh>>,
    render_mesh_instances: Res<RenderMesh2dInstances>,
    mut transparent_render_phases: ResMut<ViewSortedRenderPhases<Transparent2d>>,
    views: Query<(Entity, &RenderVisibleEntities, &ExtractedView, &Msaa)>,
) {
    if render_mesh_instances.is_empty() {
        return;
    }
    let draw_custom = transparent_2d_draw_functions.read().id::<DrawCustom>();

    for (view_entity, visible_entities, view, msaa) in &views {
        let Some(transparent_phase) = transparent_render_phases.get_mut(&view_entity) else {
            continue;
        };
        let mesh_key = Mesh2dPipelineKey::from_msaa_samples(msaa.samples())
            | Mesh2dPipelineKey::from_hdr(view.hdr);

        for (entity, main_entity) in visible_entities.iter::<With<Mesh2d>>() {
            let Some(mesh_instance) = render_mesh_instances.get(main_entity) else {
                continue;
            };
            let Some(mesh) = meshes.get(mesh_instance.mesh_asset_id) else {
                continue;
            };
            let mesh2d_key =
                mesh_key | Mesh2dPipelineKey::from_primitive_topology(mesh.primitive_topology());
            let pipeline = pipelines
                .specialize(&pipeline_cache, &custom_pipeline, mesh2d_key, &mesh.layout)
                .unwrap();
            transparent_phase.add(Transparent2d {
                entity: (*entity, *main_entity),
                pipeline,
                draw_function: draw_custom,
                sort_key: FloatOrd(mesh_instance.transforms.world_from_local.translation.z),
                batch_range: 0..1,
                extra_index: PhaseItemExtraIndex::NONE,
            });
        }
    }
}

#[derive(Component)]
struct InstanceBuffer {
    buffer: Buffer,
    length: usize,
}

fn prepare_instance_buffers(
    mut commands: Commands,
    query: Query<(Entity, &InstanceMaterialData)>,
    render_device: Res<RenderDevice>,
) {
    for (entity, instance_data) in &query {
        let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
            label: Some("instance data buffer"),
            contents: bytemuck::cast_slice(instance_data.as_slice()),
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
        });
        if instance_data.is_empty() {
            // 0個で描画しようとすると落ちる
            continue;
        }
        commands.entity(entity).insert(InstanceBuffer {
            buffer,
            length: instance_data.len(),
        });
    }
}

#[derive(Resource)]
struct CustomPipeline {
    shader: Handle<Shader>,
    mesh_pipeline: Mesh2dPipeline,
}

impl FromWorld for CustomPipeline {
    fn from_world(world: &mut World) -> Self {
        CustomPipeline {
            shader: world.load_asset(SHADER_ASSET_PATH),
            mesh_pipeline: Mesh2dPipeline::from_world(world),
        }
    }
}

impl SpecializedMeshPipeline for CustomPipeline {
    type Key = Mesh2dPipelineKey;

    fn specialize(
        &self,
        key: Self::Key,
        layout: &MeshVertexBufferLayoutRef,
    ) -> Result<RenderPipelineDescriptor, SpecializedMeshPipelineError> {
        let mut descriptor = self.mesh_pipeline.specialize(key, layout)?;

        descriptor.vertex.shader = self.shader.clone();
        descriptor.vertex.buffers.push(VertexBufferLayout {
            array_stride: size_of::<InstanceData>() as u64,
            step_mode: VertexStepMode::Instance,
            attributes: vec![
                VertexAttribute {
                    format: VertexFormat::Float32x4,
                    offset: 0,
                    shader_location: 3, // shader locations 0-2 are taken up by Position, Normal and UV attributes
                },
            ],
        });

        descriptor.fragment.as_mut().unwrap().shader = self.shader.clone();
        descriptor.fragment.as_mut().unwrap().targets = vec![Some(ColorTargetState {
            format: TextureFormat::bevy_default(),
            blend: None,
            write_mask: ColorWrites::ALL,
        })];
        Ok(descriptor)
    }
}

type DrawCustom = (
    SetItemPipeline,
    SetMesh2dViewBindGroup<0>,
    SetMesh2dBindGroup<1>,
    DrawMeshInstanced,
);

struct DrawMeshInstanced;

impl<P: PhaseItem> RenderCommand<P> for DrawMeshInstanced {
    type Param = (
        SRes<RenderAssets<RenderMesh>>,
        SRes<RenderMesh2dInstances>,
        SRes<MeshAllocator>,
    );
    type ViewQuery = ();
    type ItemQuery = Read<InstanceBuffer>;

    #[inline]
    fn render<'w>(
        item: &P,
        _view: (),
        instance_buffer: Option<&'w InstanceBuffer>,
        (meshes, render_mesh_instances, mesh_allocator): SystemParamItem<'w, '_, Self::Param>,
        pass: &mut TrackedRenderPass<'w>,
    ) -> RenderCommandResult {
        // A borrow check workaround.
        let mesh_allocator = mesh_allocator.into_inner();

        let Some(mesh_instance) = render_mesh_instances.get(&item.main_entity()) else {
            return RenderCommandResult::Skip;
        };
        let Some(gpu_mesh) = meshes.into_inner().get(mesh_instance.mesh_asset_id) else {
            return RenderCommandResult::Skip;
        };
        let Some(instance_buffer) = instance_buffer else {
            return RenderCommandResult::Skip;
        };
        let Some(vertex_buffer_slice) =
            mesh_allocator.mesh_vertex_slice(&mesh_instance.mesh_asset_id)
        else {
            return RenderCommandResult::Skip;
        };

        pass.set_vertex_buffer(0, vertex_buffer_slice.buffer.slice(..));
        pass.set_vertex_buffer(1, instance_buffer.buffer.slice(..));

        match &gpu_mesh.buffer_info {
            RenderMeshBufferInfo::Indexed {
                index_format,
                count,
            } => {
                let Some(index_buffer_slice) =
                    mesh_allocator.mesh_index_slice(&mesh_instance.mesh_asset_id)
                else {
                    return RenderCommandResult::Skip;
                };

                pass.set_index_buffer(index_buffer_slice.buffer.slice(..), 0, *index_format);
                pass.draw_indexed(
                    index_buffer_slice.range.start..(index_buffer_slice.range.start + count),
                    vertex_buffer_slice.range.start as i32,
                    0..instance_buffer.length as u32,
                );
            }
            RenderMeshBufferInfo::NonIndexed => {
                pass.draw(vertex_buffer_slice.range, 0..instance_buffer.length as u32);
            }
        }
        RenderCommandResult::Success
    }
}
