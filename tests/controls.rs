use lab6_shaders::{camera::Camera, shaders::Body, state::State};
use raylib::prelude::*;
#[test]
fn selection_pause_reset_and_camera_work_independently() {
    use KeyboardKey::*;
    let mut state = State::default();
    assert!(state.update(0.05, |_| false, |_| false));
    assert!(state.time > 0.0);
    state.update(0.05, |_| false, |key| key == KEY_TWO);
    assert_eq!(state.body, Body::Rocky);
    assert_eq!(state.time, 0.0);
    state.update(0.05, |_| false, |key| key == KEY_SPACE);
    let time = state.time;
    state.update(0.05, |key| key == KEY_RIGHT, |_| false);
    assert_eq!(state.time, time);
    assert_ne!(state.camera, Camera::default());
    state.update(0.05, |_| false, |key| key == KEY_R);
    assert_eq!(state.camera, Camera::default());
    assert_eq!(state.time, 0.0);
    assert!(state.paused);
    state.update(0.05, |_| false, |key| key == KEY_THREE);
    assert_eq!(state.body, Body::Gas);
}
#[test]
fn orbit_and_zoom_stay_outside_the_sphere() {
    use KeyboardKey::*;
    let mut camera = Camera::default();
    for _ in 0..200 {
        camera.update(0.1, |key| key == KEY_UP || key == KEY_EQUAL);
    }
    assert_eq!(camera.distance, 1.7);
    assert_eq!(camera.pitch, 1.3);
    for _ in 0..200 {
        camera.update(0.1, |key| key == KEY_DOWN || key == KEY_MINUS);
    }
    assert_eq!(camera.distance, 5.5);
    assert_eq!(camera.pitch, -1.3);
}
