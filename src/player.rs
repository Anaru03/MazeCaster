pub struct Player {
    pub x: f32,
    pub y: f32,
    pub angle: f32,
    pub fov: f32,
}

impl Player {
    pub fn new() -> Self {
        Self {
            x: 2.5,
            y: 2.5,
            angle: 0.0,
            fov: std::f32::consts::FRAC_PI_3,
        }
    }
}