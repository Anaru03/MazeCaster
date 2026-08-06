pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u8>,
    pub background_color: u32,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize, background_color: u32) -> Self {
        let mut fb = Framebuffer {
            width,
            height,
            buffer: vec![0; width * height * 3],
            background_color,
        };
        fb.clear();
        fb
    }

    pub fn clear(&mut self) {
        let (r, g, b) = Self::unpack_color(self.background_color);
        for i in 0..(self.width * self.height) {
            self.buffer[i * 3] = r;
            self.buffer[i * 3 + 1] = g;
            self.buffer[i * 3 + 2] = b;
        }
    }

    pub fn set_pixel(&mut self, x: i32, y: i32, color: u32) {
        if x < 0 || y < 0 || x as usize >= self.width || y as usize >= self.height {
            return;
        }
        let (r, g, b) = Self::unpack_color(color);
        let idx = (y as usize * self.width + x as usize) * 3;
        self.buffer[idx] = r;
        self.buffer[idx + 1] = g;
        self.buffer[idx + 2] = b;
    }

    pub fn get_color(&self, x: i32, y: i32) -> u32 {
        if x < 0 || y < 0 || x as usize >= self.width || y as usize >= self.height {
            return self.background_color;
        }

        let idx = (y as usize * self.width + x as usize) * 3;

        let r = self.buffer[idx] as u32;
        let g = self.buffer[idx + 1] as u32;
        let b = self.buffer[idx + 2] as u32;

        (r << 16) | (g << 8) | b
    }

    pub fn unpack_color(color: u32) -> (u8, u8, u8) {
        let r = ((color >> 16) & 0xFF) as u8;
        let g = ((color >> 8) & 0xFF) as u8;
        let b = (color & 0xFF) as u8;

        (r, g, b)
    }

    pub fn render_to_file(&self, filename: &str) {
        image::save_buffer(
            filename,
            &self.buffer,
            self.width as u32,
            self.height as u32,
            image::ColorType::Rgb8,
        )
        .expect("No se pudo guardar la imagen");
    }
}