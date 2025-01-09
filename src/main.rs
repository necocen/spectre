use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::{mesh::PrimitiveTopology, view::NoFrustumCulling}, window::PrimaryWindow,
};
use geometry::{Anchor, Geometry as _, Spectre, SuperSpectre};
use instancing::{CustomMaterialPlugin, InstanceData, InstanceMaterialData};
use lyon_tessellation::{
    geom::Point, geometry_builder::simple_builder, path::Path, FillOptions, FillTessellator,
    VertexBuffers,
};
use rstar::{primitives::CachedEnvelope, RTree, AABB};
use utils::{Angle, HexVec};

mod camera;
mod geometry;
mod instancing;
mod utils;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                // Web版はブラウザ全体に表示
                fit_canvas_to_parent: true,
                ..default()
            }),
            ..default()
        }))
        .add_plugins((camera::CameraPlugin, CustomMaterialPlugin))
        .add_systems(Startup, setup_tiles)
        .add_systems(Update, camera_view_system)
        .run();
}

/// タイルを配置するシステム
fn setup_tiles(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    let spectres_manager = SpectresManager::new();
    let mesh = setup_mesh(&mut meshes);
    // let spectres = spectres_manager.spectres.iter().map(to_instance_data).collect();
    commands.spawn((Mesh2d(mesh), InstanceMaterialData(vec![]), SpectreTag, NoFrustumCulling));
    // commands.spawn((Mesh2d(mesh), InstanceMaterialData(spectres), SpectreTag, NoFrustumCulling));
    commands.insert_resource(spectres_manager);
}

// タイルのサイズを設定
const TILE_SIZE: f32 = 10.0;

#[inline]
fn to_instance_data(spectre: &Spectre) -> InstanceData {
    let anchor_pos = spectre.point(Anchor::Anchor1).to_vec2() * TILE_SIZE;
    InstanceData {
        position: anchor_pos.extend(0.0),
        angle: spectre.angle.to_radians(),
    }
}

fn setup_mesh(meshes: &mut ResMut<Assets<Mesh>>) -> Handle<Mesh> {
    let mut path_builder = Path::builder();
    let points = Spectre::new_with_anchor(HexVec::ZERO, Anchor::Anchor1, Angle::ZERO).all_points();
    let points_vec2: Vec<Vec2> = points.iter().map(|p| p.to_vec2() * TILE_SIZE).collect();
    path_builder.begin(Point::new(points_vec2[0].x, points_vec2[0].y));
    for point in points_vec2.iter().skip(1) {
        path_builder.line_to(Point::new(point.x, point.y));
    }
    path_builder.close();
    let path = path_builder.build();

    let mut buffers: VertexBuffers<Point<f32>, u16> = VertexBuffers::new();
    {
        let mut vertex_builder = simple_builder(&mut buffers).with_inverted_winding(); // 反時計回りにする
        let mut tessellator = FillTessellator::new();
        let result =
            tessellator.tessellate_path(&path, &FillOptions::default(), &mut vertex_builder);
        assert!(result.is_ok());
    }
    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    );
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        buffers
            .vertices
            .iter()
            .map(|p| [p.x, p.y, 0.0])
            .collect::<Vec<_>>(),
    );
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_NORMAL,
        buffers
            .vertices
            .iter()
            .map(|_| [0.0, 0.0, 1.0])
            .collect::<Vec<_>>(),
    );
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_UV_0,
        buffers
            .vertices
            .iter()
            .map(|_| [0.0, 0.0])
            .collect::<Vec<_>>(),
    );
    mesh.insert_indices(bevy::render::mesh::Indices::U16(buffers.indices));
    meshes.add(mesh)
}

#[derive(Component)]
pub struct SpectreTag;


fn camera_view_system(
    manager: Res<SpectresManager>,
    windows: Query<&Window, With<PrimaryWindow>>,
    ortho_q: Query<(&OrthographicProjection, &GlobalTransform), With<Camera2d>>,
    mut entity_query: Query<&mut InstanceMaterialData, With<SpectreTag>>,
) {
    let window = windows.single();

    let (ortho, transform) = ortho_q.get_single().unwrap();

    // カメラの中心（ワールド座標）
    let camera_center = transform.translation().truncate();

    // スケールを考慮して、ウィンドウサイズから「表示半径」を求める
    let half_width = window.width() * 0.5 * transform.scale().x * ortho.scale * 1.5;
    let half_height = window.height() * 0.5 * transform.scale().y * ortho.scale * 1.5;

    let min = (camera_center - Vec2::new(half_width, half_height)) / TILE_SIZE;
    let max = (camera_center + Vec2::new(half_width, half_height)) / TILE_SIZE;
    let left = min.x;
    let right = max.x;
    let top = min.y;
    let bottom = max.y;

    // ここで計算した可視範囲に合わせてタイルの生成・破棄を行う
    let mut instance_data = Vec::<InstanceData>::with_capacity(100000);
    let spectres = manager
        .spectres
        .locate_in_envelope(&AABB::from_corners([left, top], [right , bottom])).map(|s| to_instance_data(s));
    instance_data.extend(spectres);
    entity_query.single_mut().0 = instance_data;
}

#[derive(Resource)]
struct SpectresManager {
    spectres: RTree<CachedEnvelope<Spectre>>,
}

impl SpectresManager {
    pub fn new() -> Self {
        let cluster = SuperSpectre::new_with_anchor(7, HexVec::ZERO, Anchor::Anchor1, Angle::ZERO);
        info!("cluster initialized");
        let mut spectres = RTree::new();
        for spectre in cluster.spectres() {
            spectres.insert(CachedEnvelope::new(*spectre));
        }
        info!("count: {}", spectres.size());
        Self { spectres }
    }
}
