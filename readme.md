# Music Visualizer

A real-time music visualization application written in Rust using SDL2.

## Features
- Audio playback support for MP3, WAV, and OGG files
- Real-time FFT-based visualisation
- Customisable visualisation colors
- Play/pause and file loading controls

## Dependencies
- SDL2
- SDL2_ttf

## Installation

### Windows
1. Install [Rust](https://rustup.rs/)
2. Install SDL2 development libraries:
   - Download SDL2 and SDL2_ttf development libraries
   - Place the DLL files in your project's target/debug directory


   
3. The executable and required files will be in the `release` directory

## Usage
1. Navigate to the release directory
2. Run the executable
3. Click "Load song" to select an audio file
4. Use the RGB sliders to customize visualization colors
5. Click "Play" to start playback
6. Click "Randomiser" to randomise the visualisation (updates every second)

## Development
- Build debug version: `cargo build`
- Run debug version: `cargo run`
- Build release version: `cargo build --release`

## things to do 

- Add tests to improve code quality and DX
- Add a settings menu
- Add a help menu
- Add playlist support
- Add a volume slider
