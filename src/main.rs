use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::{mesh::PrimitiveTopology, view::NoFrustumCulling},
};
use geometry::{Anchor, Geometry as _, Spectre, SuperSpectre};
use instancing::{CustomMaterialPlugin, InstanceData, InstanceMaterialData};
use lyon_tessellation::{
    geom::Point, geometry_builder::simple_builder, path::Path, FillOptions, FillTessellator,
    VertexBuffers,
};
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
        .add_plugins(CustomMaterialPlugin)
        .add_systems(Startup, (camera::setup_camera, setup_tiles))
        .add_systems(Update, camera::camera_movement_system)
        .run();
}

/// タイルを配置するシステム
fn setup_tiles(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    let mesh = setup_mesh(&mut meshes);
    let cluster = SuperSpectre::new_with_anchor(7, HexVec::ZERO, Anchor::Anchor1, Angle::ZERO);

    let instance_data = cluster.spectres().map(to_instance_data).collect::<Vec<_>>();
    println!("counter: {}", instance_data.len());

    commands.spawn((
        Mesh2d(mesh),
        InstanceMaterialData(instance_data),
        NoFrustumCulling,
    ));
}

fn to_instance_data(spectre: &Spectre) -> InstanceData {
    // タイルのサイズを設定
    const TILE_SIZE: f32 = 10.0;

    let position = {
        let anchors = [
            spectre.point(Anchor::Anchor1),
            spectre.point(Anchor::Anchor2),
            spectre.point(Anchor::Anchor3),
            spectre.point(Anchor::Anchor4),
        ];
        let sum = anchors.into_iter().fold(HexVec::ZERO, |acc, p| acc + p);
        sum.to_vec2() / 4.0
    };

    // angleから色相を計算（30度ごと）
    let hue = spectre.angle.value() as f32 * 30.0;
    // positionから彩度を計算（0.333-1.0）
    let saturation = f32::sin(1.166 * position.x) * 0.333 + 0.666;
    // HSVからRGBに変換（明度70%）
    let color = Color::hsl(hue, saturation, 0.7).with_alpha(1.0);

    let anchor_pos = spectre.point(Anchor::Anchor1).to_vec2();
    InstanceData {
        position: anchor_pos.extend(0.0),
        scale: TILE_SIZE,
        color: color.to_srgba().to_f32_array(),
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
