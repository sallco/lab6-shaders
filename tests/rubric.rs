use lab6_shaders::{
    camera::Camera,
    framebuffer::{Framebuffer, rgba},
    obj::Obj,
    renderer::Renderer,
    shaders::{
        Body, Uniforms, fragment_shader,
        rocky::{bump_normal, cloud_mask, glass_reflection, night_emission, terrain},
    },
    triangle::triangle,
};
use raylib::prelude::*;

fn sphere() -> Obj {
    Obj::load(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/sphere.obj")).unwrap()
}

#[test]
fn all_bodies_keep_identical_depths_at_different_animation_times() {
    let obj = sphere();
    let geometry = obj.vertices.clone();
    let indices = obj.indices.clone();
    let camera = Camera::default();
    let mut fb = Framebuffer::new(200, 140);
    let mut renderer = Renderer::new(200, 140);
    let mut reference = None;
    for body in [Body::Star, Body::Rocky, Body::Gas] {
        let mut u = Uniforms::new(body, camera.eye());
        for time in [0.0, 1.0, 6.0] {
            u.time = time;
            renderer.render(&mut fb, &obj, &camera, &u);
            if let Some(depths) = &reference {
                assert_eq!(depths, &fb.depths);
            } else {
                reference = Some(fb.depths.clone());
            }
        }
    }
    assert_eq!(geometry, obj.vertices);
    assert_eq!(indices, obj.indices);
}

#[test]
fn optional_lighting_effects_are_visible_on_real_surface_fragments() {
    let obj = sphere();
    let mut counts = [0_usize; 3];
    for yaw in [0.0, 0.7, 1.4, 2.1] {
        let camera = Camera {
            yaw,
            ..Camera::default()
        };
        let mut fb = Framebuffer::new(320, 224);
        let projected = camera.project(&obj, fb.width, fb.height);
        let mut fragments = vec![None; fb.width * fb.height];
        for &[a, b, c] in obj.indices.as_chunks::<3>().0 {
            triangle(&mut fb, &[projected[a], projected[b], projected[c]], |f| {
                fragments[f.screen_index] = Some(*f);
                Vector3::zero()
            });
        }
        let mut u = Uniforms::new(Body::Rocky, camera.eye());
        u.time = 6.0;
        let mut visible = [0_usize; 3];
        for f in fragments.iter().flatten() {
            let p = f.position.normalized();
            let terrain = terrain(p, &u);
            let n = bump_normal(p, f.normal, &u);
            let opacity = cloud_mask(p, u.time, &u) * 0.8 * (1.0 - terrain.lava * 0.35);
            let full = fragment_shader(f, &u);
            let night = night_emission(p, f.normal, terrain.lava, &u) * (1.0 - opacity);
            let glass = glass_reflection(f, n, terrain.glass, &u) * (1.0 - opacity);
            if pixel_difference(full, full - night) >= 12 {
                visible[0] += 1;
            }
            if pixel_difference(full, full - glass) >= 12 {
                visible[1] += 1;
            }
            let diffuse_change = (n.dot(u.light_direction).max(0.0)
                - f.normal.dot(u.light_direction).max(0.0))
            .abs();
            if terrain.lava < 0.1 && opacity < 0.2 && diffuse_change > 0.06 {
                visible[2] += 1;
            }
        }
        println!(
            "yaw {yaw:.1}: minerales nocturnos {} px; reflejos de vidrio {} px; relieve {} px",
            visible[0], visible[1], visible[2]
        );
        for (count, value) in counts.iter_mut().zip(visible) {
            *count += value;
        }
    }
    assert!(
        counts[0] > 50,
        "La emisión nocturna debe resultar visible en la esfera renderizada: {counts:?}"
    );
    assert!(
        counts[1] > 30,
        "Los reflejos deben resultar visibles en el vidrio real: {counts:?}"
    );
    assert!(
        counts[2] > 100,
        "El relieve debe cambiar perceptiblemente la iluminación de la roca: {counts:?}"
    );
}

fn pixel_difference(a: Vector3, b: Vector3) -> u8 {
    let a = rgba(a);
    let b = rgba(b);
    (0..3).map(|i| a[i].abs_diff(b[i])).max().unwrap()
}
