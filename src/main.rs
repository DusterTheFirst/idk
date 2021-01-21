use std::{env::args, fs, rc::Rc, sync::RwLock};

use cairo::Rectangle;
use draw::Drawable;
use gio::prelude::*;
use glib::clone;
use gtk::{prelude::*, Align, AspectFrame, Box, Button, ButtonBox, DrawingArea, Orientation};
use sudoku::{CellContents, Digit, Relation, Sudoku};

mod color;
mod draw;
mod sudoku;

fn build_ui(application: &gtk::Application, sudoku: Rc<RwLock<Sudoku>>) {
    let window = gtk::ApplicationWindow::new(application);
    let box_container = Box::new(Orientation::Vertical, 5);

    let aspect_frame = AspectFrame::new(None, 0.5, 0.5, 1.0, false);
    box_container.add(&aspect_frame);
    box_container.set_child_expand(&aspect_frame, true);

    let buttons = ButtonBox::new(Orientation::Horizontal);
    buttons.set_spacing(5);
    buttons.set_margin_bottom(5);
    buttons.set_margin_start(5);
    buttons.set_halign(Align::Start);
    box_container.add(&buttons);
    box_container.set_child_expand(&buttons, false);

    let drawing_area = DrawingArea::new();
    aspect_frame.add(&drawing_area);

    let start_button = Button::new();
    start_button.set_label("Solve");
    buttons.add(&start_button);

    drawing_area.connect_draw(clone!(@strong sudoku => move |a, cr| {
        cr.scale(
            a.get_allocated_width() as f64,
            a.get_allocated_height() as f64,
        );

        sudoku.read().unwrap().draw(
            cr,
            Rectangle {
                x: 0.0,
                y: 0.0,
                width: 1.0,
                height: 1.0,
            },
        );

        Inhibit(false)
    }));
    drawing_area.set_size_request(500, 500);

    start_button.connect_button_press_event(
        clone!(@strong sudoku, @strong drawing_area => move |_, _| {
            let mut sudoku = sudoku.write().unwrap();


            // Add all pencils
            for x in 0..9 {
                for y in 0..9 {
                    if let CellContents::Pencil(set) = &mut sudoku[(x, y)] {
                        for digit in Digit::iterator() {
                            set.insert(digit);
                        }
                    }
                }
            }

            drawing_area.queue_draw();

            // Filter pencils
            for x in 0..9 {
                for y in 0..9 {
                    if let CellContents::Pencil(set) = &sudoku[(x, y)] {
                        let mut set = set.clone();

                        for neighbor in sudoku.all_neighbors((x, y)) {
                            if let CellContents::Given(digit) = neighbor {
                                set.remove(&digit);
                            }
                        }

                        sudoku[(x, y)] = CellContents::Pencil(set);
                    }
                }
            }

            drawing_area.queue_draw();

            Inhibit(false)
        }),
    );

    // window.connect_button_press_event();
    // window.connect_motion_notify_event(clone!(@strong sudoku, @strong drawing_area => move |_, motion| {
    //     sudoku.write().unwrap()[4][4].digit = Some((motion.get_position().0 % 9.0 + 1.0) as u8);
    //     drawing_area.queue_draw();

    //     Inhibit(false)
    // }));
    // window.add_events(EventMask::POINTER_MOTION_MASK);

    window.set_default_size(500, 500);

    window.add(&box_container);
    window.show_all();

    window.connect_delete_event(clone!(@strong application => move |_, _| {
        application.quit();

        Inhibit(false)
    }));
}

fn main() {
    let application = gtk::Application::new(Some("com.dusterthefirst.sudoku"), Default::default())
        .expect("Initialization failed...");

    let sudoku = fs::read_to_string(args().nth(1).unwrap_or_else(|| "sudoku.txt".into())).unwrap();
    let sudoku = Rc::new(RwLock::new(
        sudoku.parse().unwrap_or_else(|e| panic!("{}", e)),
    ));

    application.connect_activate(move |app| {
        build_ui(app, Rc::clone(&sudoku));
    });

    application.run(&[]);
}
