use rfd::FileDialog;
use sdl2::event::Event;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::ttf::init as ttf_init;
use std::sync::mpsc;
use std::time::Instant;

mod structs;
use structs::audio_playback_state::AudioPlaybackState;
use structs::app_state::AppState;

mod ui;
use structs::sample_wrapper::SampleWrapper;
use ui::visualisation::draw_visualization;
mod audio;
use crate::audio::playback::start_audio_playback;
use structs::colour_slider::ColourSlider;
use structs::element::Element;
use ui::visualisation::update_visualization_bar_colors;
use ui::ui::update_duration_display;
use structs::buttons::Button;   
use structs::buttons::BUTTON_HEIGHT;
use structs::buttons::BUTTON_WIDTH;
use ui::ui::create_element_texture;
use ui::ui::create_play_button_texture;

struct UiElements<'a> {
    open_button: Button,
    play_button: Button,
    randomiser_button: Button,
    periodic_randomiser_button: Button,
    song_name_element: Element,
    song_position_element: Element,
    song_duration: Element,
    separator_element: Element,
    red_slider: ColourSlider,
    green_slider: ColourSlider,
    blue_slider: ColourSlider,
    // Only keep textures that are actually used
    texture_play: sdl2::render::Texture<'a>,
    song_name_texture: sdl2::render::Texture<'a>,
    song_position_texture: sdl2::render::Texture<'a>,
    song_duration_texture: sdl2::render::Texture<'a>,
    separator_texture: sdl2::render::Texture<'a>,
    open_button_texture: sdl2::render::Texture<'a>,
    randomiser_button_texture: sdl2::render::Texture<'a>,
    periodic_randomiser_button_texture: sdl2::render::Texture<'a>,
}

fn setup_sdl() -> Result<(sdl2::Sdl, sdl2::video::Window, sdl2::ttf::Sdl2TtfContext), Box<dyn std::error::Error>> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let ttf_context = ttf_init().map_err(|e| e.to_string())?;

    let window = video_subsystem
        .window("Music Visualizer", 1280, 720)
        .position_centered()
        .build()?;

    Ok((sdl_context, window, ttf_context))
}

fn setup_ui_elements<'a, 'b>(
    font: &'a sdl2::ttf::Font<'a, 'b>,
    texture_creator: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>,
) -> Result<UiElements<'a>, Box<dyn std::error::Error>> {
    let mut open_button = Button::new(10, 10, BUTTON_WIDTH, BUTTON_HEIGHT, "Load song");
    let mut play_button = Button::new(220, 10, BUTTON_WIDTH, BUTTON_HEIGHT, "Play");
    let mut randomiser_button = Button::new(900, 10, BUTTON_WIDTH, BUTTON_HEIGHT, "Randomise");
    let mut periodic_randomiser_button = Button::new(1100, 10, BUTTON_WIDTH, BUTTON_HEIGHT, "Auto");

    // Create elements
    let song_name_element = Element::new(10, 200, 800, 20, "No song selected");
    let song_position_element = Element::new(520, 25, 100, 20, "00:00");
    let separator_element = Element::new(615, 25, 20, 20, "/");
    let song_duration = Element::new(630, 25, 100, 20, "00:00");

    // Create sliders
    let red_slider = ColourSlider::new(900, 100, 300, 20, "R");
    let green_slider = ColourSlider::new(900, 130, 300, 20, "G");
    let blue_slider = ColourSlider::new(900, 160, 300, 20, "B");

    // Create button textures first
    let open_button_texture = create_element_texture(font, texture_creator, "Load song")?;
    let texture_play = create_play_button_texture(font, texture_creator, false)?;
    let randomiser_button_texture = create_element_texture(font, texture_creator, "Randomise")?;
    let periodic_randomiser_button_texture = create_element_texture(font, texture_creator, "Auto")?;

    // Update text positions with correct textures
    open_button.update_text_position(open_button_texture.query().width, open_button_texture.query().height);
    play_button.update_text_position(texture_play.query().width, texture_play.query().height);
    randomiser_button.update_text_position(randomiser_button_texture.query().width, randomiser_button_texture.query().height);
    periodic_randomiser_button.update_text_position(periodic_randomiser_button_texture.query().width, periodic_randomiser_button_texture.query().height);

    Ok(UiElements {
        open_button,
        play_button,
        randomiser_button,
        periodic_randomiser_button,
        song_name_element,
        song_position_element,
        song_duration,
        separator_element,
        red_slider,
        green_slider,
        blue_slider,
        texture_play,
        song_name_texture: create_element_texture(font, texture_creator, "No song selected")?,
        song_position_texture: create_element_texture(font, texture_creator, "00:00")?,
        song_duration_texture: create_element_texture(font, texture_creator, "00:00")?,
        separator_texture: create_element_texture(font, texture_creator, "/")?,
        open_button_texture,
        randomiser_button_texture,
        periodic_randomiser_button_texture,
    })
}

