use anchor::Anchor;
use angle::Angle;
use bevy::{asset::RenderAssetUsages, prelude::*, render::mesh::PrimitiveTopology};
use lyon_tessellation::{
    geom::Point, geometry_builder::simple_builder, path::Path, FillOptions, FillTessellator,
    VertexBuffers,
};
use spectre::{Spectre, SuperSpectre};

mod anchor;
mod angle;
mod camera;
mod spectre;
mod spectre_like;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (camera::setup_camera, setup_tiles))
        .add_systems(Update, camera::camera_movement_system)
        .run();
}

/// タイルを配置するシステム
fn setup_tiles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mesh = setup_mesh(&mut meshes);
    let cluster = SuperSpectre::new_with_anchor(4, Vec2::ZERO, Anchor::Anchor1, Angle::ZERO);
    let mut counter = 0;
    for spectre in cluster.spectres() {
        spawn_tile(&mut commands, &mesh, &mut materials, spectre);
        counter += 1;
    }
    println!("counter: {}", counter);
}

/// タイルを描画する
fn spawn_tile(
    commands: &mut Commands,
    mesh: &Handle<Mesh>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    spectre: &Spectre,
) {
    // タイルのサイズを設定
    const TILE_SIZE: f32 = 10.0;

    let position = (spectre.anchor(Anchor::Anchor1)
        + spectre.anchor(Anchor::Anchor2)
        + spectre.anchor(Anchor::Anchor3)
        + spectre.anchor(Anchor::Anchor4))
        / 4.0;

    // angleから色相を計算（30度ごと）
    let hue = spectre.angle.value() as f32 * 30.0;
    // positionら彩度を計算（0.333-1.0）
    let saturation = (1.166 * position.x).sin() * 0.333 + 0.666;
    // HSVからRGBに変換（彩度80%、明度80%）
    let color = Color::hsl(hue, saturation, 0.8);
    // 位置
    let transform = Transform::from_scale(Vec3::splat(TILE_SIZE))
        .mul_transform(Transform::from_translation(
            spectre.anchor(Anchor::Anchor1).extend(0.0),
        ))
        .mul_transform(Transform::from_rotation(Quat::from_rotation_z(
            spectre.angle.to_radians(),
        )));

    // タイルのエンティティを生成
    commands.spawn((
        Mesh2d(mesh.clone()),
        transform,
        MeshMaterial2d(materials.add(color)),
    ));
}

fn setup_mesh(meshes: &mut ResMut<Assets<Mesh>>) -> Handle<Mesh> {
    let mut path_builder = Path::builder();
    let points = Spectre::new_with_anchor(Vec2::ZERO, Anchor::Anchor1, Angle::ZERO).all_points();
    path_builder.begin(Point::new(points[0].x, points[0].y));
    for point in points.iter().skip(1) {
        path_builder.line_to(Point::new(point.x, point.y));
    }
    path_builder.close();
    let path = path_builder.build();

    let mut buffers: VertexBuffers<Point<f32>, u16> = VertexBuffers::new();
    {
        let mut vertex_builder = simple_builder(&mut buffers);
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
            .into_iter()
            .map(|p| [p.x, p.y, 0.0])
            .collect::<Vec<_>>(),
    );
    mesh.insert_indices(bevy::render::mesh::Indices::U16(buffers.indices));
    meshes.add(mesh)
}
