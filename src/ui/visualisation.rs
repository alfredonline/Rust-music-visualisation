use rand::prelude::*;
use sdl2::rect::Rect;
use crate::structs::colour_slider::ColourSlider;
use crate::structs::visualisation_state::VisualisationState;

pub fn draw_visualization(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    frequencies: &[f32],
    state: &VisualisationState,
) -> Result<(), String> {
    let viz_width = 1000;
    let viz_height = 300;
    let viz_x = 140;
    let viz_y = 720 - viz_height - 50;

    let bar_count = 16;
    let bar_width = (viz_width / bar_count) - 2;
    let max_bar_height = viz_height;

    for (i, &amplitude) in frequencies.iter().enumerate() {
        let bar_height = (amplitude * max_bar_height as f32) as i32;
        let x = viz_x + (i as i32 * (bar_width + 2));
        let y = viz_y + (viz_height - bar_height);

        let bar_rect = Rect::new(x, y, bar_width as u32, bar_height as u32);
        canvas.set_draw_color(state.get_selected_color());
        canvas.fill_rect(bar_rect)?;
    }

    Ok(())
}

pub fn update_visualization_bar_colors(state: &mut VisualisationState, 
    red_slider: &mut ColourSlider,
    green_slider: &mut ColourSlider,
    blue_slider: &mut ColourSlider,
) -> Result<(), String> {
    let mut rng = rand::rng();
    let mut nums: Vec<i32> = (0..255).collect();
    nums.shuffle(&mut rng);

    state.set_red_slider(nums[0] as u8);
    state.set_green_slider(nums[1] as u8);
    state.set_blue_slider(nums[2] as u8);

    red_slider.value = nums[0] as u8;
    green_slider.value = nums[1] as u8;
    blue_slider.value = nums[2] as u8;

    Ok(())
}
