use lab6_shaders::{
    bloom::Bloom,
    framebuffer::{BACKGROUND, Framebuffer},
};
use raylib::prelude::*;
#[test]
fn emissive_surface_spreads_light_without_changing_depth_or_geometry() {
    let mut fb = Framebuffer::new(96, 96);
    for y in 36..60 {
        for x in 36..60 {
            let i = y * 96 + x;
            fb.depths[i] = 0.0;
            fb.write_color(i, Vector3::new(0.55, 0.85, 1.0));
        }
    }
    let depths = fb.depths.clone();
    Bloom::new(96, 96).apply(&mut fb);
    let outside = 48 * 96 + 66;
    assert!(fb.pixels[outside * 4 + 2] > BACKGROUND[2] + 3);
    assert!(fb.depths[outside].is_infinite());
    assert_eq!(depths, fb.depths);
}
#[test]
fn dark_frames_have_no_glow() {
    let mut fb = Framebuffer::new(31, 23);
    let original = fb.pixels.clone();
    Bloom::new(31, 23).apply(&mut fb);
    assert_eq!(original, fb.pixels);
}
