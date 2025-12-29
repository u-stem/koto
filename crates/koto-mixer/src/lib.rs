//! Koto Mixer - Mixer console

/// Mixer channel
pub struct MixerChannel {
    pub name: String,
    pub volume: f32,
    pub pan: f32,
    pub mute: bool,
    pub solo: bool,
}

impl MixerChannel {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            volume: 1.0,
            pan: 0.0,
            mute: false,
            solo: false,
        }
    }
}

impl Default for MixerChannel {
    fn default() -> Self {
        Self::new("Channel")
    }
}

/// Mixer console
pub struct Mixer {
    pub channels: Vec<MixerChannel>,
    pub master_volume: f32,
}

impl Mixer {
    pub fn new() -> Self {
        Self {
            channels: Vec::new(),
            master_volume: 1.0,
        }
    }

    pub fn add_channel(&mut self, channel: MixerChannel) -> usize {
        let index = self.channels.len();
        self.channels.push(channel);
        index
    }

    pub fn remove_channel(&mut self, index: usize) {
        if index < self.channels.len() {
            self.channels.remove(index);
        }
    }

    pub fn get_channel(&self, index: usize) -> Option<&MixerChannel> {
        self.channels.get(index)
    }

    pub fn get_channel_mut(&mut self, index: usize) -> Option<&mut MixerChannel> {
        self.channels.get_mut(index)
    }
}

impl Default for Mixer {
    fn default() -> Self {
        Self::new()
    }
}
