use std::collections::LinkedList;
use piston_window::{Context, G2d};
use piston_window::types::Color;

use draw::draw_block;
use crate::draw::draw_block;
use crate::main;

const SHAKE_COLOR: Color = [0.00, 0.80, 0.00, 1.00];

#[derive(Copy, Clone, PartialEq)]
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
#[derive(Debug, Clone)]
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
        let t =self.body.front().unwrap();
        (t.x, t.y)

    }

    pub fn move_forward(&mut self, dir: Option<Direction>) {
        match dir {
            Some(d) => self.direction = d,
            None => ()
        }

        let (last_x, last_y) = self.head_position();
        let new_block = match self.direction {
            Direction::Up => Block(
                x: last_x,
                y: last_y -1
            ),
            Direction::Down => Block(
                x: last_x,
                y: last_y + 1
            ),
            Direction::Left => Block(
                x: last_x - 1,
                y: last_y
            ),
            Direction::Right => Block(
                x: last_x + 1,
                y: last_y
            )
        };

        self.body.push_back(new_block);
        let rm_block = self.body.pop_back().unwrap();
        self.tail           = Some(rm_block);
    }

    pub fn next_direction(&self) -> Direction {
        self.direction
    }

    pub fn next_head(&self, new_dir: Option<Direction>) -> (i32, i32){
        let (curp_x, curp_y) = self.head_position();

        let mut current_direction =  self.direction;
        match new_dir {
            Some(d) => current_direction = d,
            None             => {}
        }

        match current_direction {
            Direction:Up   => (curp_x, curp_y + 1),
            Direction:Down => (curp_x, curp_y - 1),
            Direction:Left => (curp_x - 1, curp_y),
            Direction:Right=> (curp_x + 1, curp_y)
        }
    }

    pub fn restore_tail(&mut self){
        let tail= self.tail.unwrap();
        self.body.push_back(tail);
    }

    pub fn overlap_tail(&self, x: i32, y: i32) -> bool{
        let mut ch = 0;
        for block in &self.body{
            if x == block.x && y == block.y{
                return true;
            }
            ch += 1;
            if ch == self.body.len() - 1{
                break;
            }
        }
        return false;
    }

}









