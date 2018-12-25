use std::collections::VecDeque;
use rand::prelude::*;
use wasm_bindgen::prelude::*;

use self::tetromino::{Shape, Tetromino};
pub mod tetromino;

pub const W: usize = 12;
pub const H: usize = 22;

const DIRECTIONS: [(i8, i8); 4] = [
    (0, -1), (0, 1),
    (-1, 0), (1, 0)
];

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

type Board = [[(Cell, Option<u8>); W]; H];

#[wasm_bindgen]
pub struct Domris {
    started: bool,
    gameover: bool,
    board: Board,
    max_number: u8,
    current_mino: Tetromino,
    #[wasm_bindgen(readonly)]
    pub point: u32,
    drop_interval: u32,
    interval_count: u32,
    control_queue: VecDeque<Control>,
}

impl Default for Domris {
    fn default() -> Self {
        Self {
            started: false,
            gameover: false,
            board: [[(Cell::Empty, None); W]; H],
            max_number: 1,
            current_mino: Tetromino::random(1),
            point: 0,
            drop_interval: 1000,
            interval_count: 0,
            control_queue: VecDeque::new(), 
        }
    }
}

#[wasm_bindgen]
impl Domris {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self { .. Default::default() }
    }

    pub fn start(&mut self, level: u8) {
        let max_number = match level {
            1 => 1,
            2 => 3,
            3 => 6,
            _ => panic!("level should be in [1, 2, 3].")
        };
        *self = Self {
            started: true,
            board: Self::board_initialize(max_number),
            max_number,
            current_mino: Tetromino::random(max_number),
            .. Default::default()
        };
    }

    pub fn update(&mut self, interval: u32) -> bool {
        if !self.playing() { return false; } 
        while let Some(control) = self.control_queue.pop_front() {
            self.try_control(control);
            if control == Control::MoveBottom {
                self.next_tetromino();
                return true;
            }
        }
        self.interval_count += interval;
        if self.interval_count >= self.drop_interval { 
            if self.try_drop() {
                self.interval_count -= self.drop_interval;
            } else {
                self.next_tetromino();
                return true;
            }
        }
        false
    }

    pub fn playing(&self) -> bool {
        self.started && !self.gameover
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
        for y in 0..(H - 1) {
            if self.board[y].iter().skip(1).take(W - 2)
                .any(|cell| cell.0 == Cell::Empty) { continue; }
            for yy in (1..=y).rev() {
                for x in 1..(W - 1) {
                    self.board[yy][x] = self.board[yy - 1][x];
                }
            }
            for cell in self.board[0].iter_mut().skip(1).take(W - 2) {
                cell.0 = Cell::Empty;
            }
            self.point += 1000;
        }
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
            *y >= 0 && self.board[*y as usize][*x as usize].0 != Cell::Empty
        )
    }

    fn penalty(&mut self) {
        let mut delete_candidates = Vec::new();

        {
            let mino = self.current_mino();
            for ((x, y), ref_num) in mino.coordinates().iter().zip(mino.numbers()) {
                for (dx, dy) in DIRECTIONS.iter() {
                    if y + dy < 0 { continue; }

                    let xx = (x + dx) as usize;
                    let yy = (y + dy) as usize;
                    match self.board[yy][xx] {
                        (Cell::Shape(_), Some(num)) => { 
                            if num == *ref_num {
                                return;
                            } else {
                                delete_candidates.push((xx, yy));
                            }
                        },
                        (Cell::Empty, _) => { continue; },
                        (Cell::Wall, Some(num)) => {
                            if num == *ref_num { return; }
                        },
                        (Cell::Wall, None) => { return; },
                        (_, _) => {},
                    } 
                }
            }
        }

        for (x, y) in delete_candidates {
            self.board[y][x] = (Cell::Empty, None);
        }
    }

    fn next_tetromino(&mut self) {
        self.penalty();
        let mut tmp_mino = Tetromino::random(self.max_number);
        std::mem::swap(&mut tmp_mino, &mut self.current_mino);
        let shape = tmp_mino.shape;
        for ((x, y), num) in tmp_mino.coordinates().iter().zip(tmp_mino.numbers()) {
            if *y < 0 {
                self.gameover = true;
                return;
            }
            self.board[*y as usize][*x as usize] = (Cell::Shape(shape), Some(*num));
        }
        if self.current_mino.coordinates().iter().any(|(x, y)|
            self.board[*y as usize][*x as usize].0 != Cell::Empty) {
            self.gameover = true;
            return;
        }
        self.delete_line();
        self.control_queue.clear();
        self.interval_count = 0;
    }

    fn board_initialize(max_number: u8) -> Board {
        let mut board: Board = [[(Cell::Empty, None); W]; H];
        for cell in board[H - 1].iter_mut() {
            *cell = (Cell::Wall, None);
        }

        let mut rng = thread_rng();
        let range = rand::distributions::Uniform::new(0, max_number + 1); 
        for line in board.iter_mut().take(H - 1) {
            line[0]     = (Cell::Wall, Some(range.sample(&mut rng)));
            line[W - 1] = (Cell::Wall, Some(range.sample(&mut rng)));
        }
        board
    }
}
