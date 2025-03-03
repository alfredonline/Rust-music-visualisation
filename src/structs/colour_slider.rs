use sdl2::rect::Rect;
use sdl2::pixels::Color;

pub struct ColourSlider {
    pub rect: Rect,
    pub value: u8,
    pub is_dragging: bool,
    pub label: String,
    pub color: Color,
    pub background_rect: Rect,
}

impl ColourSlider {
    pub fn new(x: i32, y: i32, width: u32, height: u32, label: &str) -> Self {
        let padding = 10;
        ColourSlider {
            rect: Rect::new(x + padding, y, width - (padding * 2) as u32, height),
            background_rect: Rect::new(
                x, 
                y - padding, 
                width, 
                height + (padding * 2) as u32
            ),
            value: 0,
            is_dragging: false,
            label: label.to_string(),
            color: Color::RGB(0, 0, 0),
        }
    }

    pub fn update(&mut self, mouse_x: i32) {
        if self.is_dragging {
            let relative_x = mouse_x - self.rect.x;
            self.value = ((relative_x as f32 / self.rect.width() as f32) * 255.0)
                .max(0.0)
                .min(255.0) as u8;
            self.color = Color::RGB(self.value, self.value, self.value);
        }
    }
}