use std::sync::{Arc, Mutex};
use std::time::Instant;
use std::path::PathBuf;
use std::thread;
use crate::structs::audio_analyzer::AudioAnalyzer;
use crate::structs::audio_playback_state::AudioPlaybackState;
use crate::structs::visualisation_state::VisualisationState;

pub struct AppState {
    pub analyzer: AudioAnalyzer,
    pub frequencies: Vec<f32>,
    pub playback_state: Arc<Mutex<AudioPlaybackState>>,
    pub visualisation_state: Arc<Mutex<VisualisationState>>,
    pub playback_handle: Option<thread::JoinHandle<()>>,
    pub file_path: Option<PathBuf>,
    pub last_second: Instant,
}

impl AppState {
    pub fn new() -> Self {
        let playback_state = Arc::new(Mutex::new(AudioPlaybackState::new()));
        let visualisation_state = Arc::new(Mutex::new(VisualisationState::new()));
        
        {
            let mut state = visualisation_state.lock().unwrap();
            state.set_initial_color();
        }

        Self {
            analyzer: AudioAnalyzer::new(1024),
            frequencies: vec![0.0f32; 16],
            playback_state,
            visualisation_state,
            playback_handle: None,
            file_path: None,
            last_second: Instant::now(),
        }
    }
}