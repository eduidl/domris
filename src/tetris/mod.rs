use wasm_bindgen::prelude::*;
use std::collections::VecDeque;

use self::tetromino::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

pub const W: usize = 12;
pub const H: usize = 22;

pub mod tetromino;

#[derive(PartialEq, Copy, Clone)]
pub enum Cell {
    Shape(Shape),
    Empty,
    Wall,
}

#[wasm_bindgen]
#[derive(PartialEq, Copy, Clone)]
pub enum Control {
    MoveLeft,
    MoveRight,
    MoveDown,
    MoveBottom,
    RotateLeft,
    RotateRight,
}

type Board = [[Cell; W]; H];

#[wasm_bindgen]
pub struct Tetris {
    board: Board,
    current_mino: Tetromino,
    drop_interval: u32,
    interval_count: u32,
    control_queue: VecDeque<Control>
}

#[wasm_bindgen]
impl Tetris {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Tetris {
        Tetris {
            board: Tetris::board_initialize(),
            current_mino: Tetromino::random(),
            drop_interval: 1000,
            interval_count: 0,
            control_queue: VecDeque::new(), 
        }
    }

    pub fn update(&mut self, interval: u32) {
        let mut control_done = false;
        while let Some(control) = self.control_queue.pop_front() {
            if self.try_control(control) { control_done = true; }
        }
        if control_done { 
            return; 
        }
        self.interval_count += interval;
        if self.interval_count >= self.drop_interval { 
            if self.try_drop() {
                self.interval_count -= self.drop_interval;
            } else {
                self.next_tetromino();
                self.interval_count = 0;
            }
        }
    }

    pub fn enqueue_control(&mut self, control: Control) {
        self.control_queue.push_back(control);
    }

    pub(super) fn board(&self) -> &Board {
        &self.board
    }
    
    pub(super) fn current_mino(&self) -> &Tetromino {
        &self.current_mino
    }

    fn delete_line(&mut self) {
        for y in 0..(self.board.len() - 1) {
            if self.board[y].iter().skip(1).take(W - 2)
                .all(|cell| *cell != Cell::Empty) {
                for yy in (1..=y).rev() {
                    for x in 1..(W-1) {
                        self.board[yy][x] = self.board[yy - 1][x];
                    }
                }
                for x in 1..(W-1) {
                    self.board[0][x] = Cell::Empty;
                }
            }
        }
    }

    fn try_drop(&mut self) -> bool {
        self.current_mino.move_y(1);
        if self.overwrapping() {
            self.current_mino.move_y(-1);
            return false;
        }
        true
    }

    fn move_bottom(&mut self) -> bool {
        while self.try_drop() {}
        false
    }

    fn try_control(&mut self, control: Control) -> bool {
        match control {
            Control::MoveLeft => self.try_move_x(-1),
            Control::MoveRight => self.try_move_x(1),
            Control::MoveDown => self.try_drop(),
            Control::MoveBottom => self.move_bottom(),
            Control::RotateLeft => self.try_rotate(3),
            Control::RotateRight => self.try_rotate(1),
        }
    }

    fn try_move_x(&mut self, diff: i8) -> bool {
        self.current_mino.move_x(diff);
        if self.overwrapping() {
            self.current_mino.move_x(-diff);
            return false
        }
        true
    }

    fn try_rotate(&mut self, diff: usize) -> bool {
        self.current_mino.rotate(diff);
        if self.overwrapping() {
            self.current_mino.rotate(4 - diff);
            return false
        }
        true
    }
    
    fn overwrapping(&self) -> bool {
        self.current_mino.coordinates().iter().any(|(x, y)|
            self.board[*y as usize][*x as usize] != Cell::Empty
        )
    }

    // 1. 落ちたテトロミノを固定
    // 2. 新しいテトロミノをセット
    // 3. 揃ったラインを消す
    fn next_tetromino(&mut self) {
        let mut tmp_mino = Tetromino::random();
        std::mem::swap(&mut tmp_mino, &mut self.current_mino);
        let shape = tmp_mino.shape();
        for (x, y) in tmp_mino.coordinates() {
            self.board[y as usize][x as usize] = Cell::Shape(shape);
        }
        self.delete_line();
    }

    fn board_initialize() -> Board {
        let mut board: Board = [[Cell::Empty; W]; H];
        for y in 0..H {
            for x in 0..W {
                if x == 0 || x == W - 1 || y == H - 1 {
                    board[y][x] = Cell::Wall;
                }
            }
        }
        board
    }
}
