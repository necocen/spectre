use std::cell::Cell;
use std::rc::Rc;

use glam::{Mat4, Vec2, Vec3};
use mikage::{Camera, Camera2d, InteractiveCamera};

/// Bridges camera animation to reactive redraws until Mikage supports it directly.
/// See https://github.com/necocen/mikage/issues/2.
pub(crate) struct SpectreCamera {
    inner: Camera2d,
    animating: Rc<Cell<bool>>,
}

impl SpectreCamera {
    pub(crate) fn new(animating: Rc<Cell<bool>>) -> Self {
        let mut inner = Camera2d::default();
        inner.zoom = 0.028;
        inner.damping = 0.95;
        inner.min_zoom = 0.003;
        inner.max_zoom = 0.12;
        inner.zoom_speed = 0.2;
        inner.zoom_smoothing = 0.2;
        Self { inner, animating }
    }

    pub(crate) fn viewport_bounds(&self, aspect: f32) -> (Vec2, Vec2) {
        self.inner.viewport_bounds(aspect)
    }
}

impl Camera for SpectreCamera {
    fn view_matrix(&self) -> Mat4 {
        self.inner.view_matrix()
    }

    fn projection_matrix(&self, aspect: f32) -> Mat4 {
        self.inner.projection_matrix(aspect)
    }

    fn position(&self) -> Vec3 {
        self.inner.position()
    }
}

impl InteractiveCamera for SpectreCamera {
    fn on_mouse_drag(&mut self, dx: f64, dy: f64, left: bool, right: bool, middle: bool) {
        self.inner.on_mouse_drag(dx, dy, left, right, middle);
    }

    fn on_scroll(&mut self, delta: f32) {
        self.inner.on_scroll(delta);
    }

    fn update(&mut self, dt: f32) {
        // The runner's dt includes the preceding idle interval. Resume with one
        // reference frame, then use the real elapsed time while animating.
        let dt = if self.animating.get() { dt } else { 1.0 / 60.0 };
        let before = (self.inner.position, self.inner.zoom);
        self.inner.update(dt);
        self.animating
            .set(before != (self.inner.position, self.inner.zoom));
    }

    fn on_drag_end(&mut self) {
        self.inner.on_drag_end();
    }

    fn on_touch_drag(&mut self, dx: f64, dy: f64) {
        self.inner.on_touch_drag(dx, dy);
    }

    fn on_touch_drag_end(&mut self) {
        self.inner.on_touch_drag_end();
    }

    fn on_pinch_pan(
        &mut self,
        zoom_delta: f32,
        pan_dx: f64,
        pan_dy: f64,
        focus: Option<(f64, f64)>,
    ) {
        self.inner.on_pinch_pan(zoom_delta, pan_dx, pan_dy, focus);
    }

    fn set_viewport_size(&mut self, width: u32, height: u32) {
        self.inner.set_viewport_size(width, height);
    }

    fn set_cursor_position(&mut self, x: f64, y: f64) {
        self.inner.set_cursor_position(x, y);
    }

    fn set_enabled(&mut self, enabled: bool) {
        self.inner.set_enabled(enabled);
    }

    fn is_enabled(&self) -> bool {
        self.inner.is_enabled()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const FRAME_DT: f32 = 1.0 / 60.0;

    fn new_camera() -> (SpectreCamera, Rc<Cell<bool>>) {
        let animating = Rc::new(Cell::new(false));
        (SpectreCamera::new(animating.clone()), animating)
    }

    fn settle(camera: &mut SpectreCamera, animating: &Cell<bool>) -> usize {
        for frames in 1..=600 {
            camera.update(FRAME_DT);
            if !animating.get() {
                return frames;
            }
        }
        panic!("camera animation did not settle");
    }

    #[test]
    fn idle_camera_does_not_request_more_frames() {
        let (mut camera, animating) = new_camera();
        for dt in [FRAME_DT, 0.25, 60.0] {
            camera.update(dt);
            assert!(!animating.get());
        }
    }

    #[test]
    fn zoom_animates_to_target_and_can_restart_after_idle() {
        let (mut camera, animating) = new_camera();
        let initial_zoom = camera.inner.zoom;
        camera.on_scroll(1.0);
        camera.update(60.0);

        // Only one frame of the new animation elapses, even after a long idle.
        let target = initial_zoom * 1.2;
        let first_zoom = initial_zoom + (target - initial_zoom) * 0.2;
        assert!((camera.inner.zoom - first_zoom).abs() < 1e-7);
        assert!(animating.get());
        assert!(settle(&mut camera, &animating) > 1);
        assert!((camera.inner.zoom - target).abs() < 1e-7);

        camera.on_scroll(-1.0);
        camera.update(60.0);
        assert!((camera.inner.zoom - target * 0.96).abs() < 1e-7);
        assert!(animating.get());
        settle(&mut camera, &animating);
        assert!((camera.inner.zoom - target * 0.8).abs() < 1e-7);
    }

    #[test]
    fn drag_release_keeps_inertia_running_until_it_settles() {
        let (mut camera, animating) = new_camera();
        camera.on_mouse_drag(100.0, -30.0, true, false, false);
        let dragged_position = camera.inner.position;
        camera.update(FRAME_DT);
        assert_eq!(camera.inner.position, dragged_position);
        assert!(!animating.get());

        camera.on_drag_end();
        camera.update(60.0);
        assert_ne!(camera.inner.position, dragged_position);
        assert!(animating.get());
        assert!(settle(&mut camera, &animating) > 1);

        let settled_position = camera.inner.position;
        camera.update(60.0);
        assert_eq!(camera.inner.position, settled_position);
        assert!(!animating.get());
    }

    #[test]
    fn active_animation_uses_elapsed_frame_time() {
        let (mut camera, _) = new_camera();
        let (mut reference, _) = new_camera();
        camera.on_scroll(1.0);
        reference.on_scroll(1.0);
        camera.update(FRAME_DT);
        reference.update(FRAME_DT);

        camera.update(1.0 / 30.0);
        reference.inner.update(1.0 / 30.0);
        assert_eq!(camera.inner.zoom, reference.inner.zoom);
        assert_eq!(camera.inner.position, reference.inner.position);
    }
}
