use lab6_shaders::{
    camera::Camera,
    framebuffer::Framebuffer,
    obj::Obj,
    renderer::render,
    shaders::{
        Body, Uniforms, fragment_shader,
        gas::{VORTEX_CENTER, vortex},
        rocky::{bump_normal, cloud_mask, glass_reflection, night_emission, terrain},
        star::star_shader,
    },
    triangle::Fragment,
};
use raylib::prelude::*;
fn points() -> Vec<Vector3> {
    (0..12)
        .flat_map(|y| {
            (0..24).map(move |x| {
                let latitude = -1.4 + y as f32 * 2.8 / 11.0;
                let longitude = x as f32 * std::f32::consts::TAU / 24.0;
                Vector3::new(
                    longitude.sin() * latitude.cos(),
                    latitude.sin(),
                    longitude.cos() * latitude.cos(),
                )
            })
        })
        .collect()
}
#[test]
fn all_shaders_are_finite_and_reproducible_at_poles_and_seams() {
    let mut positions = points();
    positions.extend([Vector3::new(0.0, 1.0, 0.0), Vector3::new(0.0, -1.0, 0.0)]);
    for body in [Body::Star, Body::Rocky, Body::Gas] {
        let mut u = Uniforms::new(body, Vector3::new(0.0, 0.0, 3.1));
        u.time = 6.0;
        for p in &positions {
            let f = Fragment {
                position: *p * 0.85,
                normal: *p,
            };
            let a = fragment_shader(&f, &u);
            let b = fragment_shader(&f, &u);
            assert_eq!(a, b);
            assert!(a.x.is_finite() && a.y.is_finite() && a.z.is_finite());
            assert!(a.x >= 0.0 && a.y >= 0.0 && a.z >= 0.0);
        }
    }
}
#[test]
fn stellar_material_changes_with_time_but_ignores_external_light() {
    let mut u = Uniforms::new(Body::Star, Vector3::new(0.0, 0.0, 3.1));
    let f = Fragment {
        position: Vector3::new(0.0, 0.0, 0.85),
        normal: Vector3::new(0.0, 0.0, 1.0),
    };
    let a = star_shader(&f, &u);
    u.light_direction = -u.light_direction;
    assert_eq!(a, star_shader(&f, &u));
    u.time = 4.0;
    assert!((a - star_shader(&f, &u)).length() > 0.001);
}
#[test]
fn moving_clouds_do_not_move_the_terrain() {
    let mut u = Uniforms::new(Body::Rocky, Vector3::new(0.0, 0.0, 3.1));
    let mut difference = 0.0;
    for p in points() {
        let a = terrain(p, &u);
        let cloud = cloud_mask(p, 0.0, &u);
        u.time = 8.0;
        let b = terrain(p, &u);
        assert_eq!(a.albedo, b.albedo);
        assert_eq!(a.lava, b.lava);
        assert_eq!(a.glass, b.glass);
        difference += (cloud - cloud_mask(p, 8.0, &u)).abs();
    }
    assert!(difference > 1.0);
}
#[test]
fn mineral_emission_only_appears_on_the_night_side() {
    let mut u = Uniforms::new(Body::Rocky, Vector3::new(0.0, 0.0, 3.1));
    let mut visible = 0.0;
    for p in points() {
        u.light_direction = p;
        assert_eq!(night_emission(p, p, 0.0, &u), Vector3::zero());
        u.light_direction = -p;
        visible += night_emission(p, p, 0.0, &u).length();
    }
    assert!(visible > 0.1);
}
#[test]
fn glass_reflection_depends_on_material_and_view_direction() {
    let mut u = Uniforms::new(Body::Rocky, Vector3::new(0.0, 0.0, 3.1));
    let f = Fragment {
        position: Vector3::new(0.0, 0.0, 0.85),
        normal: Vector3::new(0.0, 0.0, 1.0),
    };
    u.light_direction = f.normal;
    let bright = glass_reflection(&f, f.normal, 1.0, &u);
    assert_eq!(glass_reflection(&f, f.normal, 0.0, &u), Vector3::zero());
    u.camera_position = Vector3::new(3.0, 0.0, 0.0);
    assert!(bright.length() > glass_reflection(&f, f.normal, 1.0, &u).length() + 0.1);
}
#[test]
fn bump_changes_normals_and_light_response_without_moving_vertices() {
    let u = Uniforms::new(Body::Rocky, Vector3::new(0.0, 0.0, 3.1));
    let mut difference = 0.0;
    for p in points() {
        let n = bump_normal(p, p, &u);
        assert!((n.length() - 1.0).abs() < 0.0001);
        difference += (n.dot(u.light_direction) - p.dot(u.light_direction)).abs();
    }
    assert!(difference > 1.0);
}
#[test]
fn vortex_changes_local_coordinates_and_leaves_distant_bands_unchanged() {
    let far = Vector3::new(0.0, 1.0, 0.0);
    assert!((vortex(far) - far).length() < 0.0001);
    let near = (VORTEX_CENTER + Vector3::new(0.18, 0.05, 0.0)).normalized();
    let warped = vortex(near);
    assert!((warped - near).length() > 0.02);
    assert!((warped.length() - 1.0).abs() < 0.0001);
}
#[test]
fn three_bodies_share_geometry_and_have_distinct_reproducible_images() {
    let obj = Obj::load(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/sphere.obj")).unwrap();
    let positions = obj.vertices.clone();
    let indices = obj.indices.clone();
    let camera = Camera::default();
    let mut fb = Framebuffer::new(160, 112);
    let mut images = Vec::new();
    for body in [Body::Star, Body::Rocky, Body::Gas] {
        let mut u = Uniforms::new(body, camera.eye());
        u.time = 6.0;
        assert!(render(&mut fb, &obj, &camera, &u) > 0);
        let a = fb.pixels.clone();
        render(&mut fb, &obj, &camera, &u);
        assert_eq!(a, fb.pixels);
        images.push(a);
    }
    assert_ne!(images[0], images[1]);
    assert_ne!(images[1], images[2]);
    assert_eq!(positions, obj.vertices);
    assert_eq!(indices, obj.indices);
}
