use lab6_shaders::{
    camera::Camera,
    framebuffer::Framebuffer,
    obj::Obj,
    renderer::render,
    shaders::{Body, Uniforms},
};
use raylib::prelude::*;
use std::process::ExitCode;
const WIDTH: usize = 1000;
const HEIGHT: usize = 700;
fn run() -> Result<(), Box<dyn std::error::Error>> {
    let obj = Obj::load(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/sphere.obj"))?;
    let camera = Camera::default();
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args == ["--export-captures"] {
        let directory = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("docs");
        std::fs::create_dir_all(&directory)?;
        let mut fb = Framebuffer::new(WIDTH, HEIGHT);
        for (body, filename) in [
            (Body::Star, "cerula.png"),
            (Body::Rocky, "obsidia.png"),
            (Body::Gas, "artaud.png"),
        ] {
            let mut uniforms = Uniforms::new(body, camera.eye());
            uniforms.time = 6.0;
            let started = std::time::Instant::now();
            let fragments = render(&mut fb, &obj, &camera, &uniforms);
            fb.export_png(&directory.join(filename))?;
            println!(
                "{}: {} triángulos, {fragments} fragmentos, {:.1} ms",
                body.name(),
                obj.indices.len() / 3,
                started.elapsed().as_secs_f64() * 1000.0
            );
        }
        return Ok(());
    }
    if !args.is_empty() {
        return Err("Uso: cargo run --release -- [--export-captures]".into());
    }
    let uniforms = Uniforms::new(Body::Star, camera.eye());
    let mut fb = Framebuffer::new(WIDTH, HEIGHT);
    let (mut window, thread) = raylib::init()
        .size(WIDTH as i32, HEIGHT as i32)
        .title("Lab 6 | Shaders en CPU")
        .build();
    window.set_target_fps(60);
    let image = Image::gen_image_color(WIDTH as i32, HEIGHT as i32, Color::BLACK);
    let mut texture = window.load_texture_from_image(&thread, &image)?;
    while !window.window_should_close() {
        render(&mut fb, &obj, &camera, &uniforms);
        texture.update_texture(&fb.pixels)?;
        let mut draw = window.begin_drawing(&thread);
        draw.clear_background(Color::BLACK);
        draw.draw_texture(&texture, 0, 0, Color::WHITE);
    }
    Ok(())
}
fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("Error: {error}");
            ExitCode::FAILURE
        }
    }
}
