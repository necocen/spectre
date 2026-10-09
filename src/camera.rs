use mikage::Camera2d;

pub(crate) fn new() -> Camera2d {
    let mut camera = Camera2d::default();
    camera.zoom = 0.028;
    camera.damping = 0.95;
    camera.min_zoom = 0.003;
    camera.max_zoom = 0.12;
    camera.zoom_speed = 0.2;
    camera.zoom_smoothing = 0.2;
    camera
}

#[cfg(test)]
mod tests {
    use super::*;
    use mikage::InteractiveCamera;

    const FRAME_DT: f32 = 1.0 / 60.0;

    fn settle(camera: &mut Camera2d, dt: f32) -> usize {
        for frames in 1..=(10.0 / dt).ceil() as usize {
            camera.update(dt);
            if !camera.needs_redraw() {
                return frames;
            }
        }
        panic!("camera animation did not settle");
    }

    #[test]
    fn idle_camera_does_not_request_more_frames() {
        let mut camera = new();
        for dt in [FRAME_DT, 0.25, 60.0] {
            camera.update(dt);
            assert!(!camera.needs_redraw());
        }
    }

    #[test]
    fn zoom_finishes_and_restarts_at_high_refresh_rates() {
        for fps in [60.0, 360.0, 1000.0] {
            let mut camera = new();
            let initial_zoom = camera.zoom;
            camera.on_scroll(1.0);
            assert!(camera.needs_redraw());
            camera.update(FRAME_DT);

            let target = initial_zoom * 1.2;
            let first_zoom = initial_zoom + (target - initial_zoom) * 0.2;
            assert!((camera.zoom - first_zoom).abs() < 1e-7);
            assert!(settle(&mut camera, 1.0 / fps) > 1);
            assert!((camera.zoom - target).abs() < 1e-7);

            camera.update(60.0);
            assert!(!camera.needs_redraw());
            camera.on_scroll(-1.0);
            assert!(camera.needs_redraw());
            camera.update(FRAME_DT);
            assert!((camera.zoom - target * 0.96).abs() < 1e-7);
            settle(&mut camera, 1.0 / fps);
            assert!((camera.zoom - target * 0.8).abs() < 1e-7);
        }
    }

    #[test]
    fn drag_release_keeps_inertia_running_until_it_settles() {
        let mut camera = new();
        camera.on_mouse_drag(100.0, -30.0, true, false, false);
        let dragged_position = camera.position;
        camera.update(FRAME_DT);
        assert_eq!(camera.position, dragged_position);
        assert!(!camera.needs_redraw());

        camera.on_drag_end();
        assert!(camera.needs_redraw());
        camera.update(FRAME_DT);
        assert_ne!(camera.position, dragged_position);
        assert!(settle(&mut camera, FRAME_DT) > 1);

        let settled_position = camera.position;
        camera.update(60.0);
        assert_eq!(camera.position, settled_position);
        assert!(!camera.needs_redraw());
    }

    #[test]
    fn camera_motion_is_independent_of_frame_rate() {
        let mut at_60_hz = new();
        let mut at_120_hz = new();
        for camera in [&mut at_60_hz, &mut at_120_hz] {
            camera.on_mouse_drag(100.0, -30.0, true, false, false);
            camera.on_drag_end();
            camera.on_scroll(1.0);
        }
        for _ in 0..60 {
            at_60_hz.update(1.0 / 60.0);
        }
        for _ in 0..120 {
            at_120_hz.update(1.0 / 120.0);
        }
        assert!((at_60_hz.zoom - at_120_hz.zoom).abs() < 1e-7);
        assert!(at_60_hz.position.distance(at_120_hz.position) < 1e-3);
    }
}
