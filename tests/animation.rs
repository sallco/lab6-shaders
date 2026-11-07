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
