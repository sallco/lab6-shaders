use crate::{camera::Camera, shaders::Body};
use raylib::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct State {
    pub body: Body,
    pub time: f32,
    pub paused: bool,
    pub camera: Camera,
}
impl Default for State {
    fn default() -> Self {
        Self {
            body: Body::Star,
            time: 0.0,
            paused: false,
            camera: Camera::default(),
        }
    }
}
impl State {
    pub fn update(
        &mut self,
        dt: f32,
        held: impl Fn(KeyboardKey) -> bool,
        pressed: impl Fn(KeyboardKey) -> bool,
    ) -> bool {
        use KeyboardKey::*;
        let before = *self;
        if pressed(KEY_SPACE) {
            self.paused = !self.paused;
        }
        let mut reset = pressed(KEY_R);
        for (key, body) in [
            (KEY_ONE, Body::Star),
            (KEY_TWO, Body::Rocky),
            (KEY_THREE, Body::Gas),
        ] {
            if pressed(key) {
                self.body = body;
                reset = true;
            }
        }
        if reset {
            self.time = 0.0;
            self.camera = Camera::default();
        } else {
            if !self.paused {
                self.time += dt.max(0.0);
            }
            self.camera.update(dt, held);
        }
        *self != before
    }
}
