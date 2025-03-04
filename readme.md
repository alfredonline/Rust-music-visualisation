# Music Visualizer

A real-time music visualization application written in Rust using SDL2.

## Features
- Audio playback support for MP3, WAV, and OGG files
- Real-time FFT-based visualization
- Customizable visualization colors
- Play/pause and file loading controls

## Download and Run
1. Go to the [Releases](https://github.com/yourusername/music-visualizer/releases) page
2. Download the latest release zip file for your platform
3. Extract the zip file
4. Run the executable

## Usage
1. Click "Load song" to select an audio file
2. Use the RGB sliders to customize visualization colors
3. Click "Play" to start playback
4. Click "Randomiser" to randomize the visualization (updates every second)

## Future Improvements
- Add a settings menu
- Add a help menu
- Add playlist support
- Add a volume slider

## For Developers
If you want to build from source:

### Dependencies
- Rust
- SDL2
- SDL2_ttf

### Building
1. Clone the repository
2. Run `cargo build --release`
3. Run `cargo run --bin prepare_release` to create a distributable package

