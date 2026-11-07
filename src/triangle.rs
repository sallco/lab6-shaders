use crate::framebuffer::Framebuffer;
use raylib::prelude::*;

#[derive(Clone, Copy, Debug)]
pub struct ScreenVertex {
    pub screen: Vector2,
    pub depth: f32,
    pub inv_w: f32,
    pub position: Vector3,
    pub normal: Vector3,
}

#[derive(Clone, Copy, Debug)]
pub struct Fragment {
    pub position: Vector3,
    pub normal: Vector3,
}

// Función de arista y recorrido baricéntrico adaptados de filling-triangles.
fn edge(a: Vector2, b: Vector2, p: Vector2) -> f32 {
    (b.x - a.x) * (p.y - a.y) - (b.y - a.y) * (p.x - a.x)
}

pub fn triangle(
    fb: &mut Framebuffer,
    vertices: &[ScreenVertex; 3],
    mut shade: impl FnMut(&Fragment) -> Vector3,
) -> usize {
    let [a, mut b, mut c] = *vertices;
    let mut area = edge(a.screen, b.screen, c.screen);
    if area.abs() < 1e-6 {
        return 0;
    }
    if area < 0.0 {
        std::mem::swap(&mut b, &mut c);
        area = -area;
    }
    let min_x = a.screen.x.min(b.screen.x).min(c.screen.x).floor().max(0.0) as i32;
    let max_x = a
        .screen
        .x
        .max(b.screen.x)
        .max(c.screen.x)
        .ceil()
        .min(fb.width as f32 - 1.0) as i32;
    let min_y = a.screen.y.min(b.screen.y).min(c.screen.y).floor().max(0.0) as i32;
    let max_y = a
        .screen
        .y
        .max(b.screen.y)
        .max(c.screen.y)
        .ceil()
        .min(fb.height as f32 - 1.0) as i32;
    let mut count = 0;
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let p = Vector2::new(x as f32 + 0.5, y as f32 + 0.5);
            let wa = edge(b.screen, c.screen, p) / area;
            let wb = edge(c.screen, a.screen, p) / area;
            let wc = edge(a.screen, b.screen, p) / area;
            if wa < 0.0 || wb < 0.0 || wc < 0.0 {
                continue;
            }
            let index = y as usize * fb.width + x as usize;
            let depth = wa * a.depth + wb * b.depth + wc * c.depth;
            if !(-1.0..=1.0).contains(&depth) {
                continue;
            }
            let denominator = wa * a.inv_w + wb * b.inv_w + wc * c.inv_w;
            let qa = wa * a.inv_w / denominator;
            let qb = wb * b.inv_w / denominator;
            let qc = wc * c.inv_w / denominator;
            let normal = a.normal * qa + b.normal * qb + c.normal * qc;
            let fragment = Fragment {
                position: a.position * qa + b.position * qb + c.position * qc,
                normal: normal.normalized(),
            };
            fb.write_color(index, shade(&fragment));
            count += 1;
        }
    }
    count
}
