extern crate rand;
extern crate wasm_bindgen;
extern crate web_sys;

use wasm_bindgen::prelude::*;

mod domris;
use domris::{Cell, Domris};
use domris::tetromino::{Tetromino, Shape};

type Context2d = web_sys::CanvasRenderingContext2d;

const SQUARE_PX: u16 = 25;

fn shape_to_color(shape: Shape) -> String {
    match shape {
        Shape::I => "lightblue".to_string(),
        Shape::O => "gold".to_string(),
        Shape::T => "purple".to_string(),
        Shape::J => "blue".to_string(),
        Shape::L => "orange".to_string(),
        Shape::S => "green".to_string(),
        Shape::Z => "red".to_string(),
    }
}

fn cell_to_color(cell: Cell) -> String {
    match cell {
        Cell::Shape(shape) => shape_to_color(shape),
        Cell::Empty => "gray".to_string(),
        Cell::Wall  => "black".to_string(),
    }
}

fn draw_background_full(game: &Domris, ctx: &Context2d) {
    ctx.set_font(&"bold 25px 'Times New Roman'".to_string());
    for (y, row) in game.board().iter().enumerate() {
        for (x, (cell, num)) in row.iter().enumerate() {
            ctx.set_fill_style(&cell_to_color(*cell).into());
            ctx.fill_rect(
                (x as u16 * SQUARE_PX).into(),
                (y as u16 * SQUARE_PX).into(),
                SQUARE_PX.into(), SQUARE_PX.into());

            ctx.set_fill_style(&"white".to_string().into());
            let text = if let Some(num) = num {
                num.to_string()
            } else if *cell == Cell::Wall { 
                "*".to_string()
            } else {
                continue;
            };
            ctx.fill_text_with_max_width(&text,
                (x as u16 * SQUARE_PX + 5).into(),
                ((y + 1) as u16 * SQUARE_PX - 5).into(),
                SQUARE_PX.into()).unwrap();
        }
    }
}

fn draw_background_partial(game: &Domris, ctx: &Context2d) {
    for (y, row) in game.board().iter().enumerate() {
        for (x, (cell, _)) in row.iter().enumerate() {
            if *cell != Cell::Empty { continue; }
            ctx.set_fill_style(&cell_to_color(*cell).into());
            ctx.fill_rect(
                (x as u16 * SQUARE_PX).into(),
                (y as u16 * SQUARE_PX).into(),
                SQUARE_PX.into(), SQUARE_PX.into());
        }
    }
}

fn draw_moving_tetromino(mino: &Tetromino, ctx: &Context2d) {
    for ((x, y), num) in mino.coordinates().iter().zip(mino.numbers()) {
        ctx.set_fill_style(&shape_to_color(mino.shape).into());
        ctx.fill_rect(
            (*x as u16 * SQUARE_PX).into(),
            (*y as u16 * SQUARE_PX).into(),
            SQUARE_PX.into(), SQUARE_PX.into());

        ctx.set_fill_style(&"white".to_string().into());
        ctx.fill_text_with_max_width(
            &*num.to_string(),
            (*x as u16 * SQUARE_PX + 5).into(),
            ((*y + 1) as u16 * SQUARE_PX - 5).into(),
            SQUARE_PX.into()).unwrap();
    }
}

#[wasm_bindgen]
pub fn draw(game: &Domris, ctx: &web_sys::CanvasRenderingContext2d,
            partial_update: bool) {
    if partial_update {
        draw_background_full(game, ctx);
    } else {
        draw_background_partial(game, ctx);
    }

    draw_moving_tetromino(game.current_mino(), ctx);
}
