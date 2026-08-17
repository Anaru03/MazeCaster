pub struct Texture {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u8>,
}

impl Texture {
    pub fn load(path: &str) -> Self {
        let image = image::open(path)
            .expect("No se pudo cargar la textura")
            .to_rgb8();

        Self {
            width: image.width() as usize,
            height: image.height() as usize,
            buffer: image.into_raw(),
        }
    }

    pub fn get_color(&self, u: f32, v: f32) -> u32 {
        let u = u.clamp(0.0, 0.9999);
        let v = v.clamp(0.0, 0.9999);

        let x = (u * self.width as f32) as usize;
        let y = (v * self.height as f32) as usize;
        let index = (y * self.width + x) * 3;

        let r = self.buffer[index] as u32;
        let g = self.buffer[index + 1] as u32;
        let b = self.buffer[index + 2] as u32;

        (r << 16) | (g << 8) | b
    }
}
