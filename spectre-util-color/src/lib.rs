use std::fmt;

pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

impl Color {
    pub fn new(red: u8, green: u8, blue: u8, alpha: u8) -> Color {
        Color {
            red,
            green,
            blue,
            alpha,
        }
    }
}

impl From<u32> for Color {
    fn from(color: u32) -> Color {
        Color {
            alpha: ((color >> 24) & 0xFF) as u8,
            red: ((color >> 16) & 0xFF) as u8,
            green: ((color >> 8) & 0xFF) as u8,
            blue: (color & 0xFF) as u8,
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Color {{ red: {}, green: {}, blue: {}, alpha: {} }}",
            self.red, self.green, self.blue, self.alpha
        )
    }
}
