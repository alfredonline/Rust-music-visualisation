use sdl2::rect::Rect;

pub struct Element {
    pub rect: Rect,
    pub text: String
}

impl Element {
    pub fn new(x: i32, y: i32, width: u32, height: u32, text: &str) -> Self {
        let rect = Rect::new(x, y, width, height);
        
        Element {
            rect,
            text: text.to_string(),
        }
    }

    pub fn update_text_position(&mut self, text_width: u32, text_height: u32) {
        let text_x = self.rect.x + (self.rect.width() as i32 - text_width as i32) / 2;
        let text_y = self.rect.y + (self.rect.height() as i32 - text_height as i32) / 2;
        self.rect = Rect::new(text_x, text_y, text_width, text_height);
    }

    pub fn move_element(&mut self, x: i32, y: i32, shift_x: i32) {
        self.rect = Rect::new(x + shift_x, y, self.rect.width(), self.rect.height());
    }

    pub fn get_element_current_position(&self) -> i32 {
        self.rect.x
    }

}
