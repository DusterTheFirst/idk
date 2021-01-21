use cairo::{Context, Matrix, Rectangle};

use crate::{
    color::{get_digit_color, rgb, rgba, SetColor},
    sudoku::{CellContents, Sudoku},
};

pub trait Drawable {
    fn draw(&self, ctx: &Context, bounds: Rectangle) {
        ctx.save();

        let matrix = ctx.get_matrix();

        ctx.set_matrix(Matrix {
            x0: matrix.x0 + bounds.x * matrix.xx,
            y0: matrix.y0 + bounds.y * matrix.yy,
            xx: bounds.width * matrix.xx,
            yy: bounds.height * matrix.yy,
            xy: 0.0,
            yx: 0.0,
        });

        self.draw_impl(ctx);

        ctx.restore();
    }

    fn draw_impl(&self, ctx: &Context);
}

impl Drawable for Sudoku {
    fn draw_impl(&self, ctx: &Context) {
        let border_width = 0.02;

        for i in 0..9 {
            let x = i % 3;
            let y = i / 3;

            Block { sudoku: self, x, y }.draw(
                ctx,
                Rectangle {
                    x: x as f64 / 3.0 + (border_width * x as f64) / 2.0,
                    y: y as f64 / 3.0 + (border_width * y as f64) / 2.0,
                    width: 1.0 / 3.0 - (border_width * 2.0) / 2.0,
                    height: 1.0 / 3.0 - (border_width * 2.0) / 2.0,
                },
            );
        }
    }
}

struct Block<'s> {
    sudoku: &'s Sudoku,
    x: usize,
    y: usize,
}
impl<'s> Drawable for Block<'s> {
    fn draw_impl(&self, ctx: &Context) {
        let border_width = 0.01;

        for i in 0..9 {
            let x = i % 3;
            let y = i / 3;

            let global_x = x + self.x * 3;
            let global_y = y + self.y * 3;

            Cell {
                contents: &self.sudoku[(global_x, global_y)],
                sudoku: self.sudoku,
                x: global_x,
                y: global_y,
            }
            .draw(
                ctx,
                Rectangle {
                    x: x as f64 / 3.0 + (border_width * x as f64) / 2.0,
                    y: y as f64 / 3.0 + (border_width * y as f64) / 2.0,
                    width: 1.0 / 3.0 - (border_width * 2.0) / 2.0,
                    height: 1.0 / 3.0 - (border_width * 2.0) / 2.0,
                },
            );
        }

        // match self.sudoku.block_status((self.x, self.y)) {
        //     SolveStatus::Unsolved => {}
        //     SolveStatus::Solved => {
        //         ctx.set_color(rgba(0x00ff0050));
        //         ctx.rectangle(0.0, 0.0, 1.0, 1.0);
        //         ctx.fill();
        //     }
        //     SolveStatus::Invalid => {
        //         ctx.set_color(rgba(0xff000050));
        //         ctx.rectangle(0.0, 0.0, 1.0, 1.0);
        //         ctx.fill();
        //     }
        // }
    }
}

#[derive(Debug, Clone)]
struct Cell<'s> {
    sudoku: &'s Sudoku,
    contents: &'s CellContents,
    x: usize,
    y: usize,
}
impl Drawable for Cell<'_> {
    fn draw_impl(&self, ctx: &Context) {
        ctx.set_color(rgba(0x80808080));
        ctx.rectangle(0.0, 0.0, 1.0, 1.0);
        ctx.fill();

        match self.contents {
            CellContents::Given(digit) => {
                let digit = *digit;

                ctx.set_color(get_digit_color(digit));
                ctx.rectangle(0.0, 0.0, 1.0, 1.0);
                ctx.fill();

                ctx.set_font_size(0.8);
                ctx.set_color(rgb(match self.sudoku.cell_status((self.x, self.y)) {
                    crate::sudoku::SolveStatus::Unsolved => 0x000000,
                    crate::sudoku::SolveStatus::Solved => 0x00ff00,
                    crate::sudoku::SolveStatus::Invalid => 0xff0000,
                }));

                let digit = u8::from(digit).to_string();
                let text_extents = ctx.text_extents(&digit);

                let x_pos = 0.5 - text_extents.width / 2.0 - text_extents.x_bearing;
                let y_pos = 0.5 - text_extents.height / 2.0 - text_extents.y_bearing;

                ctx.move_to(x_pos, y_pos);
                ctx.show_text(&digit);
            }
            CellContents::Digit(digit) => {
                let digit = *digit;

                ctx.set_font_size(0.8);

                ctx.set_color(rgb(match self.sudoku.cell_status((self.x, self.y)) {
                    crate::sudoku::SolveStatus::Unsolved => 0x000000,
                    crate::sudoku::SolveStatus::Solved => 0x00ff00,
                    crate::sudoku::SolveStatus::Invalid => 0xff0000,
                }));

                let digit = u8::from(digit).to_string();
                let text_extents = ctx.text_extents(&digit);

                let x_pos = 0.5 - text_extents.width / 2.0 - text_extents.x_bearing;
                let y_pos = 0.5 - text_extents.height / 2.0 - text_extents.y_bearing;

                ctx.move_to(x_pos, y_pos);
                ctx.show_text(&digit);
            }
            CellContents::Pencil(pencil) => {
                assert!(
                    pencil.len() <= 9,
                    "Too many pencil marks, something went super wrong"
                );

                ctx.set_font_size(0.3);
                ctx.set_color(rgb(0x808080));

                for (i, pencil_mark) in pencil.iter().copied().enumerate() {
                    let y_offset = (i / 3) as f64 * (1.0 / 3.0);
                    let x_offset = (i % 3) as f64 * (1.0 / 3.0);

                    let digit = u8::from(pencil_mark).to_string();
                    let text_extents = ctx.text_extents(&digit);

                    let x_pos =
                        x_offset + (1.0 / 6.0) - text_extents.width / 2.0 - text_extents.x_bearing;
                    let y_pos =
                        y_offset + (1.0 / 6.0) - text_extents.height / 2.0 - text_extents.y_bearing;

                    ctx.move_to(x_pos, y_pos);
                    ctx.show_text(&digit);
                }
            }
        }
    }
}
