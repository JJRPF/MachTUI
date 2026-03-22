//! Sound Engine for MachTUI.
//! Provides high-level audio playback and visualization support using Rodio 0.22.

use rodio::{source::Source, Player};

pub struct SoundEngine {
    _handle: rodio::MixerDeviceSink,
    pub player: Player,
}

impl SoundEngine {
    pub fn new() -> Option<Self> {
        if let Ok(handle) = rodio::DeviceSinkBuilder::open_default_sink() {
            let player = Player::connect_new(&handle.mixer());
            return Some(Self {
                _handle: handle,
                player,
            });
        }
        None
    }

    /// Plays a simple sine wave tone for visualization testing.
    pub fn play_tone(&self, freq: f32) {
        let source = rodio::source::SineWave::new(freq)
            .take_duration(std::time::Duration::from_secs_f32(0.2))
            .amplify(0.1);
        self.player.append(source);
    }
}
