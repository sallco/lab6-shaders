use super::{
    Uniforms,
    procedural::{mix, rgb, smooth},
};
use crate::triangle::Fragment;
use raylib::prelude::*;

pub const VORTEX_RADIUS: f32 = 0.49;
pub const VORTEX_STRENGTH: f32 = 7.0;
pub const VORTEX_CENTER: Vector3 = Vector3 {
    x: 0.35,
    y: -0.19,
    z: 0.917,
};

pub const BAND_FREQUENCY: f32 = 23.0;
pub const TURBULENCE: f32 = 3.0;
pub const WIND_SPEED: f32 = 0.38;
pub const WAVE_SPEED: f32 = 1.4;

pub fn gas_shader(f: &Fragment, u: &Uniforms) -> Vector3 {
    let p = f.position.normalized();
    let q = vortex(p);
    let (s, c) = (u.time * WIND_SPEED * (1.0 + 0.55 * (q.y * 7.0).sin())).sin_cos();
    let wind = Vector3::new(q.x * c + q.z * s, q.y, -q.x * s + q.z * c);
    let flow = u.noise.warp(wind * 4.0, 0.8);
    let latitude = q.y
        + 0.055 * (q.x * 7.0 + q.z * 4.0 + u.time * WAVE_SPEED).sin()
        + 0.025 * (q.z * 11.0 - u.time * 0.9).sin();
    let turbulence = u.noise.sample(flow) * TURBULENCE;
    let bands = 0.5
        + 0.5
            * (latitude * BAND_FREQUENCY + turbulence + 0.3 * (q.x * 11.0 + q.z * 7.0).sin()).sin();
    let mut color = mix(rgb(0x287334), rgb(0x68ab3a), smooth(0.0, 0.4, bands));
    color = mix(color, rgb(0xb3d34a), smooth(0.32, 0.78, bands));
    color = mix(color, rgb(0xf2e59a), smooth(0.76, 1.0, bands));
    let fine = u.noise.fbm(flow * 8.0, 2);
    color *= 0.9 + fine * 0.2;
    let day = f.normal.dot(u.light_direction).max(0.0);
    color * (0.19 + 0.81 * day)
}

pub fn vortex(p: Vector3) -> Vector3 {
    let center = VORTEX_CENTER.normalized();
    let distance = (p - center).length();
    let influence = 1.0 - smooth(0.0, VORTEX_RADIUS, distance);
    let angle = VORTEX_STRENGTH * influence * influence;
    let (s, c) = angle.sin_cos();
    p * c + center.cross(p) * s + center * center.dot(p) * (1.0 - c)
}
