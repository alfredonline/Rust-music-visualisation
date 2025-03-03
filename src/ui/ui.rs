use crate::structs::audio_playback_state::AudioPlaybackState;
use crate::structs::element::Element;
use sdl2::pixels::Color;

pub fn update_duration_display<'a>(
    state: &AudioPlaybackState,
    font: &'a sdl2::ttf::Font,
    texture_creator: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    song_duration: &mut Element,
) -> Result<sdl2::render::Texture<'a>, Box<dyn std::error::Error>> {
    let duration = state.get_song_duration();
    let minutes = duration / 60;
    let seconds = duration % 60;
    let duration_str = format!("{:02}:{:02}", minutes, seconds);

    let texture = create_element_texture(font, texture_creator, &duration_str)?;
    song_duration.update_text_position(texture.query().width, texture.query().height);
    Ok(texture)
    }


    
pub fn create_element_texture<'a>(
    font: &'a sdl2::ttf::Font,
    texture_creator: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    text: &str,
) -> Result<sdl2::render::Texture<'a>, Box<dyn std::error::Error>> {
    let surface = font
        .render(text)
        .blended(Color::RGB(47, 198, 18))
        .map_err(|e| e.to_string())?;

    texture_creator
        .create_texture_from_surface(&surface)
        .map_err(|e| e.to_string().into())
}


pub fn create_play_button_texture<'a>(
    font: &'a sdl2::ttf::Font,
    texture_creator: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    is_playing: bool,
) -> Result<sdl2::render::Texture<'a>, Box<dyn std::error::Error>> {
    let surface = font
        .render(if is_playing { "Pause" } else { "Play" })
        .blended(Color::RGB(47, 198, 18))
        .map_err(|e| e.to_string())?;

    texture_creator
        .create_texture_from_surface(&surface)
        .map_err(|e| e.to_string().into())
}

