use tetromino::*;

pub const W: usize = 12;
pub const H: usize = 22;

#[derive(Copy, Clone)]
pub enum Cell {
    Shape(Shape),
    Empty,
    Wall,
}

type Board = [[Cell; W]; H];

pub struct Tetris {
    board: Board,
    current_tetromino: Option<Tetromino>,
    score: u16,
    finished: bool,
}

impl Tetris {
    pub fn new() -> Tetris {
        Tetris {
            board: Tetris::board_initialize(),
            current_tetromino: None,
            score: 0,
            finished: false,
        }
    }

    pub fn board(&self) -> &Board {
        &self.board
    }
    
    pub fn current_tetromino(&self) -> &Option<Tetromino> {
        &self.current_tetromino
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
