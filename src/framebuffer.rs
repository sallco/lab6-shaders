use raylib::prelude::*;

pub const BACKGROUND: [u8; 4] = [5, 8, 16, 255];

pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<u8>,
    pub depths: Vec<f32>,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        assert!(width > 0 && height > 0);
        let mut result = Self {
            width,
            height,
            pixels: vec![0; width * height * 4],
            depths: vec![f32::INFINITY; width * height],
        };
        result.clear();
        result
    }

    pub fn export_png(&self, path: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
        let mut image = Image::gen_image_color(self.width as i32, self.height as i32, Color::BLACK);
        for (index, rgba) in self.pixels.as_chunks::<4>().0.iter().enumerate() {
            image.draw_pixel(
                (index % self.width) as i32,
                (index / self.width) as i32,
                Color::new(rgba[0], rgba[1], rgba[2], rgba[3]),
            );
        }
        std::fs::File::create(path)?;
        image.export_image(path.to_str().ok_or("La ruta PNG no es UTF-8")?);
        if std::fs::metadata(path)?.len() < 8 {
            return Err("No se pudo exportar el PNG".into());
        }
        Ok(())
    }

    pub fn clear(&mut self) {
        for pixel in self.pixels.as_chunks_mut::<4>().0 {
            *pixel = BACKGROUND;
        }
        self.depths.fill(f32::INFINITY);
    }

    pub fn write_color(&mut self, index: usize, color: Vector3) {
        let rgba = &mut self.pixels[index * 4..index * 4 + 4];
        rgba.copy_from_slice(&[
            (color.x.clamp(0.0, 1.0) * 255.0).round() as u8,
            (color.y.clamp(0.0, 1.0) * 255.0).round() as u8,
            (color.z.clamp(0.0, 1.0) * 255.0).round() as u8,
            255,
        ]);
    }
}