fn main() {
    match run() {
        Ok(_) => {
            println!("Program completed successfully");
            println!("Press Enter to exit...");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            println!("\nPress Enter to exit...");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
        }
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting application...");
    
    let (tx, rx) = mpsc::channel::<SampleWrapper>();
    println!("Created channels...");
    
    let mut app_state = AppState::new();
    println!("Created app state...");
    
    // Setup SDL
    let (sdl_context, window, ttf_context) = setup_sdl()?;
    println!("SDL setup complete...");
    
    let mut canvas = window.into_canvas().build()?;
    let texture_creator = canvas.texture_creator();
    println!("Loading font...");
    let font = ttf_context.load_font("assets/fonts/times.ttf", 24)?;
    
    // Setup UI
    println!("Setting up UI...");
    let mut ui_elements = setup_ui_elements(&font, &texture_creator)?;
    let mut event_pump = sdl_context.event_pump()?;

    println!("Entering main loop...");
    'running: loop {
        // Clear canvas
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    handle_quit(&mut app_state)?;
                    break 'running;
                },
                Event::MouseButtonDown { mouse_btn: MouseButton::Left, x, y, .. } => {
                    handle_mouse_click(&mut app_state, &mut ui_elements, x, y, &tx, &font, &texture_creator)?;
                },
                Event::MouseButtonUp { mouse_btn: MouseButton::Left, .. } => {
                    handle_mouse_up(&mut ui_elements);
                },
                Event::MouseMotion { x, y, .. } => {
                    handle_mouse_motion(&mut app_state, &mut ui_elements, x)?;
                },
                _ => {}
            }
        }

        // Update state
        update_state(&mut app_state, &mut ui_elements, &rx, &font, &texture_creator)?;

        // Draw UI
        draw_ui(&mut canvas, &app_state, &ui_elements)?;
        
        canvas.present();
    }
    Ok(())
}

fn handle_quit(app_state: &mut AppState) -> Result<(), Box<dyn std::error::Error>> {
    {
        let mut state = app_state.playback_state.lock().unwrap();
        state.should_stop = true;
    }
    if let Some(handle) = app_state.playback_handle.take() {
        handle.join().ok();
    }
    Ok(())
}

fn handle_mouse_up(ui_elements: &mut UiElements) {
    ui_elements.red_slider.is_dragging = false;
    ui_elements.green_slider.is_dragging = false;
    ui_elements.blue_slider.is_dragging = false;
}

fn handle_mouse_motion(
    app_state: &mut AppState,
    ui_elements: &mut UiElements,
    x: i32,
) -> Result<(), Box<dyn std::error::Error>> {
    if ui_elements.red_slider.is_dragging {
        ui_elements.red_slider.update(x);
        let mut state = app_state.visualisation_state.lock().unwrap();
        state.set_red_slider(ui_elements.red_slider.value);
    }
    if ui_elements.green_slider.is_dragging {
        ui_elements.green_slider.update(x);
        let mut state = app_state.visualisation_state.lock().unwrap();
        state.set_green_slider(ui_elements.green_slider.value);
    }
    if ui_elements.blue_slider.is_dragging {
        ui_elements.blue_slider.update(x);
        let mut state = app_state.visualisation_state.lock().unwrap();
        state.set_blue_slider(ui_elements.blue_slider.value);
    }
    Ok(())
}

