use piston_window::*;
use piston_window::types::Color;

use rand::Rng;
use rand::thread_rng;

use snake::Direction;
use snake::Snake;

use draw::{draw_block, draw_rectangle};

const FOOD_COLOR: Color     = [8.0, 0.0, 0.0, 1.0];
const BORDER_COLOR: Color   = [0.0, 0.0, 0.0, 1.0];
const GAMEOVER_COLOR: Color = [0.9, 0.0, 0.0, 0.5];


const MOVING_PERIOD: f64 = 0.1;
const RESTART_TIME:  f64 = 1.0;


pub struct  Game {
    snake:      Snake,
    food_exist: bool,
    food_x:     i32,
    food_y:     i32,

    width:      i32,
    height:     i32,

    game_over:    bool,
    waiting_time: f64
}

