use anchor::Anchor;
use angle::Angle;
use bevy::{prelude::*, window::PrimaryWindow};
use bevy_prototype_lyon::prelude::*;
use spectre::Spectre;
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
/// - 彩度（S）：80%で固定
/// - 明度（V）：80%で固定
fn spawn_tile(commands: &mut Commands, spectre: &Spectre) {
    // タイルの頂点から多角形を生成
    let polygon = shapes::Polygon {
        points: spectre.points.to_vec(),
        closed: true,
    };

    // angleから色相を計算（30度ごと）
    let hue = spectre.angle.value() as f32 * 30.0;
    let saturation =
        (spectre.anchor(Anchor::Anchor1).length() / spectre.size * 2.0).sin() * 0.25 + 0.75;
    println!(
        "angle: {}, length: {}, hue: {}, saturation: {}",
        spectre.angle.value(),
        spectre.anchor(Anchor::Anchor1).length(),
        hue,
        saturation
    );
    // HSVからRGBに変換（彩度80%、明度80%）
    let color = Color::hsl(hue, saturation, 0.8);

    // タイルのエンティティを生成
    commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&polygon),
            ..default()
        },
        Fill::color(color),
    ));
}

/// タイルを生成して配置する
///
/// # Arguments
/// * `commands` - Bevyのコマンドバッファ
/// * `tiles` - 生成済みのタイル配列
/// * `connections` - タイルの接続情報：(接続元タイルのインデックス, 接続元アンカー, 接続先アンカー)
/// * `start_index` - 生成するタイルの開始インデックス（色分け用）
///
/// # Details
/// 指定された接続情報に基づいて新しいタイルを生成し、既存のタイルに接続します。
/// 生成したタイルは`tiles`に追加され、同時に描画用のエンティティも生成されます。
/// `start_index`は色分けのために使用され、タイルの階層を視覚的に表現します。
fn place_connected_tiles(
    commands: &mut Commands,
    tiles: &mut Vec<Spectre>,
    connections: &[(usize, Anchor, Anchor)],
) {
    for (from_idx, from_anchor, to_anchor) in connections {
        let tile = tiles[*from_idx].adjacent_spectre(*from_anchor, *to_anchor);
        let tile_clone = tile.clone();
        tiles.push(tile);
        spawn_tile(commands, &tile_clone);
    }
}

/// タイルを配置するシステム
///
/// # Details
/// タイルを以下の手順で配置します：
/// 1. 中心タイル：原点に配置（角度0度）
/// 2. 1段目のタイル：中心タイルから4方向に接続
/// 3. 2段目のタイル：1段目のタイルから外側に接続
/// 4. 中心タイルを最後に再描画して最前面に表示
fn setup_tiles(mut commands: Commands) {
    // タイルのサイズを設定
    const TILE_SIZE: f32 = 40.0;

    // 中心となるタイルを生成（原点に配置、角度0度）
    let center =
        Spectre::new_with_anchor(Vec2::new(0.0, 0.0), Anchor::Anchor1, TILE_SIZE, Angle::ZERO)
            .to_mystic_like();

    let center_spectres = center.spectres();
    for spectre in center_spectres {
        spawn_tile(&mut commands, spectre);
    }
    let mut tiles = vec![center.spectres()[0].clone()];

    // 1段目：中心タイルから4方向に接続
    let inner_connections = [
        // (接続元インデックス, 接続元アンカー, 接続先アンカー)
        (0, Anchor::Anchor1, Anchor::Anchor1), // A
        (1, Anchor::Anchor3, Anchor::Anchor1), // F
        (2, Anchor::Anchor4, Anchor::Anchor2), // B
        (3, Anchor::Anchor3, Anchor::Anchor1), // C
        (4, Anchor::Anchor3, Anchor::Anchor1), // G
        (5, Anchor::Anchor4, Anchor::Anchor2), // D
        (6, Anchor::Anchor3, Anchor::Anchor1), // E
    ];
    place_connected_tiles(&mut commands, &mut tiles, &inner_connections);
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