fn handle_mouse_click<'a, 'b>(
    app_state: &mut AppState,
    ui_elements: &mut UiElements<'a>,
    x: i32,
    y: i32,
    tx: &mpsc::Sender<SampleWrapper>,
    font: &'a sdl2::ttf::Font<'a, 'b>,
    texture_creator: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>,
) -> Result<(), Box<dyn std::error::Error>> {
    let click_point = Point::new(x, y);

    if ui_elements.open_button.is_clicked(click_point) {
        handle_open_button(app_state, ui_elements, font, texture_creator)?;
    } else if app_state.file_path.is_some() && ui_elements.play_button.is_clicked(click_point) {
        handle_play_button(app_state, ui_elements, tx, font, texture_creator)?;
    } else if ui_elements.randomiser_button.is_clicked(click_point) {
        handle_randomiser_button(app_state, ui_elements)?;
    } else if ui_elements.periodic_randomiser_button.is_clicked(click_point) {
        handle_periodic_randomiser_button(app_state)?;
    } else {
        handle_slider_click(app_state, ui_elements, x, y)?;
    }

    Ok(())
}

fn handle_open_button<'a, 'b>(
    app_state: &mut AppState,
    ui_elements: &mut UiElements<'a>,
    font: &'a sdl2::ttf::Font<'a, 'b>,
    texture_creator: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(path) = FileDialog::new()
        .add_filter("Audio", &["mp3", "wav", "ogg"])
        .pick_file()
    {
        {
            let mut state = app_state.playback_state.lock().unwrap();
            state.should_stop = true;
            state.is_playing = false;

            let file_name = path
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("No song selected");
            let selected_song = format!("Now playing: {}", file_name);
            state.set_selected_song(selected_song.clone());

            update_song_display(&mut *state, ui_elements, font, texture_creator, &selected_song)?;
        }

        if let Some(handle) = app_state.playback_handle.take() {
            handle.join().ok();
        }
        app_state.file_path = Some(path);
        ui_elements.texture_play = create_play_button_texture(font, texture_creator, false)?;
    }
    Ok(())
}

fn update_song_display<'a, 'b>(
    state: &mut AudioPlaybackState,
    ui_elements: &mut UiElements<'a>,
    font: &'a sdl2::ttf::Font<'a, 'b>,
    texture_creator: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    display_song: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let duration = state.get_song_duration();
    let position = state.get_song_position();
    
    ui_elements.song_name_texture = create_element_texture(font, texture_creator, display_song)?;
    ui_elements.song_position_texture = create_element_texture(
        font, 
        texture_creator, 
        &format!("{:02}:{:02}", position / 60, position % 60)
    )?;
    ui_elements.song_duration_texture = create_element_texture(
        font, 
        texture_creator, 
        &format!("{:02}:{:02}", duration / 60, duration % 60)
    )?;

    update_element_positions(ui_elements);
    Ok(())
}

fn update_element_positions(ui_elements: &mut UiElements<'_>) {
    ui_elements.song_position_element.update_text_position(
        ui_elements.song_position_texture.query().width,
        ui_elements.song_position_texture.query().height,
    );
    ui_elements.song_name_element.update_text_position(
        ui_elements.song_name_texture.query().width,
        ui_elements.song_name_texture.query().height,
    );
    ui_elements.song_duration.update_text_position(
        ui_elements.song_duration_texture.query().width,
        ui_elements.song_duration_texture.query().height,
    );
}

fn handle_play_button<'a, 'b>(
    app_state: &mut AppState,
    ui_elements: &mut UiElements<'a>,
    tx: &mpsc::Sender<SampleWrapper>,
    font: &'a sdl2::ttf::Font<'a, 'b>,
    texture_creator: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut state = app_state.playback_state.lock().unwrap();
    if state.is_playing {
        state.is_playing = false;
    } else {
        if app_state.playback_handle.is_none() {
            state.should_stop = false;
            state.is_playing = true;
            let path = app_state.file_path.as_ref().unwrap().clone();
            let tx_clone = tx.clone();
            let state_clone = app_state.playback_state.clone();
            drop(state);
            app_state.playback_handle = Some(start_audio_playback(path, tx_clone, state_clone));
            state = app_state.playback_state.lock().unwrap();
        } else {
            state.is_playing = true;
        }
    }
    ui_elements.texture_play = create_play_button_texture(font, texture_creator, state.is_playing)?;
    Ok(())
}

