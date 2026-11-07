use crate::framebuffer::{Framebuffer, rgba};
use raylib::prelude::*;

pub const BLOOM_STRENGTH: f32 = 1.4;
pub const BLOOM_RADIUS: usize = 8;
const SCALE: usize = 4;

pub struct Bloom {
    width: usize,
    height: usize,
    light: Vec<Vector3>,
    scratch: Vec<Vector3>,
}
impl Bloom {
    pub fn new(width: usize, height: usize) -> Self {
        let width = width.div_ceil(SCALE);
        let height = height.div_ceil(SCALE);
        Self {
            width,
            height,
            light: vec![Vector3::zero(); width * height],
            scratch: vec![Vector3::zero(); width * height],
        }
    }
    pub fn apply(&mut self, fb: &mut Framebuffer) {
        self.light.fill(Vector3::zero());
        for (index, pixel) in fb.pixels.as_chunks::<4>().0.iter().enumerate() {
            let color = Vector3::new(pixel[0] as f32, pixel[1] as f32, pixel[2] as f32) / 255.0;
            let bright = ((color.x.max(color.y).max(color.z) - 0.6) / 0.4).max(0.0);
            let small = (index / fb.width / SCALE) * self.width + (index % fb.width / SCALE);
            self.light[small] += color * (bright / (SCALE * SCALE) as f32);
        }
        for _ in 0..3 {
            blur(
                &self.light,
                &mut self.scratch,
                self.width,
                self.height,
                true,
            );
            blur(
                &self.scratch,
                &mut self.light,
                self.width,
                self.height,
                false,
            );
        }
        for (index, pixel) in fb.pixels.as_chunks_mut::<4>().0.iter_mut().enumerate() {
            let x = ((index % fb.width) as f32 + 0.5) / SCALE as f32 - 0.5;
            let y = ((index / fb.width) as f32 + 0.5) / SCALE as f32 - 0.5;
            let x = x.clamp(0.0, (self.width - 1) as f32);
            let y = y.clamp(0.0, (self.height - 1) as f32);
            let ix = x as usize;
            let iy = y as usize;
            let nx = (ix + 1).min(self.width - 1);
            let ny = (iy + 1).min(self.height - 1);
            let tx = x - ix as f32;
            let ty = y - iy as f32;
            let a = self.light[iy * self.width + ix] * (1.0 - tx)
                + self.light[iy * self.width + nx] * tx;
            let b = self.light[ny * self.width + ix] * (1.0 - tx)
                + self.light[ny * self.width + nx] * tx;
            let glow = a * (1.0 - ty) + b * ty;
            let original = Vector3::new(pixel[0] as f32, pixel[1] as f32, pixel[2] as f32) / 255.0;
            // Conservar el detalle de la fotosfera y extender la luz sobre el fondo.
            let gain = if fb.depths[index].is_finite() {
                0.16
            } else {
                BLOOM_STRENGTH
            };
            *pixel = rgba(original + glow * gain);
        }
    }
}
fn blur(source: &[Vector3], target: &mut [Vector3], width: usize, height: usize, horizontal: bool) {
    for y in 0..height {
        for x in 0..width {
            let coord = if horizontal { x } else { y };
            let limit = if horizontal { width } else { height };
            let start = coord.saturating_sub(BLOOM_RADIUS);
            let end = (coord + BLOOM_RADIUS).min(limit - 1);
            let mut total = Vector3::zero();
            for tap in start..=end {
                total += source[if horizontal {
                    y * width + tap
                } else {
                    tap * width + x
                }];
            }
            target[y * width + x] = total / (end - start + 1) as f32;
        }
    }
}
