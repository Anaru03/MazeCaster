use rodio::{Decoder, DeviceSinkBuilder, MixerDeviceSink, Player, Source};
use std::fs::File;

pub struct Audio {
    _device: MixerDeviceSink,
    music: Player,
    effects: Player,
}

impl Audio {
    pub fn new() -> Self {
        let device = DeviceSinkBuilder::open_default_sink().expect("No se pudo iniciar el audio");

        let music = Player::connect_new(device.mixer());
        let effects = Player::connect_new(device.mixer());

        Self {
            _device: device,
            music,
            effects,
        }
    }

    pub fn play_music(&self, path: &str) {
        let file = File::open(path).expect("No se pudo abrir la musica");

        let source = Decoder::try_from(file)
            .expect("No se pudo reproducir la musica")
            .repeat_infinite();

        self.music.append(source);
    }

    pub fn stop_music(&self) {
        self.music.stop();
    }

    pub fn play_effect(&self, path: &str) {
        let file = File::open(path).expect("No se pudo abrir el efecto");

        let source = Decoder::try_from(file).expect("No se pudo reproducir el efecto");

        self.effects.append(source);
    }
}