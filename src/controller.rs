use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::{mesh::PrimitiveTopology, view::NoFrustumCulling},
    window::PrimaryWindow,
};
use lyon_tessellation::{
    geom::Point, geometry_builder::simple_builder, path::Path, FillOptions, FillTessellator,
    VertexBuffers,
};

use crate::{
    instancing::{InstanceData, InstanceMaterialData},
    tiles::{Anchor, Skeleton, Spectre, SpectreCluster, SpectreIter},
    utils::{Aabb, Angle, HexVec},
};

pub struct TilesControllerPlugin;

impl Plugin for TilesControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup)
            .init_resource::<LastViewState>()
            .add_systems(Update, camera_view_system);
    }
}

fn startup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    let mesh = setup_mesh(&mut meshes);
    commands.spawn((Mesh2d(mesh), InstanceMaterialData(vec![]), NoFrustumCulling));
    commands.insert_resource(TilesController::new());
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

fn camera_view_system(
    mut manager: ResMut<TilesController>,
    windows: Query<&Window, With<PrimaryWindow>>,
    ortho_q: Query<(&OrthographicProjection, &GlobalTransform), With<Camera2d>>,
    mut entity_query: Query<&mut InstanceMaterialData>,
    mut last_view: ResMut<LastViewState>,
) {
    let window = windows.single();
    let (ortho, transform) = ortho_q.get_single().unwrap();

    // カメラの中心（ワールド座標）
    let camera_center = transform.translation().truncate();

    // スケールを考慮して、ウィンドウサイズからbboxを求める
    // あまり小さいとタイル表示数のゆらぎが大きくなって拡張判定に失敗するので、ある程度の大きさを最小値として設定する
    const MIN_SIZE: f32 = 15.0;
    let half_width = f32::max(
        window.width() * 0.5 * transform.scale().x * ortho.scale * 1.5,
        MIN_SIZE,
    );
    let half_height = f32::max(
        window.height() * 0.5 * transform.scale().y * ortho.scale * 1.5,
        MIN_SIZE,
    );
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

#[inline]
fn to_instance_data(spectre: &Spectre) -> InstanceData {
    let anchor_pos = spectre.coordinate(Anchor::Anchor1).to_vec2();
    InstanceData {
        position: anchor_pos.extend(0.0),
        angle: spectre.rotation().to_radians(),
    }
}

#[derive(Resource)]
struct TilesController {
    spectres: Box<SpectreCluster>,
}

impl TilesController {
    pub fn new() -> Self {
        let skeleton = Skeleton::with_anchor(Anchor::Anchor1, HexVec::ZERO, Angle::ZERO, 5)
            .to_spectre_cluster(&Aabb::NULL);
        let spectres = Box::new(skeleton);
        Self { spectres }
    }

    pub fn expand(&mut self) {
        if self.spectres.level() > 18 {
            tracing::warn!("Cannot expand more");
            return;
        }

        // 現在のSpectreClusterをAまたはFとして上位のSpectreClusterを生成する
        let mut spectres = Box::new(
            Skeleton::with_anchor(Anchor::Anchor1, HexVec::ZERO, Angle::ZERO, 1)
                .to_spectre_cluster(&Aabb::NULL),
        );
        std::mem::swap(&mut self.spectres, &mut spectres);
        if spectres.level() % 2 == 0 {
            tracing::info!("Expand from A");
            self.spectres = Box::new(SpectreCluster::with_child_a(*spectres));
        } else {
            tracing::info!("Expand from F");
            self.spectres = Box::new(SpectreCluster::with_child_f(*spectres));
        }
    }

    pub fn update(&mut self, bbox: &Aabb) {
        self.spectres.update(bbox);
    }

    pub fn spectres_in(&self, bbox: &Aabb) -> SpectreIter {
        self.spectres.spectres_in(*bbox)
    }
}

#[derive(Resource, Default)]
struct LastViewState {
    /// カメラの表示範囲
    bbox: Option<Aabb>,
    /// 前のフレームでタイルを拡大したかどうか
    expanded: bool,
}
