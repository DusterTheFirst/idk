use std::{env::args, fs, rc::Rc, str::FromStr};

use cairo::{Context, FontSlant, FontWeight};
use derive_more::{AsRef, Deref, DerefMut};
use gio::prelude::*;
use gtk::prelude::*;
use gtk::{AspectFrame, DrawingArea};
use thiserror::Error;

pub fn rgb(hex: u32) -> RGB {
    assert!(hex <= 0xffffff);

    let red = ((hex & 0xff0000) >> 16) as f64 / 255f64;
    let green = ((hex & 0x00ff00) >> 8) as f64 / 255f64;
    let blue = (hex & 0x0000ff) as f64 / 255f64;

    (red, green, blue)
}

pub fn rgba(hex: u32) -> RGBA {
    let alpha = (hex & 0x000000ff) as f64 / 255f64;
    let (red, green, blue) = rgb((hex & 0xffffff00) >> 8);

    (red, green, blue, alpha)
}

trait SetColor<T> {
    fn set_color(&self, color: T);
}

type RGB = (f64, f64, f64);
impl SetColor<RGB> for Context {
    fn set_color(&self, (red, green, blue): RGB) {
        self.set_source_rgb(red, green, blue);
    }
}

type RGBA = (f64, f64, f64, f64);
impl SetColor<RGBA> for Context {
    fn set_color(&self, (red, green, blue, alpha): RGBA) {
        self.set_source_rgba(red, green, blue, alpha);
    }
}

fn get_number_color(number: u8) -> RGB {
    match number {
        1 | 9 => rgb(0xb9e6f0),
        2 | 8 => rgb(0x94ebae),
        3 | 7 => rgb(0xdeb6de),
        4 | 6 => rgb(0xfff975),
        5 => rgb(0xf9b0b4),
        _ => unreachable!("Only numbers 1-9 are used in sudoku"),
    }
}

#[derive(Default, Debug, Deref, DerefMut, AsRef, Clone)]
struct Sudoku([Block; 9]);

#[derive(Error, Debug, Clone, Copy)]
enum SudokuParseError {
    #[error("the sudoku board was of invalid size. expected length of 81, found {0}")]
    TooShort(usize),
    #[error("encountered an invalid character: {0}")]
    InvalidChar(char),
}

impl FromStr for Sudoku {
    type Err = SudokuParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.replace("\n", "");

        if s.len() != 81 {
            return Err(SudokuParseError::TooShort(s.len()));
        }

        let mut sudoku = Sudoku::default();

        for block in 0..9 {
            for cell in 0..9 {
                let x = (block % 3) * 3 + cell % 3;
                let y = (block / 3) * 3 + cell / 3;

                let char = s.chars().nth(x + y * 9).unwrap();

                sudoku[block][cell].digit = if char == '-' {
                    None
                } else {
                    Some(
                        char.to_digit(10)
                            .ok_or_else(|| SudokuParseError::InvalidChar(char))?
                            as u8,
                    )
                }
            }
        }

        Ok(sudoku)
    }
}

#[derive(Default, Debug, Deref, DerefMut, AsRef, Clone)]
struct Block([Cell; 9]);

#[derive(Default, Debug, Clone)]
struct Cell {
    pub digit: Option<u8>,
    pub pencil: Vec<u8>,
}

fn build_ui(application: &gtk::Application, sudoku: Rc<Sudoku>) {
    let window = gtk::ApplicationWindow::new(application);
    let drawing_area = DrawingArea::new();

    drawing_area.connect_draw(move |a, cr| {
        cr.scale(
            a.get_allocated_width() as f64,
            a.get_allocated_height() as f64,
        );

        cr.set_color(rgb(0xffffff));

        for i in 1..9 {
            cr.set_line_width(if i % 3 == 0 { 0.02 } else { 0.005 });

            cr.move_to(i as f64 / 9.0, 0.0);
            cr.line_to(i as f64 / 9.0, 1.0);

            cr.move_to(0.0, i as f64 / 9.0);
            cr.line_to(1.0, i as f64 / 9.0);

            cr.stroke();
        }

        for (bx, by, block) in sudoku.iter().enumerate().map(|(i, x)| (i % 3, i / 3, x)) {
            for (cx, cy, cell) in block.iter().enumerate().map(|(i, x)| (i % 3, i / 3, x)) {
                let x = cx + bx * 3;
                let y = cy + by * 3;

                if let Some(digit) = cell.digit {
                    cr.set_font_size(0.05);
                    cr.set_color(get_number_color(digit));

                    let digit = digit.to_string();
                    let text_extents = cr.text_extents(&digit);

                    let x_pos = x as f64 / 9.0 + 1.0 / 18.0 - text_extents.width / 2.0;
                    let y_pos = y as f64 / 9.0 + 1.0 / 18.0 + text_extents.height / 2.0;

                    cr.move_to(x_pos, y_pos);
                    cr.show_text(&digit);
                }
            }
        }

        Inhibit(false)
    });
    drawing_area.set_size_request(500, 500);

    window.set_default_size(500, 500);

    let aspect_frame = AspectFrame::new(None, 0.5, 0.5, 1.0, false);
    aspect_frame.add(&drawing_area);

    window.add(&aspect_frame);
    window.show_all();
}

fn main() {
    let application = gtk::Application::new(Some("com.dusterthefirst.sudoku"), Default::default())
        .expect("Initialization failed...");

    let sudoku = fs::read_to_string("sudoku.txt").unwrap();
    let sudoku = Rc::new(sudoku.parse().unwrap_or_else(|e| panic!("{}", e)));

    application.connect_activate(move |app| {
        build_ui(app, Rc::clone(&sudoku));
    });

    application.run(&args().collect::<Vec<_>>());
}
