use rand::prelude::*;
use rand::distributions::Uniform;

#[derive(PartialEq, Copy, Clone)]
pub enum Shape {
    I,
    O,
    T,
    J,
    L,
    S,
    Z,
}

const SHAPES: [Shape; 7] = [
    Shape::I,
    Shape::O,
    Shape::T,
    Shape::J,
    Shape::L,
    Shape::S,
    Shape::Z,
];

type Block = [[(i8, i8); 4]; 4];

const BLOCKS: [Block; 7] = [
    [
        [(0, 1), (1, 1), (2, 1), (3, 1)],
        [(2, 0), (2, 1), (2, 2), (2, 3)],
        [(0, 2), (1, 2), (2, 2), (3, 2)],
        [(1, 0), (1, 1), (1, 2), (1, 3)]
    ], [ 
        [(0, 0), (0, 1), (1, 0), (1, 1)],
        [(0, 0), (0, 1), (1, 0), (1, 1)],
        [(0, 0), (0, 1), (1, 0), (1, 1)],
        [(0, 0), (0, 1), (1, 0), (1, 1)]
    ], [
        [(1, 0), (0, 1), (1, 1), (2, 1)],
        [(2, 1), (1, 0), (1, 1), (1, 2)],
        [(1, 2), (2, 1), (1, 1), (0, 1)],
        [(0, 1), (1, 2), (1, 1), (1, 0)]
    ], [
        [(0, 0), (0, 1), (1, 1), (2, 1)],
        [(2, 0), (1, 0), (1, 1), (1, 2)],
        [(2, 2), (2, 1), (1, 1), (0, 1)],
        [(0, 2), (1, 2), (1, 1), (1, 0)]
    ], [
        [(2, 0), (0, 1), (1, 1), (2, 1)],
        [(2, 2), (1, 0), (1, 1), (1, 2)],
        [(0, 2), (2, 1), (1, 1), (0, 1)],
        [(0, 0), (1, 2), (1, 1), (1, 0)]
    ], [
        [(1, 1), (2, 1), (0, 2), (1, 2)],
        [(1, 1), (1, 2), (0, 0), (0, 1)],
        [(1, 1), (0, 1), (2, 0), (1, 0)],
        [(1, 1), (1, 0), (2, 2), (2, 1)]
    ], [
        [(1, 1), (2, 1), (0, 0), (0, 1)],
        [(1, 1), (1, 2), (2, 0), (1, 0)],
        [(1, 1), (0, 1), (2, 2), (2, 1)],
        [(1, 1), (1, 0), (0, 2), (1, 2)]
    ]
];

pub struct Tetromino {
    shape: Shape,
    rotate: u8,
    ref_pos: (i8, i8),
}

impl Tetromino {
    /*
    pub fn new(shape: Shape) -> Tetromino {
        Tetromino {
            shape: shape,
            rotate: 0,
            ref_pos: (3, 0),
        }
    }
    */

    pub fn random() -> Tetromino {
        let mut rng = thread_rng();
        let range = Uniform::new(0, SHAPES.len());
        return Tetromino {
            shape: SHAPES[range.sample(&mut rng)],
            rotate: 0,
            ref_pos: (3, 0),
        }
    }

    fn pos(&self) -> Block {
        match self.shape {
            Shape::I => BLOCKS[0],
            Shape::O => BLOCKS[1],
            Shape::T => BLOCKS[2],
            Shape::J => BLOCKS[3],
            Shape::L => BLOCKS[4],
            Shape::S => BLOCKS[5],
            Shape::Z => BLOCKS[6],
        }
    }

    fn left(&mut self) {
        self.ref_pos.0 -= 1;
    }

    fn right(&mut self) {
        self.ref_pos.0 += 1;
    }

    fn up(&mut self) {
        self.ref_pos.1 -= 1;
    }

    fn down(&mut self) {
        self.ref_pos.1 += 1;
    }

    fn clockwise(&mut self) {
        self.rotate = (self.rotate + 1) % 4;
    }

    fn anticlockwise(&mut self) {
        self.rotate = (self.rotate + 3) % 4;
    }
}
