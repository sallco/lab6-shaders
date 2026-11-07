use noise::{NoiseFn, OpenSimplex};
use raylib::prelude::*;

pub struct Noise {
    generator: OpenSimplex,
}
impl Default for Noise {
    fn default() -> Self {
        Self {
            generator: OpenSimplex::new(42),
        }
    }
}
impl Noise {
    pub fn sample(&self, p: Vector3) -> f32 {
        self.generator.get([p.x as f64, p.y as f64, p.z as f64]) as f32
    }
    pub fn fbm(&self, mut p: Vector3, octaves: u32) -> f32 {
        let (mut total, mut weight, mut amplitude) = (0.0, 0.0, 1.0);
        for _ in 0..octaves {
            total += self.sample(p) * amplitude;
            weight += amplitude;
            p *= 2.03;
            amplitude *= 0.5;
        }
        (0.5 + 0.5 * total / weight).clamp(0.0, 1.0)
    }
    pub fn evolving(&self, mut p: Vector3, time: f32, octaves: u32) -> f32 {
        let (mut total, mut weight, mut amplitude) = (0.0, 0.0, 1.0);
        for _ in 0..octaves {
            total += self
                .generator
                .get([p.x as f64, p.y as f64, p.z as f64, time as f64]) as f32
                * amplitude;
            weight += amplitude;
            p *= 2.03;
            amplitude *= 0.5;
        }
        (0.5 + 0.5 * total / weight).clamp(0.0, 1.0)
    }
    pub fn warp(&self, p: Vector3, strength: f32) -> Vector3 {
        p + Vector3::new(
            self.sample(p + Vector3::new(19.2, 3.1, 7.7)),
            self.sample(p + Vector3::new(2.6, 17.4, 5.3)),
            self.sample(p + Vector3::new(8.1, 1.4, 29.5)),
        ) * strength
    }
}
pub fn smooth(a: f32, b: f32, x: f32) -> f32 {
    let t = ((x - a) / (b - a)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}
pub fn mix(a: Vector3, b: Vector3, t: f32) -> Vector3 {
    a + (b - a) * t.clamp(0.0, 1.0)
}
pub fn rgb(hex: u32) -> Vector3 {
    Vector3::new(
        ((hex >> 16) & 255) as f32 / 255.0,
        ((hex >> 8) & 255) as f32 / 255.0,
        (hex & 255) as f32 / 255.0,
    )
}
