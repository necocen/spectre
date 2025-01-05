use bevy::{prelude::*, time::Time, window::PrimaryWindow};

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
/// ドラッグ終了後は慣性によって移動が続きます。
#[derive(Component)]
pub struct CameraController {
    /// カメラの移動速度（ピクセル単位）
    pub speed: f32,
    /// ドラッグ中かどうかのフラグ
    pub dragging: bool,
    /// 前フレームでのマウス座標
    pub last_position: Vec2,
    /// カメラの現在の速度
    pub velocity: Vec2,
    /// 慣性の減衰係数（1フレームあたり）
    pub damping: f32,
    /// ドラッグ中の速度
    pub drag_velocity: Vec2,
}

impl Default for CameraController {
    fn default() -> Self {
        Self {
            speed: 1.0,                // 1ピクセルのマウス移動で1ピクセルのカメラ移動
            dragging: false,           // 初期状態ではドラッグしていない
            last_position: Vec2::ZERO, // 初期位置は原点
            velocity: Vec2::ZERO,      // 初期速度はゼロ
            damping: 0.95,            // 1フレームあたり5%の減速
            drag_velocity: Vec2::ZERO, // ドラッグ中の速度
        }
    }
}

/// カメラの移動を制御するシステム
///
/// # Arguments
/// * `windows` - ウィンドウの情報を取得するためのクエリ
/// * `mouse_input` - マウスの入力状態
/// * `time` - 時間情報
/// * `query` - カメラのTransformとControllerを取得するクエリ
///
/// # Details
/// マウスの左クリックドラッグでカメラを移動させます：
/// 1. 左クリック押下でドラッグ開始
/// 2. ドラッグ中はマウスの移動量に応じてカメラを移動
/// 3. 左クリック解放でドラッグ終了
/// 4. ドラッグ終了後は慣性により移動が続く
pub fn camera_movement_system(
    windows: Query<&Window, With<PrimaryWindow>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut CameraController)>,
) {
    // プライマリウィンドウを取得
    let window = windows.single();
    let dt = time.delta().as_secs_f32();

    // カメラエンティティを処理
    for (mut transform, controller) in query.iter_mut() {
        let cursor_pos = window.cursor_position();
        let CameraController {
            speed,
            dragging,
            last_position,
            velocity,
            damping,
            drag_velocity,
        } = *controller;

        let (new_dragging, new_last_pos, new_velocity, new_drag_velocity) =
            if mouse_input.just_pressed(MouseButton::Left) {
                // ドラッグ開始
                if let Some(pos) = cursor_pos {
                    (true, pos, Vec2::ZERO, Vec2::ZERO)
                } else {
                    (dragging, last_position, velocity, drag_velocity)
                }
            } else if mouse_input.just_released(MouseButton::Left) {
                // ドラッグ終了：現在のドラッグ速度を慣性速度として使用
                // フレームレート（60fps）で正規化して適切な速度にする
                (false, last_position, drag_velocity * 60.0 * speed, Vec2::ZERO)
            } else if dragging {
                // ドラッグ中：現在の移動速度を計算して保存
                if let Some(pos) = cursor_pos {
                    let delta = pos - last_position;
                    let current_velocity = -Vec2::new(delta.x, -delta.y);
                    transform.translation.x -= delta.x * speed;
                    transform.translation.y += delta.y * speed;
                    // 現在のフレームの速度を保存
                    (dragging, pos, velocity, current_velocity)
                } else {
                    (dragging, last_position, velocity, drag_velocity)
                }
            } else if velocity.length_squared() > 0.01 {
                // 慣性による移動
                transform.translation.x += velocity.x * dt;
                transform.translation.y += velocity.y * dt;
                let mut new_vel = velocity * damping;
                if new_vel.length_squared() < 0.01 {
                    new_vel = Vec2::ZERO;
                }
                (dragging, last_position, new_vel, Vec2::ZERO)
            } else {
                (dragging, last_position, velocity, Vec2::ZERO)
            };

        // 状態の一括更新
        let mut controller = controller;
        controller.dragging = new_dragging;
        controller.last_position = new_last_pos;
        controller.velocity = new_velocity;
        controller.drag_velocity = new_drag_velocity;
    }
}
