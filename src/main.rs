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
use spectre::{
    camera::CameraPlugin,
    geometry::{Aabb, Anchor, Geometry as _, Skeleton, Spectre, SuperSpectre},
    instancing::{CustomMaterialPlugin, InstanceData, InstanceMaterialData},
    utils::{Angle, HexVec},
};

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
        .add_plugins((CameraPlugin, CustomMaterialPlugin))
        .add_systems(Startup, setup_tiles)
        .add_systems(Update, camera_view_system)
        .run();
}

/// タイルを配置するシステム
fn setup_tiles(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    let spectres_manager = SpectresManager::new();
    let mesh = setup_mesh(&mut meshes);
    // let spectres = spectres_manager.spectres.iter().map(to_instance_data).collect();
    commands.spawn((
        Mesh2d(mesh),
        InstanceMaterialData(vec![]),
        SpectreTag,
        NoFrustumCulling,
    ));
    // commands.spawn((Mesh2d(mesh), InstanceMaterialData(spectres), SpectreTag, NoFrustumCulling));
    commands.insert_resource(spectres_manager);
}

#[inline]
fn to_instance_data(spectre: &Spectre) -> InstanceData {
    let anchor_pos = spectre.point(Anchor::Anchor1).to_vec2();
    InstanceData {
        position: anchor_pos.extend(0.0),
        angle: spectre.angle.to_radians(),
    }
}

fn setup_mesh(meshes: &mut ResMut<Assets<Mesh>>) -> Handle<Mesh> {
    let mut path_builder = Path::builder();
    let points = Spectre::new_with_anchor(HexVec::ZERO, Anchor::Anchor1, Angle::ZERO).all_points();
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

#[derive(Component)]
pub struct SpectreTag;

fn camera_view_system(
    mut manager: ResMut<SpectresManager>,
    windows: Query<&Window, With<PrimaryWindow>>,
    ortho_q: Query<(&OrthographicProjection, &GlobalTransform), With<Camera2d>>,
    mut entity_query: Query<&mut InstanceMaterialData, With<SpectreTag>>,
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

    // ここで計算した可視範囲に合わせてタイルの生成・破棄を行う
    let mut instance_data = Vec::<InstanceData>::with_capacity(
        (entity_query.single().0.len() as f64 * 1.1).ceil() as usize,
    );
    let aabb = Aabb::from_min_max(min, max);

    manager.spectres.update_children(&aabb);
    let spectres = manager.spectres.iter(aabb);
    instance_data.extend(spectres.map(to_instance_data));

    // 描画対象タイルの重心の偏りによってタイル生成の要否を判定する
    // （欠けがある場合はその分だけ重心が偏るという考えかた）
    // FIXME: update_childrenの精度が高くないので、本当はタイルが存在するのに生成してしまうパターンがある
    let center = (aabb.min + aabb.max) * 0.5;
    let barycenter = instance_data.iter().fold(Vec2::ZERO, |acc, data| {
        acc + (data.position.truncate() - center)
    }) / instance_data.len() as f32;
    if barycenter.length() > 5.0 {
        manager.expand();
    }
    entity_query.single_mut().0 = instance_data;
}

#[derive(Resource)]
struct SpectresManager {
    spectres: Box<SuperSpectre>,
}

impl SpectresManager {
    pub fn new() -> Self {
        let skeleton = Skeleton::new_with_anchor(5, HexVec::ZERO, Anchor::Anchor1, Angle::ZERO)
            .to_super_spectre(&Aabb::NULL);
        let spectres = Box::new(skeleton);
        Self { spectres }
    }

    pub fn expand(&mut self) {
        if self.spectres.level > 18 {
            warn!("Cannot expand more");
            return;
        }

        // 現在のSuperSpectreをAまたはFとして上位のSuperSpectreを生成する
        let mut spectres = Box::new(
            Skeleton::new_with_anchor(1, HexVec::ZERO, Anchor::Anchor1, Angle::ZERO)
                .to_super_spectre(&Aabb::NULL),
        );
        std::mem::swap(&mut self.spectres, &mut spectres);
        if spectres.level % 2 == 0 {
            info!("Expand from A");
            self.spectres = Box::new(SuperSpectre::from_child_a(*spectres));
        } else {
            info!("Expand from F");
            self.spectres = Box::new(SuperSpectre::from_child_f(*spectres));
        }
    }
}
