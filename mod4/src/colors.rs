pub fn red(s: &str) -> String {
    format!("\x1b[31m{}\x1b[0m", s)
}

pub fn green(s: &str) -> String {
    format!("\x1b[32m{}\x1b[0m", s)
}

pub fn blue(s: &str) -> String {
    format!("\x1b[34m{}\x1b[0m", s)
}

pub fn bold(s: &str) -> String {
    format!("\x1b[1m{}\x1b[0m", s)
}

pub enum Color {
    Red,
    Green,
    Blue,
    Bold,
}

pub struct ColorString {
    pub color: Color,
    pub string: String,
    pub colorized: String,
}

impl ColorString {
    pub fn new(color: Color, string: &str) -> ColorString {
        let colorized = match color {
            Color::Red => red(string),
            Color::Green => green(string),
            Color::Blue => blue(string),
            Color::Bold => bold(string),
        };
        ColorString {
            color,
            string: string.to_string(),
            colorized,
        }
    }
}

pub fn run() {
    let s = ColorString::new(Color::Red, "Hello, Red!");
    println!("{}", s.colorized);
}
