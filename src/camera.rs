use bevy::{input::touch::TouchInput, prelude::*, time::Time, window::PrimaryWindow};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera)
            .add_systems(Update, camera_movement_system);
    }
}

fn setup_camera(mut commands: Commands) {
    let controller = CameraController::default();
    commands
        .spawn((
            Camera2d,
            Msaa::Sample4,
            Transform::from_scale(Vec3::splat(1.0 / controller.zoom)),
        ))
        .insert(controller);
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
        /// 前回のタッチ位置
        last_position: Vec2,
    },
    /// ピンチズーム中
    Pinching {
        /// タッチID
        id1: u64,
        id2: u64,
        /// ピンチ開始時の距離
        initial_distance: f32,
        /// ピンチ開始時のズーム倍率
        initial_zoom: f32,
    },
}

/// カメラの移動を制御するコンポーネント
#[derive(Component)]
struct CameraController {
    /// カメラの移動速度（ピクセル単位）
    pub speed: f32,
    /// ドラッグ中かどうかのフラグ
    pub dragging: bool,
    /// 前フレームでのマウス位置
    pub last_mouse_position: Option<Vec2>,
    /// カメラの現在の速度
    pub velocity: Vec2,
    /// 慣性の減衰係数（1フレームあたり）
    pub damping: f32,
    /// ドラッグ中の速度
    pub drag_velocity: Vec2,
    /// カメラの現在のズーム倍率
    pub zoom: f32,
    /// 目標のズーム倍率
    pub target_zoom: f32,
    /// ズームの最小値
    pub min_zoom: f32,
    /// ズームの最大値
    pub max_zoom: f32,
    /// ズームの速度
    pub zoom_speed: f32,
    /// ズームのスムージング係数
    pub zoom_smoothing: f32,
    /// タッチの状態
    touch_state: TouchState,
}

impl Default for CameraController {
    fn default() -> Self {
        Self {
            speed: 1.0,
            dragging: false,
            last_mouse_position: None,
            velocity: Vec2::ZERO,
            damping: 0.95,
            drag_velocity: Vec2::ZERO,
            zoom: 10.0,
            target_zoom: 10.0,
            min_zoom: 2.5,
            max_zoom: 40.0,
            zoom_speed: 0.2,
            zoom_smoothing: 0.3,
            touch_state: TouchState::None,
        }
    }
}

impl CameraController {
    /// ドラッグ開始時の処理
    fn start_drag(&mut self, position: Vec2, id: Option<u64>) {
        self.dragging = true;
        self.velocity = Vec2::ZERO;
        self.drag_velocity = Vec2::ZERO;
        if let Some(id) = id {
            self.touch_state = TouchState::Dragging {
                id,
                last_position: position,
            };
        } else {
            self.last_mouse_position = Some(position);
        }
    }

    /// ドラッグ中の処理
    fn update_drag(&mut self, position: Vec2, last_position: Vec2, transform: &mut Transform) {
        let delta = position - last_position;
        let zoom_speed_factor = self.speed / self.zoom;
        transform.translation.x -= delta.x * zoom_speed_factor;
        transform.translation.y += delta.y * zoom_speed_factor;
        self.drag_velocity = -Vec2::new(delta.x, -delta.y);
    }

