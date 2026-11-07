use super::{
    Uniforms,
    procedural::{mix, rgb, smooth},
};
use crate::triangle::Fragment;
use raylib::prelude::*;

pub const CELL_FREQUENCY: f32 = 5.6;
pub const FILAMENT_CONTRAST: f32 = 10.0;
pub const ACTIVITY_SPEED: f32 = 0.22;

pub fn star_shader(f: &Fragment, u: &Uniforms) -> Vector3 {
    let p = f.position.normalized();
    let q = u.noise.warp(p * CELL_FREQUENCY, 0.85);
    let cells = u.noise.evolving(q * 1.4, u.time * ACTIVITY_SPEED, 3);
    let activity = u.noise.evolving(q * 2.8, u.time * ACTIVITY_SPEED * 1.7, 2);
    let filament = (1.0 - (activity * 2.0 - 1.0).abs()).powf(FILAMENT_CONTRAST);
    let spots = smooth(
        0.52,
        0.72,
        u.noise.fbm(p * 3.2 + Vector3::new(12.0, 3.0, 8.0), 3),
    );
    let heat = (cells * 0.9 + filament * 0.24 - spots * 0.23).clamp(0.0, 1.0);
    let mut color = mix(rgb(0x1554c4), rgb(0x80d8ff), smooth(0.22, 0.6, heat));
    color = mix(color, rgb(0xf1fbff), smooth(0.53, 0.78, heat) * 0.9);
    color = mix(color, rgb(0xd8f7ff), filament * 0.38);
    let facing = f
        .normal
        .dot((u.camera_position - f.position).normalized())
        .max(0.0);
    color * (0.7 + 0.3 * facing.sqrt())
}
