use crate::{
    camera::Camera,
    framebuffer::Framebuffer,
    obj::Obj,
    shaders::{Uniforms, fragment_shader},
    triangle::triangle,
};

pub fn render(fb: &mut Framebuffer, obj: &Obj, camera: &Camera, uniforms: &Uniforms) -> usize {
    fb.clear();
    let vertices = camera.project(obj, fb.width, fb.height);
    let mut shaded = 0;
    for &[a, b, c] in obj.indices.as_chunks::<3>().0 {
        shaded += triangle(fb, &[vertices[a], vertices[b], vertices[c]], |f| {
            fragment_shader(f, uniforms)
        });
    }
    shaded
}
