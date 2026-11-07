use lab6_shaders::shaders::procedural::Noise;
use raylib::prelude::*;
#[test]
fn procedural_noise_is_bounded_and_reproducible() {
    let a = Noise::default();
    let b = Noise::default();
    for i in 0..50 {
        let p = Vector3::new(i as f32 * 0.13, -0.7, 1.3);
        assert_eq!(a.fbm(p, 3), b.fbm(p, 3));
        assert!((0.0..=1.0).contains(&a.fbm(p, 3)));
        assert!((0.0..=1.0).contains(&a.evolving(p, 2.0, 3)));
        assert_ne!(a.evolving(p, 0.0, 3), a.evolving(p, 4.0, 3));
    }
}
