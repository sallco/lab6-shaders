use super::{
    Uniforms,
    procedural::{mix, rgb, smooth},
};
use crate::triangle::Fragment;
use raylib::prelude::*;

pub const BAND_FREQUENCY: f32 = 23.0;
pub const TURBULENCE: f32 = 2.4;

pub fn gas_shader(f: &Fragment, u: &Uniforms) -> Vector3 {
    let p = f.position.normalized();
    let q = p;
    let flow = u.noise.warp(q * 4.0, 0.8);
    let turbulence = u.noise.sample(flow) * TURBULENCE;
    let bands = 0.5
        + 0.5 * (q.y * BAND_FREQUENCY + turbulence + 0.3 * (q.x * 11.0 + q.z * 7.0).sin()).sin();
    let mut color = mix(rgb(0x287334), rgb(0x68ab3a), smooth(0.0, 0.4, bands));
    color = mix(color, rgb(0xb3d34a), smooth(0.32, 0.78, bands));
    color = mix(color, rgb(0xf2e59a), smooth(0.76, 1.0, bands));
    let fine = u.noise.fbm(flow * 8.0, 2);
    color *= 0.9 + fine * 0.2;
    let day = f.normal.dot(u.light_direction).max(0.0);
    color * (0.19 + 0.81 * day)
}
