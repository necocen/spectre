use bevy::{prelude::*, time::Time, window::PrimaryWindow, input::touch::TouchInput};

pub fn setup_camera(mut commands: Commands) {
    commands
        .spawn((Camera2d, Msaa::Sample4))
        .insert(CameraController::default());
}

/// タッチ操作の状態を表す列挙型
#[derive(Debug, Clone, Copy, PartialEq)]
enum TouchState {
    /// タッチなし
    None,
    /// シングルタッチでのドラッグ
    Dragging {
        /// タッチID
        id: u64,
    },
    /// ピンチズーム中
    Pinching {
        /// タッチID
        id1: u64,
        id2: u64,
        /// 前回のピンチ距離
        last_distance: f32,
    },
}

/// カメラの移動を制御するコンポーネント
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
    /// タッチの状態
    touch_state: TouchState,
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
            touch_state: TouchState::None,
        }
    }
}

impl CameraController {
    /// ドラッグ開始時の処理
    fn start_drag(&mut self, position: Vec2, id: Option<u64>) {
        self.dragging = true;
        self.last_position = position;
        self.velocity = Vec2::ZERO;
        self.drag_velocity = Vec2::ZERO;
        if let Some(id) = id {
            self.touch_state = TouchState::Dragging { id };
        }
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
        if matches!(self.touch_state, TouchState::Dragging { .. }) {
            self.touch_state = TouchState::None;
        }
    }

    /// スクリーン座標をワールド座標に変換
    fn screen_to_world(&self, screen_pos: Vec2, window_size: Vec2, transform: &Transform) -> Vec2 {
        let screen_center = window_size * 0.5;
        let screen_offset = screen_pos - screen_center;
        transform.translation.truncate() + screen_offset / self.zoom
    }

    /// ワールド座標をスクリーン座標に変換
    fn world_to_screen(&self, world_pos: Vec2, window_size: Vec2, transform: &Transform) -> Vec2 {
        let screen_center = window_size * 0.5;
        let world_offset = world_pos - transform.translation.truncate();
        screen_center + world_offset * self.zoom
    }

    /// ズーム処理
    fn update_zoom(&mut self, zoom_delta: f32, cursor_pos: Vec2, window: &Window, transform: &mut Transform) {
        let old_zoom = self.zoom;

        // ズーム速度を調整（マウスホイールの場合は0.1倍）
        let adjusted_delta = if matches!(self.touch_state, TouchState::Pinching { .. }) {
            zoom_delta
        } else {
            zoom_delta * 0.1
        };

        let new_zoom = (old_zoom * (1.0 + adjusted_delta)).clamp(self.min_zoom, self.max_zoom);

        if (new_zoom - old_zoom).abs() > f32::EPSILON {
            let window_size = Vec2::new(window.width(), window.height());

            // ズーム前のワールド座標を計算
            let world_pos = self.screen_to_world(cursor_pos, window_size, transform);

            // ズームを適用
            self.zoom = new_zoom;
            transform.scale = Vec3::splat(1.0 / new_zoom);

            // ズーム後のスクリーン座標を計算して位置を補正
            let new_screen_pos = self.world_to_screen(world_pos, window_size, transform);
            let screen_delta = new_screen_pos - cursor_pos;
            // スクリーン座標の差分をワールド座標に変換（Y座標は反転）
            let world_delta = Vec2::new(screen_delta.x, -screen_delta.y) / new_zoom;
            transform.translation += world_delta.extend(0.0);
        }
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

    /// ピンチズーム開始
    fn start_pinch(&mut self, touch1: Vec2, touch2: Vec2, id1: u64, id2: u64) {
        let distance = touch1.distance(touch2);
        self.touch_state = TouchState::Pinching {
            id1,
            id2,
            last_distance: distance,
        };
        self.dragging = false;
        self.velocity = Vec2::ZERO;
        self.drag_velocity = Vec2::ZERO;
    }

    /// ピンチズーム更新
    fn update_pinch(&mut self, touch1: Vec2, touch2: Vec2, window: &Window, transform: &mut Transform) {
        let current_distance = touch1.distance(touch2);
        let center = (touch1 + touch2) * 0.5;

        if let TouchState::Pinching { last_distance, .. } = self.touch_state {
            let distance_delta = (current_distance - last_distance) / last_distance;
            let zoom_delta = distance_delta * self.zoom_speed;
            self.update_zoom(zoom_delta, center, window, transform);

            self.touch_state = match self.touch_state {
                TouchState::Pinching { id1, id2, .. } => TouchState::Pinching {
                    id1,
                    id2,
                    last_distance: current_distance,
                },
                _ => unreachable!(),
            };
        }
    }

    /// ピンチズーム終了
    fn end_pinch(&mut self) {
        self.touch_state = TouchState::None;
        self.dragging = false;
        self.velocity = Vec2::ZERO;
        self.drag_velocity = Vec2::ZERO;
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
                            if matches!(controller.touch_state, TouchState::None) {
                                controller.start_drag(touch.position, Some(touch.id));
                            }
                        }
                        bevy::input::touch::TouchPhase::Moved => {
                            if let TouchState::Dragging { id } = controller.touch_state {
                                if id == touch.id {
                                    controller.update_drag(touch.position, &mut transform);
                                }
                            }
                        }
                        bevy::input::touch::TouchPhase::Ended | bevy::input::touch::TouchPhase::Canceled => {
                            if let TouchState::Dragging { id } = controller.touch_state {
                                if id == touch.id {
                                    controller.end_drag();
                                }
                            }
                        }
                    }
                }
                2 => {
                    // ピンチズーム
                    let touch1 = &active_touches[0];
                    let touch2 = &active_touches[1];

                    match controller.touch_state {
                        TouchState::None | TouchState::Dragging { .. } => {
                            controller.start_pinch(touch1.position, touch2.position, touch1.id, touch2.id);
                        }
                        TouchState::Pinching { id1, id2, .. } => {
                            if (id1 == touch1.id && id2 == touch2.id) || (id1 == touch2.id && id2 == touch1.id) {
                                controller.update_pinch(touch1.position, touch2.position, window, &mut transform);
                            }
                        }
                    }
                }
                _ => {
                    controller.end_pinch();
                }
            }
        } else {
            // タッチ入力がない場合は状態をリセット
            if !matches!(controller.touch_state, TouchState::None) {
                controller.touch_state = TouchState::None;
            }

            // マウス入力の処理
            // ズーム処理（マウスホイール）
            let scroll_amount: f32 = scroll_evr.read().map(|e| e.y).sum();
            if scroll_amount != 0.0 && cursor_pos.is_some() {
                let zoom_delta = scroll_amount * controller.zoom_speed;
                controller.update_zoom(zoom_delta, cursor_pos.unwrap(), window, &mut transform);
            }

            // マウスドラッグ処理
            if mouse_input.just_pressed(MouseButton::Left) {
                if let Some(pos) = cursor_pos {
                    controller.start_drag(pos, None);
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
