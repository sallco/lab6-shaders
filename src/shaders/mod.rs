pub mod gas;
pub mod procedural;
pub mod rocky;
pub mod star;
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
    pub noise: procedural::Noise,
}
impl Uniforms {
    pub fn new(body: Body, camera_position: Vector3) -> Self {
        Self {
            body,
            time: 0.0,
            camera_position,
            noise: procedural::Noise::default(),
            light_direction: Vector3::new(-0.8, 0.25, 0.45).normalized(),
        }
    }
}

pub fn fragment_shader(fragment: &Fragment, uniforms: &Uniforms) -> Vector3 {
    if uniforms.body == Body::Star {
        return star::star_shader(fragment, uniforms);
    }
    if uniforms.body == Body::Rocky {
        return rocky::rocky_shader(fragment, uniforms);
    }
    gas::gas_shader(fragment, uniforms)
}
