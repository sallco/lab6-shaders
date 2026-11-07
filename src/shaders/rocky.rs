use super::{
    Uniforms,
    procedural::{mix, rgb, smooth},
};
use crate::triangle::Fragment;
use raylib::prelude::*;

pub const BUMP_STRENGTH: f32 = 0.038;
pub const BUMP_STEP: f32 = 0.003;

pub const CLOUD_COVERAGE: f32 = 0.49;
pub const CLOUD_SPEED: f32 = 0.18;

pub const NIGHT_GLOW: f32 = 0.75;

pub const GLASS_SPECULAR: f32 = 1.6;
pub const GLASS_SHININESS: f32 = 80.0;

pub const TERRAIN_FREQUENCY: f32 = 4.3;
pub const LAVA_WIDTH: f32 = 0.065;
pub const LAVA_FLOW_SPEED: f32 = 2.8;

pub struct Terrain {
    pub albedo: Vector3,
    pub lava: f32,
    pub glass: f32,
}

fn craters(p: Vector3) -> (f32, f32) {
    let mut rim: f32 = 0.0;
    let mut basin: f32 = 0.0;
    for (center, radius) in [
        (Vector3::new(0.4, 0.45, 0.798), 0.15),
        (Vector3::new(-0.55, -0.25, 0.797), 0.12),
        (Vector3::new(0.12, -0.62, 0.775), 0.17),
        (Vector3::new(-0.7, 0.52, 0.48), 0.10),
        (Vector3::new(0.68, -0.45, 0.58), 0.09),
        (Vector3::new(0.3, 0.75, -0.59), 0.14),
        (Vector3::new(-0.5, -0.8, -0.33), 0.11),
        (Vector3::new(0.7, 0.2, -0.68), 0.16),
    ] {
        let d = (p - center.normalized()).length();
        rim = rim.max(1.0 - smooth(0.0, 0.025, (d - radius).abs()));
        basin = basin.max(1.0 - smooth(radius * 0.5, radius, d));
    }
    (rim, basin)
}

pub fn terrain(p: Vector3, u: &Uniforms) -> Terrain {
    let q = u.noise.warp(p * TERRAIN_FREQUENCY, 0.6);
    let fault = u.noise.sample(q).abs();
    let branches = u.noise.sample(q * 2.7 + Vector3::new(9.0, 2.0, 6.0)).abs();
    let major = 1.0 - smooth(0.018, LAVA_WIDTH, fault);
    let minor = (1.0 - smooth(0.006, 0.025, branches)) * (1.0 - smooth(0.12, 0.32, fault));
    let lava = (major + minor * 0.55).clamp(0.0, 1.0);
    let plates = u.noise.fbm(q * 1.4, 2);
    let grain = u.noise.sample(p * 65.0) * 0.5 + 0.5;
    let (rim, basin) = craters(p);
    let glass = smooth(
        0.49,
        0.64,
        u.noise.fbm(p * 3.8 + Vector3::new(4.0, 19.0, 2.0), 2),
    ) * (1.0 - lava);
    let mut albedo = mix(rgb(0x171c2c), rgb(0x575260), plates * 0.8 + grain * 0.2);
    albedo = mix(albedo, rgb(0x9292a4), rim * 0.5);
    albedo *= 1.0 - basin * 0.5;
    albedo = mix(albedo, rgb(0x143e4b), glass * 0.8);
    Terrain {
        albedo,
        lava,
        glass,
    }
}

pub fn rocky_shader(f: &Fragment, u: &Uniforms) -> Vector3 {
    let p = f.position.normalized();
    let terrain = terrain(p, u);
    let normal = bump_normal(p, f.normal, u);
    let day = normal.dot(u.light_direction).max(0.0);
    let mut color = terrain.albedo * (0.11 + 0.89 * day);
    let molten = lava_emission(p, terrain.lava, u);
    color = mix(color, molten, terrain.lava);
    color += night_emission(p, f.normal, terrain.lava, u);
    color += glass_reflection(f, normal, terrain.glass, u);
    let cloud = cloud_mask(p, u.time, u);
    color = mix(
        color,
        rgb(0xb8c5d2) * (0.16 + 0.84 * day),
        cloud * 0.8 * (1.0 - terrain.lava * 0.35),
    );
    color
}

