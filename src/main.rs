use anchor::Anchor;
use angle::Angle;
use bevy::{prelude::*, window::PrimaryWindow};
use bevy_prototype_lyon::prelude::*;
use once_cell::unsync::Lazy;
use spectre::{Spectre, SuperSpectre};
use spectre_like::SpectreLike as _;

mod anchor;
mod angle;
mod spectre;
mod spectre_like;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, ShapePlugin))
        .add_systems(Startup, (setup_camera, setup_tiles))
        .add_systems(Update, camera_movement_system)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands
        .spawn((Camera2d, Msaa::Sample4))
        .insert(CameraController::default());
}

/// タイルを描画する
///
/// # Arguments
/// * `commands` - Bevyのコマンドバッファ
/// * `spectre` - 描画するタイル
/// * `index` - タイルのインデックス（未使用）
///
/// # Details
/// タイルの頂点から多角形を生成し、angleに応じた色で描画します。
/// 色はHSVカラーモデルを使用し、以下のように決定されます：
/// - 色相（H）：angleを30度ごとに割り当て（0-360度）
/// - 彩度（S）：位置によって決定（0.333-1.0）
/// - 明度（V）：80%で固定
fn spawn_tile(commands: &mut Commands, spectre: &Spectre) {
    // タイルのサイズを設定
    const TILE_SIZE: f32 = 10.0;
    // TODO: この部分は定数にできるはず
    // タイルの頂点から多角形を生成
    let path: Lazy<Path> = Lazy::new(|| {
        GeometryBuilder::build_as(&shapes::Polygon {
            points: Spectre::new_with_anchor(Vec2::ZERO, Anchor::Anchor1, Angle::ZERO).all_points(),
            closed: true,
        })
    });

    let position = (spectre.anchor(Anchor::Anchor1)
        + spectre.anchor(Anchor::Anchor2)
        + spectre.anchor(Anchor::Anchor3)
        + spectre.anchor(Anchor::Anchor4))
        / 4.0;

    // angleから色相を計算（30度ごと）
    let hue = spectre.angle.value() as f32 * 30.0;
    // positionら彩度を計算（0.333-1.0）
    let saturation = (1.166 * position.x).sin() * 0.333 + 0.666;
    // println!(
    //     "angle: {}, length: {}, hue: {}, saturation: {}",
    //     spectre.angle.value(),
    //     position.length(),
    //     hue,
    //     saturation
    // );
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
        ShapeBundle {
            path: path.clone(),
            transform,
            ..default()
        },
        Fill::color(color),
    ));
}

/// タイルを配置するシステム
fn setup_tiles(mut commands: Commands) {
    let cluster = SuperSpectre::new_with_anchor(5, Vec2::ZERO, Anchor::Anchor1, Angle::ZERO);
    let mut counter = 0;
    for spectre in cluster.spectres() {
        spawn_tile(&mut commands, spectre);
        counter += 1;
    }
    println!("counter: {}", counter);
}

/// カメラの移動を制御するコンポーネント
///
/// # Details
/// マウスのドラッグ操作によってカメラを移動させる機能を提供します。
/// 左クリックを押下している間、マウスの移動に合わせてカメラが移動します。
#[derive(Component)]
struct CameraController {
    /// カメラの移動速度（ピクセル単位）
    pub speed: f32,
    /// ドラッグ中かどうかのフラグ
    pub dragging: bool,
    /// 前フレームでのマウス座標
    pub last_position: Vec2,
}

impl Default for CameraController {
    fn default() -> Self {
        Self {
            speed: 1.0,                // 1ピクセルのマウス移動で1ピクセルのカメラ移動
            dragging: false,           // 初期状態ではドラッグしていない
            last_position: Vec2::ZERO, // 初期位置は原点
        }
    }
}

/// カメラの移動を制御するシステム
///
/// # Arguments
/// * `windows` - ウィンドウの情報を取得するためのクエリ
/// * `mouse_input` - マウスの入力状態
/// * `query` - カメラのTransformとControllerを取得するクエリ
///
/// # Details
/// マウスの左クリックドラッグでカメラを移動させます：
/// 1. 左クリック押下でドラッグ開始
/// 2. ドラッグ中はマウスの移動量に応じてカメラを移動
/// 3. 左クリック解放でドラッグ終了
fn camera_movement_system(
    windows: Query<&Window, With<PrimaryWindow>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut query: Query<(&mut Transform, &mut CameraController)>,
) {
    // プライマリウィンドウを取得
    let window = windows.single();

    // カメラエンティティを処理
    for (mut transform, mut controller) in query.iter_mut() {
        // 1. ドラッグ開始：左クリック押下時
        if mouse_input.just_pressed(MouseButton::Left) {
            if let Some(pos) = window.cursor_position() {
                controller.dragging = true;
                controller.last_position = pos;
            }
        }

        // 2. ドラッグ終了：左クリック解放時
        if mouse_input.just_released(MouseButton::Left) {
            controller.dragging = false;
        }

        // 3. カメラ移動：ドラッグ中
        if controller.dragging {
            if let Some(pos) = window.cursor_position() {
                // マウスの移動量を計算
                let delta = pos - controller.last_position;

                // カメラの位置を更新
                // X軸：マウスの移動と逆方向に移動
                transform.translation.x -= delta.x * controller.speed;
                // Y軸：マウスの移動と逆方向に移動（座標系の違いを調整）
                transform.translation.y += delta.y * controller.speed;

                // 次のフレームのために現在位置を保存
                controller.last_position = pos;
            }
        }
    }
}
