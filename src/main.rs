use std::{env::args, fs, rc::Rc};

use cairo::Rectangle;
use draw::Drawable;
use gio::prelude::*;
use gtk::prelude::*;
use gtk::{AspectFrame, DrawingArea};
use sudoku::Sudoku;

mod color;
mod draw;
mod sudoku;

fn build_ui(application: &gtk::Application, sudoku: Rc<Sudoku>) {
    let window = gtk::ApplicationWindow::new(application);
    let drawing_area = DrawingArea::new();

    drawing_area.connect_draw(move |a, cr| {
        cr.scale(
            a.get_allocated_width() as f64,
            a.get_allocated_height() as f64,
        );

        sudoku.draw(
            cr,
            Rectangle {
                x: 0.0,
                y: 0.0,
                width: 1.0,
                height: 1.0,
            },
        );

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
