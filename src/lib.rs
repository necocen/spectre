use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::{mesh::PrimitiveTopology, view::NoFrustumCulling},
    window::PrimaryWindow,
};
use geometry::{Anchor, Spectre};
use instancing::{InstanceData, InstanceMaterialData};
use lyon_tessellation::{
    geom::Point, geometry_builder::simple_builder, path::Path, FillOptions, FillTessellator,
    VertexBuffers,
};
use spectres_manager::SpectresManager;
use utils::{Aabb, Angle, HexVec};
#[cfg(target_family = "wasm")]
use wasm_bindgen::prelude::*;

mod camera;
pub mod geometry;
mod instancing;
mod spectres_manager;
pub mod utils;

#[cfg_attr(target_family = "wasm", wasm_bindgen)]
pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                // Web版はブラウザ全体に表示
                fit_canvas_to_parent: true,
                ..default()
            }),
            ..default()
        }))
        .add_plugins((camera::CameraPlugin, instancing::CustomMaterialPlugin))
        .add_systems(Startup, (setup_tiles, set_window_title))
        .init_resource::<LastViewState>()
        .add_systems(Update, camera_view_system)
        .run();
}

fn set_window_title(mut window_query: Query<&mut Window, With<PrimaryWindow>>) {
    if let Ok(mut window) = window_query.get_single_mut() {
        window.title = "Infinite Spectres".to_string();
    }
}

/// タイルを配置するシステム
fn setup_tiles(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    let spectres_manager = SpectresManager::new();
    let mesh = setup_mesh(&mut meshes);
    // let spectres = spectres_manager.spectres.iter().map(to_instance_data).collect();
    commands.spawn((Mesh2d(mesh), InstanceMaterialData(vec![]), NoFrustumCulling));
    commands.insert_resource(spectres_manager);
}

#[inline]
fn to_instance_data(spectre: &Spectre) -> InstanceData {
    let anchor_pos = spectre.coordinate(Anchor::Anchor1).to_vec2();
    InstanceData {
        position: anchor_pos.extend(0.0),
        angle: spectre.rotation().to_radians(),
    }
}

fn setup_mesh(meshes: &mut ResMut<Assets<Mesh>>) -> Handle<Mesh> {
    let mut path_builder = Path::builder();
    let points = Spectre::with_anchor(Anchor::Anchor1, HexVec::ZERO, Angle::ZERO).vertices();
    let points_vec2: Vec<Vec2> = points.iter().map(|p| p.to_vec2()).collect();
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

#[derive(Resource, Default)]
struct LastViewState {
    /// カメラの表示範囲
    bbox: Option<Aabb>,
    /// 前のフレームでタイルを拡大したかどうか
    expanded: bool,
}

fn camera_view_system(
    mut manager: ResMut<SpectresManager>,
    windows: Query<&Window, With<PrimaryWindow>>,
    ortho_q: Query<(&OrthographicProjection, &GlobalTransform), With<Camera2d>>,
    mut entity_query: Query<&mut InstanceMaterialData>,
    mut last_view: ResMut<LastViewState>,
) {
    let window = windows.single();

    let (ortho, transform) = ortho_q.get_single().unwrap();

    // カメラの中心（ワールド座標）
    let camera_center = transform.translation().truncate();

    // スケールを考慮して、ウィンドウサイズから「表示半径」を求める
    // あまり小さいとタイル表示数のゆらぎが大きくなって拡張判定に失敗するので、ある程度の大きさを最小値として設定する
    let half_width = f32::max(
        window.width() * 0.5 * transform.scale().x * ortho.scale,
        10.0,
    ) * 1.5;
    let half_height = f32::max(
        window.height() * 0.5 * transform.scale().y * ortho.scale,
        10.0,
    ) * 1.5;
    let min = camera_center - Vec2::new(half_width, half_height);
    let max = camera_center + Vec2::new(half_width, half_height);

    let bbox = Aabb::from_min_max(min, max);

    // 前フレームと同じbboxの場合は早期リターン
    if let Some(last_bbox) = last_view.bbox {
        if last_bbox == bbox && !last_view.expanded {
            return;
        }
    }
    last_view.bbox = Some(bbox);

    // bboxに含まれるタイルを取得してバッファを更新
    let mut instance_data = Vec::<InstanceData>::with_capacity(
        (entity_query.single().0.len() as f64 * 1.1).ceil() as usize,
    );
    manager.update(&bbox);
    let spectres = manager.spectres_in(&bbox);
    instance_data.extend(spectres.map(to_instance_data));
    entity_query.single_mut().0 = instance_data;

    // 描画対象タイルの重心の偏りによってタイル生成の要否を判定する
    // （欠けがある場合はその分だけ重心が偏るという考えかた）
    // FIXME: update_childrenの精度が高くないので、本当はタイルが存在するのに生成してしまうパターンがある
    let instance_data = &entity_query.single().0;
    last_view.expanded = false;
    if !instance_data.is_empty() {
        let center = (bbox.min + bbox.max) * 0.5;
        let barycenter = instance_data.iter().fold(Vec2::ZERO, |acc, data| {
            acc + (data.position.truncate() - center)
        }) / instance_data.len() as f32;
        if barycenter.length() > 5.0 {
            manager.expand();
            last_view.expanded = true;
        }
    }
}
