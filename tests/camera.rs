use lab6_shaders::{camera::Camera, obj::Obj};
#[test]
fn orbit_keeps_geometry_fixed_and_projection_finite() {
    let obj = Obj::load(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/sphere.obj")).unwrap();
    let original = obj.vertices.clone();
    for yaw in [0.0, 1.0, 3.0, 5.0] {
        for pitch in [-1.3, 0.0, 1.3] {
            let camera = Camera {
                yaw,
                pitch,
                distance: 1.7,
            };
            for v in camera.project(&obj, 1000, 700) {
                assert!(v.screen.x.is_finite() && v.screen.y.is_finite());
                assert!(v.inv_w > 0.0 && (-1.0..=1.0).contains(&v.depth));
            }
        }
    }
    assert_eq!(original, obj.vertices);
}
