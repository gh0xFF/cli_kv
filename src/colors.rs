#[repr(u8)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Default,
}

impl Color {
    pub fn new(&self, txt: &str) -> String {
        match self {
            Color::Black => self.cfmt("\x1b[30m", &txt),
            Color::Red => self.cfmt("\x1b[31m", &txt),
            Color::Green => self.cfmt("\x1b[32m", &txt),
            Color::Yellow => self.cfmt("\x1b[33m", &txt),
            Color::Blue => self.cfmt("\x1b[34m", &txt),
            Color::Magenta => self.cfmt("\x1b[35m", &txt),
            Color::Cyan => self.cfmt("\x1b[36m", &txt),
            Color::White => self.cfmt("\x1b[37m", &txt),
            Color::Default => self.cfmt("\x1b[39m", &txt),
        }
    }

    fn cfmt(&self, color: &str, txt: &str) -> String {
        return format!("{}{}\x1b[0m", color.to_string(), txt.to_string());
    }
}
