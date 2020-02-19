use std::collections::LinkedList;
use piston_window::{Context, G2d};
use piston_window::types::Color;

use draw::draw_block;
use crate::draw::draw_block;

const SHAKE_COLOR: Color = [0.00, 0.80, 0.00, 1.00];

pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {

    pub fn opposite(&self) -> Direction {
        match *self {
            Direction::Up    => Direction::Down,
            Direction::Down  => Direction::Up,
            Direction::Left  => Direction::Right,
            Direction::Right => Direction::Left
        }
    }
}

struct Block {
    x: i32,
    y: i32
}

pub  struct Snake {
    direction : Direction,
    body      : LinkedList<Block>,
    tail      : Option<Block>
}

impl Snake {
    pub fn new(x: i32, y: i32) -> Snake {
        let mut body: LinkedList<Block> = LinkedList::new();
        body.push_back(Block{x: x + 2, y});
        body.push_back(Block{x: x + 1, y});
        body.push_back(Block{x, y});

        Snake(direction: Direction::Right, body: body, tail: None)
    }

    pub fn draw(&self, con: &Context, g: &mut G2d){

        for block in &self.body{
            draw_block(SHAKE_COLOR, block.x, block.y, con, g);
        }
    }

    pub fn head_position(&self) -> (i32, i32) {

        self.body.fro
    }
}









