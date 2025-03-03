pub struct AudioPlaybackState {
    pub is_playing: bool,
    pub should_stop: bool,
    pub selected_song: Option<String>,
    pub song_duration: u32,
    pub song_position: u32,
}

impl AudioPlaybackState {
    pub fn new() -> Self {
        Self {
            is_playing: false,
            should_stop: false,
            selected_song: None,
            song_duration: 0,
            song_position: 0,
        }
    }
}

impl AudioPlaybackState {
    pub fn set_selected_song(&mut self, song: String) {
        self.selected_song = Some(song);
    }

    pub fn set_song_duration(&mut self, duration: u32) {
        self.song_duration = duration;
    }

    pub fn get_song_duration(&self) -> u32 {
        self.song_duration
    }

    pub fn set_song_position(&mut self, position: u32) {
        self.song_position = position;
    }

    pub fn get_song_position(&self) -> u32 {
        self.song_position
    }
}
