use bevy::{prelude::*, time::Time, window::PrimaryWindow, input::touch::TouchInput};

pub fn setup_camera(mut commands: Commands) {
    commands
        .spawn((Camera2d, Msaa::Sample4))
        .insert(CameraController::default());
}

/// カメラの移動を制御するコンポーネント
///
/// # Details
/// マウスのドラッグ操作またはタッチ操作によってカメラを移動させる機能を提供します。
/// マウス：左クリックドラッグで移動、ホイールでズーム
/// タッチ：シングルタッチでドラッグ、ピンチジェスチャーでズーム
#[derive(Component)]
pub struct CameraController {
    /// カメラの移動速度（ピクセル単位）
    pub speed: f32,
    /// ドラッグ中かどうかのフラグ
    pub dragging: bool,
    /// 前フレームでのマウス/タッチ座標
    pub last_position: Vec2,
    /// カメラの現在の速度
    pub velocity: Vec2,
    /// 慣性の減衰係数（1フレームあたり）
    pub damping: f32,
    /// ドラッグ中の速度
    pub drag_velocity: Vec2,
    /// カメラのズーム倍率
    pub zoom: f32,
    /// ズームの最小値
    pub min_zoom: f32,
    /// ズームの最大値
    pub max_zoom: f32,
    /// ズームの速度
    pub zoom_speed: f32,
    /// 前フレームでのピンチ距離
    pub last_pinch_distance: Option<f32>,
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
            zoom: 1.0,                // 初期ズーム倍率
            min_zoom: 0.5,            // 最小ズーム倍率（2倍ズームアウト）
            max_zoom: 2.0,           // 最大ズーム倍率（2倍ズームイン）
            zoom_speed: 0.1,          // ズームの速度係数
            last_pinch_distance: None, // 前フレームでのピンチ距離
        }
    }
}

impl CameraController {
    /// ドラッグ開始時の処理
    fn start_drag(&mut self, position: Vec2) {
        self.dragging = true;
        self.last_position = position;
        self.velocity = Vec2::ZERO;
        self.drag_velocity = Vec2::ZERO;
    }

    /// ドラッグ中の処理
    fn update_drag(&mut self, position: Vec2, transform: &mut Transform) {
        let delta = position - self.last_position;
        let zoom_speed_factor = self.speed / self.zoom;
        transform.translation.x -= delta.x * zoom_speed_factor;
        transform.translation.y += delta.y * zoom_speed_factor;
        self.drag_velocity = -Vec2::new(delta.x, -delta.y);
        self.last_position = position;
    }

    /// ドラッグ終了時の処理
    fn end_drag(&mut self) {
        self.dragging = false;
        self.velocity = self.drag_velocity * 60.0 * self.speed / self.zoom;
        self.drag_velocity = Vec2::ZERO;
    }

    /// ズーム処理
    fn update_zoom(&mut self, zoom_delta: f32, zoom_center: Vec2, window: &Window, transform: &mut Transform) {
        let old_zoom = self.zoom;
        self.zoom = (self.zoom + zoom_delta).clamp(self.min_zoom, self.max_zoom);

        // ズーム中心を基準に補正
        let window_size = Vec2::new(window.width(), window.height());
        let window_center = window_size * 0.5;
        let center_offset = zoom_center - window_center;

        // 現在のワールド座標でのズーム中心位置
        let current_world_pos = transform.translation.truncate() + center_offset / old_zoom;
        // 新しいズームでのズーム中心位置
        let new_world_pos = transform.translation.truncate() + center_offset / self.zoom;

        // カメラ位置を補正
        transform.translation += (current_world_pos - new_world_pos).extend(0.0);
        transform.scale = Vec3::splat(1.0 / self.zoom);
    }

    /// 慣性による移動の更新
    fn update_inertia(&mut self, dt: f32, transform: &mut Transform) {
        if !self.dragging && self.velocity.length_squared() > 0.01 {
            transform.translation.x += self.velocity.x * dt;
            transform.translation.y += self.velocity.y * dt;
            self.velocity *= self.damping;
            if self.velocity.length_squared() < 0.01 {
                self.velocity = Vec2::ZERO;
            }
        }
    }
}

/// カメラの移動を制御するシステム
pub fn camera_movement_system(
    windows: Query<&Window, With<PrimaryWindow>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut scroll_evr: EventReader<bevy::input::mouse::MouseWheel>,
    mut touch_evr: EventReader<TouchInput>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut CameraController)>,
) {
    let window = windows.single();
    let dt = time.delta().as_secs_f32();

    for (mut transform, mut controller) in query.iter_mut() {
        let cursor_pos = window.cursor_position();

        // タッチ入力の処理
        let active_touches: Vec<_> = touch_evr.read().collect();
        if !active_touches.is_empty() {
            match active_touches.len() {
                1 => {
                    // シングルタッチ（ドラッグ）
                    let touch = &active_touches[0];
                    match touch.phase {
                        bevy::input::touch::TouchPhase::Started => {
                            controller.start_drag(touch.position);
                        }
                        bevy::input::touch::TouchPhase::Moved => {
                            if controller.dragging {
                                controller.update_drag(touch.position, &mut transform);
                            }
                        }
                        bevy::input::touch::TouchPhase::Ended | bevy::input::touch::TouchPhase::Canceled => {
                            controller.end_drag();
                        }
                    }
                }
                2 => {
                    // ピンチズーム
                    let touch1 = active_touches[0].position;
                    let touch2 = active_touches[1].position;
                    let pinch_center = (touch1 + touch2) * 0.5;
                    let current_distance = touch1.distance(touch2);

                    if let Some(last_dist) = controller.last_pinch_distance {
                        let distance_delta = current_distance - last_dist;
                        let zoom_delta = distance_delta * controller.zoom_speed * 0.01;
                        controller.update_zoom(zoom_delta, pinch_center, window, &mut transform);
                    }
                    controller.last_pinch_distance = Some(current_distance);
                }
                _ => {
                    controller.last_pinch_distance = None;
                }
            }
        } else {
            // マウス入力の処理
            controller.last_pinch_distance = None;

            // ズーム処理（マウスホイール）
            let scroll_amount: f32 = scroll_evr.read().map(|e| e.y).sum();
            if scroll_amount != 0.0 && cursor_pos.is_some() {
                let zoom_delta = scroll_amount * controller.zoom_speed;
                controller.update_zoom(zoom_delta, cursor_pos.unwrap(), window, &mut transform);
            }

            // マウスドラッグ処理
            if mouse_input.just_pressed(MouseButton::Left) {
                if let Some(pos) = cursor_pos {
                    controller.start_drag(pos);
                }
            } else if mouse_input.just_released(MouseButton::Left) {
                controller.end_drag();
            } else if controller.dragging {
                if let Some(pos) = cursor_pos {
                    controller.update_drag(pos, &mut transform);
                }
            }
        }

        // 慣性による移動の更新
        controller.update_inertia(dt, &mut transform);
    }
}
