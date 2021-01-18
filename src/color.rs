use cairo::Context;

pub fn rgb(hex: u32) -> RGB {
    assert!(hex <= 0xffffff);

    let red = ((hex & 0xff0000) >> 16) as f64 / 255f64;
    let green = ((hex & 0x00ff00) >> 8) as f64 / 255f64;
    let blue = (hex & 0x0000ff) as f64 / 255f64;

    RGB { red, green, blue }
}

pub fn rgba(hex: u32) -> RGBA {
    let alpha = (hex & 0x000000ff) as f64 / 255f64;
    let RGB { red, green, blue } = rgb((hex & 0xffffff00) >> 8);

    RGBA {
        red,
        green,
        blue,
        alpha,
    }
}

pub trait SetColor<T> {
    fn set_color(&self, color: T);
}

pub struct RGB {
    red: f64,
    green: f64,
    blue: f64,
}

impl SetColor<RGB> for Context {
    fn set_color(&self, RGB { red, green, blue }: RGB) {
        self.set_source_rgb(red, green, blue);
    }
}

pub struct RGBA {
    red: f64,
    green: f64,
    blue: f64,
    alpha: f64,
}
impl SetColor<RGBA> for Context {
    fn set_color(
        &self,
        RGBA {
            red,
            green,
            blue,
            alpha,
        }: RGBA,
    ) {
        self.set_source_rgba(red, green, blue, alpha);
    }
}

pub fn get_number_color(number: u8) -> RGB {
    match number {
        1 | 9 => rgb(0xb9e6f0),
        2 | 8 => rgb(0x94ebae),
        3 | 7 => rgb(0xdeb6de),
        4 | 6 => rgb(0xfff975),
        5 => rgb(0xf9b0b4),
        _ => unreachable!("Only numbers 1-9 are used in sudoku"),
    }
}