fn handle_randomiser_button(
    app_state: &mut AppState,
    ui_elements: &mut UiElements,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut state = app_state.visualisation_state.lock().unwrap();
    update_visualization_bar_colors(&mut state, &mut ui_elements.red_slider, &mut ui_elements.green_slider, &mut ui_elements.blue_slider)?;
    Ok(())
}

fn handle_periodic_randomiser_button(
    app_state: &mut AppState,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut state = app_state.visualisation_state.lock().unwrap();
    let is_auto_randomising = state.get_is_auto_randomising();
    state.set_is_auto_randomising(!is_auto_randomising);
    Ok(())
}

fn handle_slider_click(
    app_state: &mut AppState,
    ui_elements: &mut UiElements,
    x: i32,
    y: i32,
) -> Result<(), Box<dyn std::error::Error>> {
    let click_point = Point::new(x, y);
    
    if ui_elements.red_slider.rect.contains_point(click_point) {
        ui_elements.red_slider.is_dragging = true;
        ui_elements.red_slider.update(x);
        let mut state = app_state.visualisation_state.lock().unwrap();
        state.set_red_slider(ui_elements.red_slider.value);
    } else if ui_elements.green_slider.rect.contains_point(click_point) {
        ui_elements.green_slider.is_dragging = true;
        ui_elements.green_slider.update(x);
        let mut state = app_state.visualisation_state.lock().unwrap();
        state.set_green_slider(ui_elements.green_slider.value);
    } else if ui_elements.blue_slider.rect.contains_point(click_point) {
        ui_elements.blue_slider.is_dragging = true;
        ui_elements.blue_slider.update(x);
        let mut state = app_state.visualisation_state.lock().unwrap();
        state.set_blue_slider(ui_elements.blue_slider.value);
    }
    Ok(())
}

fn update_state<'a, 'b>(
    app_state: &mut AppState,
    ui_elements: &mut UiElements<'a>,
    rx: &mpsc::Receiver<SampleWrapper>,
    font: &'a sdl2::ttf::Font<'a, 'b>,
    texture_creator: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>,
) -> Result<(), Box<dyn std::error::Error>> {
    if app_state.last_second.elapsed().as_secs() >= 1 {
        let state = app_state.playback_state.lock().unwrap();
        if state.is_playing {
            {
                let mut visualisation_state = app_state.visualisation_state.lock().unwrap();
                if visualisation_state.get_is_auto_randomising() {
                    update_visualization_bar_colors(&mut visualisation_state, &mut ui_elements.red_slider, &mut ui_elements.green_slider, &mut ui_elements.blue_slider)?;
                }
            }
            ui_elements.song_name_element.move_element(
                ui_elements.song_name_element.rect.x,
                ui_elements.song_name_element.rect.y,
                ui_elements.song_position_texture.query().width as i32 - 1 / 20,
            );
            if ui_elements.song_name_element.get_element_current_position() > 750 {
                ui_elements.song_name_element.move_element(
                    ui_elements.song_name_element.rect.x,
                    ui_elements.song_name_element.rect.y,
                    -1300,
                );
            }
        }
        app_state.last_second = Instant::now();
    }

    if let Ok(sample_wrapper) = rx.try_recv() {
        let spectrum = app_state.analyzer.process(&sample_wrapper.samples);
        let spectrum_len = spectrum.len();

        {
            let state = app_state.playback_state.lock().unwrap();
            let position = state.get_song_position();
            let minutes = position / 60;
            let seconds = position % 60;
            let position_str = format!("{:02}:{:02}", minutes, seconds);
            ui_elements.song_position_texture = create_element_texture(font, texture_creator, &position_str)?;
            ui_elements.song_position_element.update_text_position(
                ui_elements.song_position_texture.query().width,
                ui_elements.song_position_texture.query().height,
            );
            ui_elements.song_duration_texture = update_duration_display(&state, font, texture_creator, &mut ui_elements.song_duration)?;
        }

        for i in 0..16 {
            let start = i * spectrum_len / 16;
            let end = (i + 1) * spectrum_len / 16;
            app_state.frequencies[i] = spectrum[start..end]
                .iter()
                .fold(0.0, |max, &x| f32::max(max, x));
            app_state.frequencies[i] = (app_state.frequencies[i] * 5.0).min(1.0);
        }
    }
    Ok(())
}

