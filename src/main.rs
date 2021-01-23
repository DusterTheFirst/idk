use std::{collections::{BTreeSet, HashSet}, env::args, fs, sync::{Arc, RwLock}, thread, time::Duration};

use cairo::Rectangle;
use draw::Drawable;
use gio::prelude::*;
use glib::clone;
use gtk::{prelude::*, Align, AspectFrame, Box, Button, ButtonBox, DrawingArea, Orientation};
use sudoku::{CellValue, Sudoku};

mod color;
mod draw;
mod sudoku;

fn build_ui(application: &gtk::Application, sudoku: Arc<RwLock<Sudoku>>) {
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

    let (tx, rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

    start_button.connect_button_press_event(
        clone!(@strong sudoku => move |_, _| {
            thread::spawn(clone!(@strong sudoku, @strong tx => move || {
                let mut moves = Vec::new();

                'outer: loop {
                    thread::sleep(Duration::from_millis(50));

                    let mut sudoku_lock = sudoku.write().unwrap();

                    let smallest = sudoku_lock.all().filter_map(|(pos, cell)| if let CellValue::Unknown(options) = cell {
                        Some((pos, options))
                    } else {
                        None
                    }).min_by_key(|(_, marks)| marks.len()).map(|(pos, marks)| (pos, marks.iter().next().copied()));

                    if let Some((pos, mark)) = smallest {
                        if let Some(digit) = mark {
                            moves.push((pos, {
                                let mut set = BTreeSet::new();
                                set.insert(digit);
                                set
                            }));
                            sudoku_lock.set(pos, Some(digit));
                        } else {
                            eprintln!("Invalid board, ran into 0 pencil marks at {:?}", pos);
                            eprintln!("Backtracking from {:?}", moves.last());

                            drop(sudoku_lock); // Clear other lock
                            loop {
                                thread::sleep(Duration::from_millis(50));
                                let mut sudoku_lock = sudoku.write().unwrap();

                                if let Some((back_move, mut back_digits)) = moves.pop() {
                                    let possibilities = sudoku_lock.possibilities(back_move);

                                    if let Some(other_possibility) = possibilities.difference(&back_digits).next().map(|x| *x) {
                                        sudoku_lock.set(back_move, Some(other_possibility));

                                        back_digits.insert(other_possibility);
                                        moves.push((back_move, back_digits));
                                        break;
                                    } else {
                                        sudoku_lock.set(back_move, None);
                                    }
                                } else {
                                    eprintln!("Backtracked to the start");
                                    
                                    break 'outer;
                                }

                                tx.send(()).expect("Could not poll refresh");
                            }
                        }
                    } else {
                        eprintln!("Solved?");

                        break;
                    }

                    tx.send(()).expect("Could not poll refresh");
                }
                
                tx.send(()).expect("Could not poll refresh");
            }));


            Inhibit(false)
        }),
    );

    rx.attach(
        None,
        clone!(@strong drawing_area => move |_: ()| {
            drawing_area.queue_draw();

            glib::Continue(true)
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
    let sudoku = Arc::new(RwLock::new(
        sudoku.parse().unwrap_or_else(|e| panic!("{}", e)),
    ));

    application.connect_activate(move |app| {
        build_ui(app, Arc::clone(&sudoku));
    });

    application.run(&[]);
}
