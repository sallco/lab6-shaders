use lab6_shaders::{
    framebuffer::{BACKGROUND, Framebuffer},
    triangle::{ScreenVertex, triangle},
};
use raylib::prelude::*;

fn vertices(depth: f32) -> [ScreenVertex; 3] {
    [(1.0, 1.0), (7.0, 1.0), (1.0, 7.0)].map(|(x, y)| ScreenVertex {
        screen: Vector2::new(x, y),
        depth,
        inv_w: 1.0,
        position: Vector3::new(x, y, 0.0),
        normal: Vector3::new(0.0, 0.0, 1.0),
    })
}

#[test]
fn depth_is_independent_of_draw_order_and_skips_hidden_shading() {
    let mut first = Framebuffer::new(8, 8);
    triangle(&mut first, &vertices(-0.5), |_| Vector3::new(1.0, 0.0, 0.0));
    let mut calls = 0;
    assert_eq!(
        triangle(&mut first, &vertices(0.5), |_| {
            calls += 1;
            Vector3::new(0.0, 1.0, 0.0)
        }),
        0
    );
    assert_eq!(calls, 0);
    let mut second = Framebuffer::new(8, 8);
    triangle(&mut second, &vertices(0.5), |_| Vector3::new(0.0, 1.0, 0.0));
    triangle(&mut second, &vertices(-0.5), |_| {
        Vector3::new(1.0, 0.0, 0.0)
    });
    assert_eq!(first.pixels, second.pixels);
    first.clear();
    assert!(first.depths.iter().all(|d| d.is_infinite()));
    assert!(
        first
            .pixels
            .as_chunks::<4>()
            .0
            .iter()
            .all(|p| p == &BACKGROUND)
    );
}

#[test]
fn winding_clipping_and_degenerate_faces_are_safe() {
    let mut fb = Framebuffer::new(8, 8);
    let v = vertices(0.0);
    let n = triangle(&mut fb, &v, |_| Vector3::one());
    assert!(n > 0);
    let initial = fb.pixels.clone();
    fb.clear();
    assert_eq!(
        triangle(&mut fb, &[v[0], v[2], v[1]], |_| Vector3::one()),
        n
    );
    assert_eq!(initial, fb.pixels);
    assert_eq!(triangle(&mut fb, &[v[0]; 3], |_| Vector3::one()), 0);
    let outside = v.map(|mut v| {
        v.screen.x -= 50.0;
        v
    });
    assert_eq!(triangle(&mut fb, &outside, |_| Vector3::one()), 0);
}

#[test]
fn attributes_use_perspective_correct_weights() {
    let mut fb = Framebuffer::new(8, 8);
    let mut v = vertices(0.0);
    v[1].inv_w = 0.5;
    let mut found = false;
    triangle(&mut fb, &v, |f| {
        if (f.position.x - 1.2608696).abs() < 0.0001 && (f.position.y - 1.5217391).abs() < 0.0001 {
            found = true;
        }
        assert!((f.normal.length() - 1.0).abs() < 0.0001);
        Vector3::one()
    });
    assert!(found);
}
