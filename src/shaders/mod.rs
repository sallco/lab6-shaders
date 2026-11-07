use crate::triangle::Fragment;
use raylib::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Body {
    Star,
    Rocky,
    Gas,
}
impl Body {
    pub fn name(self) -> &'static str {
        match self {
            Self::Star => "Cérula",
            Self::Rocky => "Obsidia",
            Self::Gas => "Artaud",
        }
    }
}

pub struct Uniforms {
    pub body: Body,
    pub time: f32,
    pub camera_position: Vector3,
    pub light_direction: Vector3,
}
impl Uniforms {
    pub fn new(body: Body, camera_position: Vector3) -> Self {
        Self {
            body,
            time: 0.0,
            camera_position,
            light_direction: Vector3::new(-0.8, 0.25, 0.45).normalized(),
        }
    }
}

pub fn fragment_shader(fragment: &Fragment, uniforms: &Uniforms) -> Vector3 {
    Vector3::new(0.45, 0.62, 0.8)
        * (0.12 + 0.88 * fragment.normal.dot(uniforms.light_direction).max(0.0))
}
