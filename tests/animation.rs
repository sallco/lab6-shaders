use lab6_shaders::shaders::{
    Body, Uniforms,
    rocky::{lava_emission, terrain},
};
use raylib::prelude::*;
#[test]
fn heat_waves_change_lava_color_without_moving_faults() {
    let mut u = Uniforms::new(Body::Rocky, Vector3::new(0.0, 0.0, 3.1));
    let mut variation = 0.0;
    for i in 0..100 {
        let p = Vector3::new((i as f32 * 0.33).sin(), (i as f32 * 0.21).cos(), 1.0).normalized();
        let a = terrain(p, &u);
        let before = lava_emission(p, a.lava, &u);
        u.time = 1.0;
        let b = terrain(p, &u);
        variation += (lava_emission(p, b.lava, &u) - before).length();
        assert_eq!(a.lava, b.lava);
        assert_eq!(a.albedo, b.albedo);
        u.time = 0.0;
    }
    assert!(variation > 3.0);
}

#[test]
fn planetary_materials_change_visibly_in_one_second_without_geometry_motion() {
    use lab6_shaders::{camera::Camera, framebuffer::Framebuffer, obj::Obj, renderer::Renderer};
    let obj = Obj::load(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/sphere.obj")).unwrap();
    let camera = Camera::default();
    let mut fb = Framebuffer::new(320, 224);
    let mut renderer = Renderer::new(320, 224);
    for body in [Body::Rocky, Body::Gas] {
        let mut u = Uniforms::new(body, camera.eye());
        renderer.render(&mut fb, &obj, &camera, &u);
        let initial = fb.pixels.clone();
        let depths = fb.depths.clone();
        u.time = 1.0;
        renderer.render(&mut fb, &obj, &camera, &u);
        assert_eq!(depths, fb.depths);
        let (mut count, mut changed, mut total) = (0, 0, 0.0);
        for (i, depth) in depths.iter().enumerate() {
            if depth.is_finite() {
                count += 1;
                let differences = [0, 1, 2].map(|channel| {
                    initial[i * 4 + channel].abs_diff(fb.pixels[i * 4 + channel]) as f32 / 255.0
                });
                if differences.iter().copied().fold(0.0, f32::max) > 16.0 / 255.0 {
                    changed += 1;
                }
                total += differences.iter().sum::<f32>() / 3.0;
            }
        }
        let ratio = changed as f32 / count as f32;
        let mean = total / count as f32;
        println!(
            "{}: {:.1}% de la superficie cambia perceptiblemente en 1 s; diferencia media {:.3}",
            body.name(),
            ratio * 100.0,
            mean
        );
        assert!(
            ratio > 0.3,
            "{} necesita más movimiento visible",
            body.name()
        );
        assert!(
            mean > 0.035,
            "{} necesita más contraste temporal",
            body.name()
        );
    }
}
