use cairo::{Context, Matrix, Rectangle};

use crate::{
    color::{get_number_color, rgb, rgba, SetColor},
    sudoku::{Block, Cell, Sudoku},
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

        for (i, block) in self.iter().enumerate() {
            let x = (i % 3) as f64;
            let y = (i / 3) as f64;

            block.draw(
                ctx,
                Rectangle {
                    x: x / 3.0 + (border_width * x) / 2.0,
                    y: y / 3.0 + (border_width * y) / 2.0,
                    width: 1.0 / 3.0 - (border_width * 2.0) / 2.0,
                    height: 1.0 / 3.0 - (border_width * 2.0) / 2.0,
                },
            );
        }
    }
}

impl Drawable for Block {
    fn draw_impl(&self, ctx: &Context) {
        let border_width = 0.01;

        for (i, cell) in self.iter().enumerate() {
            let x = (i % 3) as f64;
            let y = (i / 3) as f64;

            cell.draw(
                ctx,
                Rectangle {
                    x: x / 3.0 + (border_width * x) / 2.0,
                    y: y / 3.0 + (border_width * y) / 2.0,
                    width: 1.0 / 3.0 - (border_width * 2.0) / 2.0,
                    height: 1.0 / 3.0 - (border_width * 2.0) / 2.0,
                },
            );
        }
    }
}

impl Drawable for Cell {
    fn draw_impl(&self, ctx: &Context) {
        ctx.set_color(rgba(0x80808080));
        ctx.rectangle(0.0, 0.0, 1.0, 1.0);
        ctx.fill();

        if let Some(digit) = self.digit {
            ctx.set_color(get_number_color(digit));
            ctx.rectangle(0.0, 0.0, 1.0, 1.0);
            ctx.fill();

            ctx.set_font_size(0.8);
            ctx.set_color(rgb(0x000000));

            let digit = digit.to_string();
            let text_extents = ctx.text_extents(&digit);

            let x_pos = 0.5 - text_extents.width / 2.0 - text_extents.x_bearing;
            let y_pos = 0.5 - text_extents.height / 2.0 - text_extents.y_bearing;

            ctx.move_to(x_pos, y_pos);
            ctx.show_text(&digit);
        }
    }
}
