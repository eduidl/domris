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

type Point = (i8, i8);
type Block = Vec<Point>;

const BLOCKS: [[[Point; 4]; 4]; 7] = [
    [
        // I
        [(0, 1), (1, 1), (2, 1), (3, 1)],
        [(2, 0), (2, 1), (2, 2), (2, 3)],
        [(0, 2), (1, 2), (2, 2), (3, 2)],
        [(1, 0), (1, 1), (1, 2), (1, 3)]
    ], [ 
        // O
        [(0, 0), (0, 1), (1, 0), (1, 1)],
        [(0, 0), (0, 1), (1, 0), (1, 1)],
        [(0, 0), (0, 1), (1, 0), (1, 1)],
        [(0, 0), (0, 1), (1, 0), (1, 1)]
    ], [
        // T
        [(1, 0), (0, 1), (1, 1), (2, 1)],
        [(2, 1), (1, 0), (1, 1), (1, 2)],
        [(1, 2), (0, 1), (1, 1), (2, 1)],
        [(0, 1), (1, 0), (1, 1), (1, 2)],
    ], [
        // J
        [(0, 0), (0, 1), (1, 1), (2, 1)],
        [(2, 0), (1, 0), (1, 1), (1, 2)],
        [(2, 2), (0, 1), (1, 1), (2, 1)],
        [(0, 2), (1, 0), (1, 1), (1, 2)],
    ], [
        // L
        [(2, 0), (0, 1), (1, 1), (2, 1)],
        [(2, 2), (1, 0), (1, 1), (1, 2)],
        [(0, 2), (0, 1), (1, 1), (2, 1)],
        [(0, 0), (1, 0), (1, 1), (1, 2)],
    ], [
        // S
        [(1, 0), (0, 1), (1, 1), (2, 0)],
        [(2, 1), (1, 0), (1, 1), (2, 2)],
        [(1, 2), (0, 2), (1, 1), (2, 1)],
        [(0, 1), (0, 0), (1, 1), (1, 2)],
    ], [
        // Z
        [(1, 0), (0, 0), (1, 1), (2, 1)],
        [(2, 1), (2, 0), (1, 1), (1, 2)],
        [(1, 2), (0, 1), (1, 1), (2, 2)],
        [(0, 1), (1, 0), (1, 1), (0, 2)],
    ]
];

pub struct Tetromino {
    shape: Shape,
    rotate: usize,
    ref_pos: (i8, i8),
}

impl Tetromino {
    pub(super) fn random() -> Tetromino {
        let mut rng = thread_rng();
        let range = Uniform::new(0, SHAPES.len());
        
        return Tetromino {
            shape: SHAPES[range.sample(&mut rng)],
            rotate: 0,
            ref_pos: (4, 0),
        }
    }

    pub fn shape(&self) -> Shape {
        self.shape
    }

    pub fn coordinates(&self) -> Block {
        let blocks = match self.shape {
            Shape::I => BLOCKS[0],
            Shape::O => BLOCKS[1],
            Shape::T => BLOCKS[2],
            Shape::J => BLOCKS[3],
            Shape::L => BLOCKS[4],
            Shape::S => BLOCKS[5],
            Shape::Z => BLOCKS[6],
        }[self.rotate];

        blocks.into_iter()
            .map(|(x, y)| (self.ref_pos.0 + x, self.ref_pos.1 + y))
            .collect()
    }

    pub(super) fn move_x(&mut self, diff: i8) {
        self.ref_pos.0 += diff;
    }

    pub(super) fn move_y(&mut self, diff: i8) {
        self.ref_pos.1 += diff;
    }

    pub(super) fn rotate(&mut self, diff: usize) {
        self.rotate = (self.rotate + diff) % 4;
    }
}
