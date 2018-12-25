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

type Block = Vec<(i8, i8)>;

const BLOCKS: [[[(i8, i8); 4]; 4]; 7] = [
    [
        // I
        [(0, 1), (1, 1), (2, 1), (3, 1)],
        [(2, 0), (2, 1), (2, 2), (2, 3)],
        [(3, 2), (2, 2), (1, 2), (0, 2)],
        [(1, 3), (1, 2), (1, 1), (1, 0)],
    ], [ 
        // O
        [(0, 0), (0, 1), (1, 1), (1, 0)],
        [(1, 0), (0, 0), (0, 1), (1, 1)],
        [(1, 1), (1, 0), (0, 0), (0, 1)],
        [(0, 1), (1, 1), (1, 0), (0, 0)],
    ], [
        // T
        [(1, 0), (0, 1), (1, 1), (2, 1)],
        [(2, 1), (1, 0), (1, 1), (1, 2)],
        [(1, 2), (2, 1), (1, 1), (0, 1)],
        [(0, 1), (1, 2), (1, 1), (1, 0)],
    ], [
        // J
        [(0, 0), (0, 1), (1, 1), (2, 1)],
        [(2, 0), (1, 0), (1, 1), (1, 2)],
        [(2, 2), (2, 1), (1, 1), (0, 1)],
        [(0, 2), (1, 2), (1, 1), (1, 0)],
    ], [
        // L
        [(2, 0), (0, 1), (1, 1), (2, 1)],
        [(2, 2), (1, 0), (1, 1), (1, 2)],
        [(0, 2), (2, 1), (1, 1), (0, 1)],
        [(0, 0), (1, 2), (1, 1), (1, 0)],
    ], [
        // S
        [(1, 0), (0, 1), (1, 1), (2, 0)],
        [(2, 1), (1, 0), (1, 1), (2, 2)],
        [(1, 2), (2, 1), (1, 1), (0, 2)],
        [(0, 1), (1, 2), (1, 1), (0, 0)],
    ], [
        // Z
        [(1, 0), (0, 0), (1, 1), (2, 1)],
        [(2, 1), (2, 0), (1, 1), (1, 2)],
        [(1, 2), (2, 2), (1, 1), (0, 1)],
        [(0, 1), (0, 2), (1, 1), (1, 0)],
    ],
];

pub struct Tetromino {
    pub shape: Shape,
    rotate: usize,
    ref_pos: (i8, i8),
    numbers: Vec<u8>,
}

impl Tetromino {
    pub(super) fn random(number_max: u8) -> Self {
        let mut rng = thread_rng();
        let shape_range = Uniform::new(0, SHAPES.len());
        let number_range = Uniform::new(0, number_max + 1);
        Self {
            shape: SHAPES[shape_range.sample(&mut rng)],
            rotate: 0,
            ref_pos: (4, 0),
            numbers: number_range.sample_iter(&mut rng).take(4).collect(),
        }
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

    pub fn numbers(&self) -> &Vec<u8> {
        &self.numbers
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
