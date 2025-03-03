use sdl2::pixels::Color;

pub struct VisualisationState {
    pub red_slider: u8,
    pub green_slider: u8,
    pub blue_slider: u8,
    pub is_auto_randomising: bool,
}

impl VisualisationState {

    pub fn new() -> Self {
        Self {
            red_slider: 0,
            green_slider: 0,
            blue_slider: 0,
            is_auto_randomising: false,
        }
    }

    pub fn set_red_slider(&mut self, value: u8) {
        self.red_slider = value;
    }

    pub fn set_green_slider(&mut self, value: u8) {
        self.green_slider = value;
    }

    pub fn set_blue_slider(&mut self, value: u8) {
        self.blue_slider = value;
    }

    pub fn set_initial_color(&mut self) {
        self.red_slider = 47;
        self.green_slider = 198;
        self.blue_slider = 18;
    }

    pub fn get_selected_color(&self) -> Color {
        Color::RGB(self.red_slider, self.green_slider, self.blue_slider)
    }

    pub fn set_is_auto_randomising(&mut self, value: bool) {
        self.is_auto_randomising = value;
    }

    pub fn get_is_auto_randomising(&self) -> bool {
        self.is_auto_randomising
    }
}
