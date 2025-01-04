use bevy::{prelude::*, window::PrimaryWindow};

pub fn setup_camera(mut commands: Commands) {
    commands
        .spawn((Camera2d, Msaa::Sample4))
        .insert(CameraController::default());
}

/// カメラの移動を制御するコンポーネント
///
/// # Details
/// マウスのドラッグ操作によってカメラを移動させる機能を提供します。
/// 左クリックを押下している間、マウスの移動に合わせてカメラが移動します。
#[derive(Component)]
pub struct CameraController {
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
pub fn camera_movement_system(
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
