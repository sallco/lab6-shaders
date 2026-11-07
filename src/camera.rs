use crate::{obj::Obj, triangle::ScreenVertex};
use raylib::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Camera {
    pub yaw: f32,
    pub pitch: f32,
    pub distance: f32,
}
impl Default for Camera {
    fn default() -> Self {
        Self {
            yaw: 0.0,
            pitch: 0.0,
            distance: 3.1,
        }
    }
}
fn multiply(m: Matrix, v: Vector4) -> Vector4 {
    Vector4::new(
        m.m0 * v.x + m.m4 * v.y + m.m8 * v.z + m.m12 * v.w,
        m.m1 * v.x + m.m5 * v.y + m.m9 * v.z + m.m13 * v.w,
        m.m2 * v.x + m.m6 * v.y + m.m10 * v.z + m.m14 * v.w,
        m.m3 * v.x + m.m7 * v.y + m.m11 * v.z + m.m15 * v.w,
    )
}
impl Camera {
    pub fn update(&mut self, dt: f32, held: impl Fn(KeyboardKey) -> bool) {
        use KeyboardKey::*;
        let dt = dt.clamp(0.0, 0.1);
        let axis =
            |positive, negative| (i32::from(held(positive)) - i32::from(held(negative))) as f32;
        self.yaw =
            (self.yaw + axis(KEY_RIGHT, KEY_LEFT) * dt * 1.3).rem_euclid(std::f32::consts::TAU);
        self.pitch = (self.pitch + axis(KEY_UP, KEY_DOWN) * dt * 1.1).clamp(-1.3, 1.3);
        let zoom_in = held(KEY_EQUAL) || held(KEY_KP_ADD);
        let zoom_out = held(KEY_MINUS) || held(KEY_KP_SUBTRACT);
        self.distance = (self.distance
            * ((i32::from(zoom_out) - i32::from(zoom_in)) as f32 * dt).exp())
        .clamp(1.7, 5.5);
    }

    pub fn eye(&self) -> Vector3 {
        Vector3::new(
            self.yaw.sin() * self.pitch.cos(),
            self.pitch.sin(),
            self.yaw.cos() * self.pitch.cos(),
        ) * self.distance
    }
    pub fn project(&self, obj: &Obj, width: usize, height: usize) -> Vec<ScreenVertex> {
        let view = Matrix::look_at(self.eye(), Vector3::zero(), Vector3::new(0.0, 1.0, 0.0));
        let projection = Matrix::perspective(
            45.0_f32.to_radians(),
            width as f32 / height as f32,
            0.1,
            50.0,
        );
        obj.vertices
            .iter()
            .zip(&obj.normals)
            .map(|(&position, &normal)| {
                let p = multiply(view, Vector4::new(position.x, position.y, position.z, 1.0));
                let clip = multiply(projection, p);
                // shortcut: la cámara está fuera de la esfera; añadir recorte cercano si se permite entrar en modelos.
                debug_assert!(clip.w > 0.1);
                let inv_w = 1.0 / clip.w;
                ScreenVertex {
                    screen: Vector2::new(
                        (clip.x * inv_w * 0.5 + 0.5) * width as f32,
                        (0.5 - clip.y * inv_w * 0.5) * height as f32,
                    ),
                    depth: clip.z * inv_w,
                    inv_w,
                    position,
                    normal,
                }
            })
            .collect()
    }
}
