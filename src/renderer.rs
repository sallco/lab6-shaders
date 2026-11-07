use crate::{
    bloom::Bloom,
    camera::Camera,
    framebuffer::{Framebuffer, rgba},
    obj::Obj,
    shaders::{Body, Uniforms, fragment_shader},
    triangle::{Fragment, triangle},
};
use raylib::prelude::*;

pub struct Renderer {
    fragments: Vec<Option<Fragment>>,
    bloom: Bloom,
}
impl Renderer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            fragments: vec![None; width * height],
            bloom: Bloom::new(width, height),
        }
    }
    pub fn render(
        &mut self,
        fb: &mut Framebuffer,
        obj: &Obj,
        camera: &Camera,
        uniforms: &Uniforms,
    ) -> usize {
        assert_eq!(self.fragments.len(), fb.width * fb.height);
        fb.clear();
        self.fragments.fill(None);
        let vertices = camera.project(obj, fb.width, fb.height);
        for &[a, b, c] in obj.indices.as_chunks::<3>().0 {
            triangle(fb, &[vertices[a], vertices[b], vertices[c]], |f| {
                self.fragments[f.screen_index] = Some(*f);
                Vector3::zero()
            });
        }
        let count = self.fragments.iter().flatten().count();
        let workers = std::thread::available_parallelism()
            .map_or(1, |n| n.get())
            .min(8);
        let chunk = self.fragments.len().div_ceil(workers);
        // Cada hilo tiene píxeles exclusivos; la profundidad se resolvió antes del sombreado.
        std::thread::scope(|scope| {
            for (pixels, fragments) in fb
                .pixels
                .chunks_mut(chunk * 4)
                .zip(self.fragments.chunks(chunk))
            {
                scope.spawn(move || {
                    for (pixel, fragment) in pixels.as_chunks_mut::<4>().0.iter_mut().zip(fragments)
                    {
                        if let Some(fragment) = fragment {
                            *pixel = rgba(fragment_shader(fragment, uniforms));
                        }
                    }
                });
            }
        });
        if uniforms.body == Body::Star {
            self.bloom.apply(fb);
        }
        count
    }
}