fn draw_ui(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    app_state: &AppState,
    ui_elements: &UiElements,
) -> Result<(), Box<dyn std::error::Error>> {
    // Draw buttons
    draw_buttons(canvas, app_state, ui_elements)?;
    
    // Draw sliders
    draw_sliders(canvas, ui_elements)?;
    
    // Draw text elements
    draw_text_elements(canvas, ui_elements)?;
    
    // Draw visualization
    {
        let state = app_state.visualisation_state.lock().unwrap();
        draw_visualization(canvas, &app_state.frequencies, &state)?;
    }
    
    Ok(())
}

fn draw_buttons(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    app_state: &AppState,
    ui_elements: &UiElements,
) -> Result<(), Box<dyn std::error::Error>> {
    // Draw open button
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.fill_rect(ui_elements.open_button.rect)?;
    canvas.set_draw_color(Color::RGB(47, 198, 18));
    canvas.draw_rect(ui_elements.open_button.rect)?;
    canvas.copy(&ui_elements.open_button_texture, None, Some(ui_elements.open_button.text_rect))?;
    
    // Draw play button if file is loaded
    if app_state.file_path.is_some() {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.fill_rect(ui_elements.play_button.rect)?;
        canvas.set_draw_color(Color::RGB(47, 198, 18));
        canvas.draw_rect(ui_elements.play_button.rect)?;
        canvas.copy(&ui_elements.texture_play, None, Some(ui_elements.play_button.text_rect))?;
    }
    
    // Draw randomiser buttons
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.fill_rect(ui_elements.randomiser_button.rect)?;
    canvas.fill_rect(ui_elements.periodic_randomiser_button.rect)?;
    canvas.set_draw_color(Color::RGB(47, 198, 18));
    canvas.draw_rect(ui_elements.randomiser_button.rect)?;
    canvas.draw_rect(ui_elements.periodic_randomiser_button.rect)?;
    canvas.copy(&ui_elements.randomiser_button_texture, None, Some(ui_elements.randomiser_button.text_rect))?;
    canvas.copy(&ui_elements.periodic_randomiser_button_texture, None, Some(ui_elements.periodic_randomiser_button.text_rect))?;
    
    Ok(())
}

fn draw_sliders(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    ui_elements: &UiElements,
) -> Result<(), Box<dyn std::error::Error>> {
    for slider in [&ui_elements.red_slider, &ui_elements.green_slider, &ui_elements.blue_slider].iter() {
        // Draw background
        canvas.set_draw_color(Color::RGB(20, 20, 20));
        canvas.fill_rect(slider.background_rect)?;

        // Draw track
        let mut gradient_rect = slider.rect.clone();
        gradient_rect.set_width(1);
        for x in 0..slider.rect.width() {
            let progress = x as f32 / slider.rect.width() as f32;
            let color_value = (progress * 255.0) as u8;
            match slider.label.as_str() {
                "R" => canvas.set_draw_color(Color::RGB(color_value, 0, 0)),
                "G" => canvas.set_draw_color(Color::RGB(0, color_value, 0)),
                "B" => canvas.set_draw_color(Color::RGB(0, 0, color_value)),
                _ => {}
            }
            gradient_rect.set_x(slider.rect.x + x as i32);
            canvas.fill_rect(gradient_rect)?;
        }
    }
    Ok(())
}

fn draw_text_elements(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    ui_elements: &UiElements,
) -> Result<(), Box<dyn std::error::Error>> {
    canvas.copy(&ui_elements.song_name_texture, None, Some(ui_elements.song_name_element.rect))?;
    canvas.copy(&ui_elements.song_position_texture, None, Some(ui_elements.song_position_element.rect))?;
    canvas.copy(&ui_elements.separator_texture, None, Some(ui_elements.separator_element.rect))?;
    canvas.copy(&ui_elements.song_duration_texture, None, Some(ui_elements.song_duration.rect))?;
    Ok(())
}