pub fn glass_reflection(f: &Fragment, normal: Vector3, mask: f32, u: &Uniforms) -> Vector3 {
    let view = (u.camera_position - f.position).normalized();
    let half = (view + u.light_direction).normalized();
    let specular = normal.dot(half).max(0.0).powf(GLASS_SHININESS) * GLASS_SPECULAR * mask;
    rgb(0xb8edff) * specular * smooth(0.0, 0.15, normal.dot(u.light_direction))
}

pub fn night_emission(p: Vector3, normal: Vector3, lava: f32, u: &Uniforms) -> Vector3 {
    let night = 1.0 - smooth(-0.12, 0.18, normal.dot(u.light_direction));
    let thread = 1.0
        - smooth(
            0.008,
            0.03,
            u.noise
                .sample(p * 26.0 + Vector3::new(17.0, 4.0, 9.0))
                .abs(),
        );
    let minerals = smooth(0.49, 0.67, u.noise.fbm(p * 8.0, 2));
    rgb(0x60cbd6) * thread * minerals * night * NIGHT_GLOW * (1.0 - lava)
}

pub fn cloud_mask(p: Vector3, time: f32, u: &Uniforms) -> f32 {
    let (s, c) = (time * CLOUD_SPEED * (1.0 + 0.35 * (p.y * 4.0).sin())).sin_cos();
    let drift = Vector3::new(p.x * c + p.z * s, p.y, -p.x * s + p.z * c);
    let q = u.noise.warp(
        drift * 3.8 + Vector3::new(3.0, 14.0 + (time * 0.7).sin() * 0.35, 6.0),
        0.9,
    );
    smooth(CLOUD_COVERAGE, CLOUD_COVERAGE + 0.17, u.noise.fbm(q, 3))
}

pub fn height(p: Vector3, u: &Uniforms) -> f32 {
    let q = u.noise.warp(p * TERRAIN_FREQUENCY, 0.6);
    let lava = 1.0 - smooth(0.018, LAVA_WIDTH, u.noise.sample(q).abs());
    let (rim, basin) = craters(p);
    u.noise.sample(p * 7.0) * 0.16 + rim * 0.12 - basin * 0.09 - lava * 0.06
}

pub fn bump_normal(p: Vector3, normal: Vector3, u: &Uniforms) -> Vector3 {
    let axis = if normal.y.abs() < 0.9 {
        Vector3::new(0.0, 1.0, 0.0)
    } else {
        Vector3::new(1.0, 0.0, 0.0)
    };
    let tangent = normal.cross(axis).normalized();
    let bitangent = normal.cross(tangent).normalized();
    let du = (height((p + tangent * BUMP_STEP).normalized(), u)
        - height((p - tangent * BUMP_STEP).normalized(), u))
        / (2.0 * BUMP_STEP);
    let dv = (height((p + bitangent * BUMP_STEP).normalized(), u)
        - height((p - bitangent * BUMP_STEP).normalized(), u))
        / (2.0 * BUMP_STEP);
    (normal - tangent * du * BUMP_STRENGTH - bitangent * dv * BUMP_STRENGTH).normalized()
}

pub fn lava_emission(p: Vector3, mask: f32, u: &Uniforms) -> Vector3 {
    if mask < 0.001 {
        return rgb(0x92162f);
    }
    let phase = p.dot(Vector3::new(14.0, -9.0, 17.0)) - u.time * LAVA_FLOW_SPEED
        + u.noise.sample(p * 9.0) * 2.5;
    let surge = smooth(0.15, 0.92, 0.5 + 0.5 * phase.sin());
    let current = mix(rgb(0xa91e3b), rgb(0xff6b58), surge);
    mix(
        current,
        rgb(0xffedbf),
        surge.powi(3) * smooth(0.35, 1.0, mask) * 0.95,
    )
}
