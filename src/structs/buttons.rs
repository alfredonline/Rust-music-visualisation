use sdl2::rect::{Point, Rect};

pub const BUTTON_WIDTH: u32 = 200;
pub const BUTTON_HEIGHT: u32 = 50;

pub struct Button {
    pub rect: Rect,
    pub text: String,
    pub text_rect: Rect,
}

impl Button {
    pub fn new(x: i32, y: i32, width: u32, height: u32, text: &str) -> Self {
        let rect = Rect::new(x, y, width, height);
        // Initialize with zero size - we'll update this when we create the texture
        let text_rect = Rect::new(0, 0, 0, 0);
        
        Button {
            rect,
            text: text.to_string(),
            text_rect,
            
        }
    }

    pub fn update_text_position(&mut self, text_width: u32, text_height: u32) {
        let text_x = self.rect.x + (self.rect.width() as i32 - text_width as i32) / 2;
        let text_y = self.rect.y + (self.rect.height() as i32 - text_height as i32) / 2;
        self.text_rect = Rect::new(text_x, text_y, text_width, text_height);
    }

    pub fn is_clicked(&self, point: Point) -> bool {
        self.rect.contains_point(point)
    }
}
