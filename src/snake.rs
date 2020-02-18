use std::collections::LinkedList;
use piston_window::{Context, G2d};
use piston_window::types::Color;

use draw::draw_block;


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