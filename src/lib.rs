extern crate rand;
extern crate wasm_bindgen;
extern crate web_sys;

use wasm_bindgen::prelude::*;

mod tetris;
use tetris::*;
use tetris::tetromino::Shape;

const SQUARE_PX: u16 = 30;
const W_PX: u16 = W as u16 * SQUARE_PX;
const H_PX: u16 = H as u16 * SQUARE_PX;

fn shape_to_color(shape: Shape) -> String {
    match shape {
        Shape::I => "lightblue".to_string(),
        Shape::O => "yellow".to_string(),
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

#[wasm_bindgen]
pub fn draw(game: &Tetris, ctx: &web_sys::CanvasRenderingContext2d) {
    // 動いているテトロミノ以外
    for (y, row) in game.board().iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            ctx.set_fill_style(&cell_to_color(*cell).into());
            ctx.fill_rect(
                (x as u16 * SQUARE_PX) as f64,
                (y as u16 * SQUARE_PX) as f64,
                SQUARE_PX.into(), SQUARE_PX.into());
        }
    }

    // 動いているテトロミノ
    let mino = game.current_mino();
    ctx.set_fill_style(&shape_to_color(mino.shape()).into());
    for (x, y) in mino.coordinates() {
        ctx.fill_rect(
            (x as u16 * SQUARE_PX) as f64,
            (y as u16 * SQUARE_PX) as f64,
            SQUARE_PX.into(), SQUARE_PX.into());
    }

    // 罫線
    ctx.set_stroke_style(&"white".into());
    ctx.begin_path();
    for x in (0..=W_PX).step_by(SQUARE_PX as usize) {
        ctx.move_to(x.into(), 0.0);
        ctx.line_to(x.into(), H_PX.into());
    }
    for y in (0..=H_PX).step_by(SQUARE_PX as usize) {
        ctx.move_to(0.0,         y.into());
        ctx.line_to(W_PX.into(), y.into());
    }
    ctx.stroke();
}