    /// ドラッグ終了時の処理
    fn end_drag(&mut self) {
        self.dragging = false;
        self.velocity = self.drag_velocity * 60.0 * self.speed / self.zoom;
        self.drag_velocity = Vec2::ZERO;
        self.touch_state = TouchState::None;
        self.last_mouse_position = None;
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
    fn update_zoom(
        &mut self,
        new_target_zoom: f32,
        cursor_pos: Vec2,
        window: &Window,
        transform: &mut Transform,
    ) {
        let old_zoom = self.zoom;
        self.target_zoom = new_target_zoom.clamp(self.min_zoom, self.max_zoom);

        // 現在のズーム値を目標値に向けて補間
        self.zoom = self.zoom + (self.target_zoom - self.zoom) * self.zoom_smoothing;

        if (self.zoom - old_zoom).abs() > f32::EPSILON {
            let window_size = Vec2::new(window.width(), window.height());
            let world_pos = self.screen_to_world(cursor_pos, window_size, transform);

            transform.scale = Vec3::splat(1.0 / self.zoom);

            let new_screen_pos = self.world_to_screen(world_pos, window_size, transform);
            let screen_delta = cursor_pos - new_screen_pos;
            let world_delta = Vec2::new(screen_delta.x, -screen_delta.y) / self.zoom;
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
            initial_distance: distance,
            initial_zoom: self.zoom,
        };
        self.dragging = false;
        self.velocity = Vec2::ZERO;
        self.drag_velocity = Vec2::ZERO;
    }

    /// ピンチズーム更新
    fn update_pinch(
        &mut self,
        touch1: Vec2,
        touch2: Vec2,
        window: &Window,
        transform: &mut Transform,
    ) {
        let current_distance = touch1.distance(touch2);
        let center = (touch1 + touch2) * 0.5;

        if let TouchState::Pinching {
            initial_distance,
            initial_zoom,
            ..
        } = self.touch_state
        {
            // 初期距離との比率から目標のズーム倍率を計算
            let target_zoom = initial_zoom * (current_distance / initial_distance);
            self.update_zoom(target_zoom, center, window, transform);
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
fn camera_movement_system(
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
        let mut active_touches: Vec<_> = touch_evr.read().collect();
        active_touches.sort_by_key(|touch| touch.id);

        if active_touches.is_empty() {
            match controller.touch_state {
                TouchState::Dragging { .. } => controller.end_drag(),
                TouchState::Pinching { .. } => controller.end_pinch(),
                TouchState::None => {}
            }
        } else {
            match active_touches.len() {
                1 => {
                    let touch = &active_touches[0];
                    match touch.phase {
                        bevy::input::touch::TouchPhase::Started => {
                            controller.start_drag(touch.position, Some(touch.id));
                        }
                        bevy::input::touch::TouchPhase::Moved => {
                            if let TouchState::Dragging { id, last_position } =
                                controller.touch_state
                            {
                                if id == touch.id {
                                    controller.update_drag(
                                        touch.position,
                                        last_position,
                                        &mut transform,
                                    );
                                    controller.touch_state = TouchState::Dragging {
                                        id,
                                        last_position: touch.position,
                                    };
                                }
                            } else {
                                controller.start_drag(touch.position, Some(touch.id));
                            }
                        }
                        bevy::input::touch::TouchPhase::Ended
                        | bevy::input::touch::TouchPhase::Canceled => {
                            if let TouchState::Dragging { id, .. } = controller.touch_state {
                                if id == touch.id {
                                    controller.end_drag();
                                }
                            }
                        }
                    }
                }
                2 => {
                    let touch1 = &active_touches[0];
                    let touch2 = &active_touches[1];

                    let both_moving = touch1.phase == bevy::input::touch::TouchPhase::Moved
                        && touch2.phase == bevy::input::touch::TouchPhase::Moved;

                    match controller.touch_state {
                        TouchState::None | TouchState::Dragging { .. } if both_moving => {
                            controller.start_pinch(
                                touch1.position,
                                touch2.position,
                                touch1.id,
                                touch2.id,
                            );
                        }
                        TouchState::Pinching { id1, id2, .. } if both_moving => {
                            if (id1 == touch1.id && id2 == touch2.id)
                                || (id1 == touch2.id && id2 == touch1.id)
                            {
                                controller.update_pinch(
                                    touch1.position,
                                    touch2.position,
                                    window,
                                    &mut transform,
                                );
                            }
                        }
                        _ => {}
                    }
                }
                _ => {
                    controller.end_pinch();
                }
            }
        }

        if active_touches.is_empty() {
            let scroll_amount: f32 = scroll_evr.read().map(|e| e.y).sum();
            if scroll_amount != 0.0 && cursor_pos.is_some() {
                let zoom_delta = scroll_amount * controller.zoom_speed;
                let new_zoom = controller.zoom * (1.0 + zoom_delta * 0.1);
                controller.update_zoom(new_zoom, cursor_pos.unwrap(), window, &mut transform);
            }

            // マウス入力の処理
            if mouse_input.just_pressed(MouseButton::Left) {
                if let Some(pos) = cursor_pos {
                    controller.start_drag(pos, None);
                }
            } else if mouse_input.just_released(MouseButton::Left) {
                controller.end_drag();
            } else if controller.dragging {
                if let Some(pos) = cursor_pos {
                    if let Some(last_pos) = controller.last_mouse_position {
                        controller.update_drag(pos, last_pos, &mut transform);
                    }
                    controller.last_mouse_position = cursor_pos;
                }
            }
        }

        controller.update_inertia(dt, &mut transform);
    }
}
