use lab6_shaders::{
    camera::Camera,
    framebuffer::Framebuffer,
    obj::Obj,
    renderer::render,
    shaders::{Body, Uniforms},
    state::State,
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
            let render_ms = started.elapsed().as_secs_f64() * 1000.0;
            fb.export_png(&directory.join(filename))?;
            println!(
                "{}: {} triángulos, {fragments} fragmentos, {:.1} ms",
                body.name(),
                obj.indices.len() / 3,
                render_ms
            );
        }
        return Ok(());
    }
    if !args.is_empty() {
        return Err("Uso: cargo run --release -- [--export-captures]".into());
    }
    let mut state = State::default();
    let mut uniforms = Uniforms::new(state.body, state.camera.eye());
    let mut fb = Framebuffer::new(WIDTH, HEIGHT);
    let (mut window, thread) = raylib::init()
        .size(WIDTH as i32, HEIGHT as i32)
        .title("Lab 6 | Cerula - Obsidia - Artaud | Shaders en CPU")
        .build();
    window.set_target_fps(60);
    let image = Image::gen_image_color(WIDTH as i32, HEIGHT as i32, Color::BLACK);
    let mut texture = window.load_texture_from_image(&thread, &image)?;
    let mut dirty = true;
    while !window.window_should_close() {
        let changed = state.update(
            window.get_frame_time(),
            |key| window.is_key_down(key),
            |key| window.is_key_pressed(key),
        );
        if dirty || changed {
            uniforms.body = state.body;
            uniforms.time = state.time;
            uniforms.camera_position = state.camera.eye();
            render(&mut fb, &obj, &state.camera, &uniforms);
            texture.update_texture(&fb.pixels)?;
            dirty = false;
        }
        let status = format!(
            "{}  |  {}  |  {:.1} s  |  {} FPS",
            state.body.name(),
            if state.paused { "Pausa" } else { "Animacion" },
            state.time,
            window.get_fps()
        );
        let mut draw = window.begin_drawing(&thread);
        draw.clear_background(Color::BLACK);
        draw.draw_texture(&texture, 0, 0, Color::WHITE);
        draw.draw_text(&status, 20, 18, 18, Color::new(210, 224, 240, 255));
        draw.draw_text(
            "1 Cerula   2 Obsidia   3 Artaud",
            20,
            44,
            16,
            Color::new(140, 160, 185, 255),
        );
        draw.draw_text(
            "Flechas: orbitar  +/-: zoom  Espacio: pausa  R: reiniciar  Esc: salir",
            20,
            HEIGHT as i32 - 30,
            16,
            Color::new(170, 190, 210, 255),
        );
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
