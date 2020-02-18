use piston_window::types::Color;
use piston_window::{G2d, Context, rectangle};

const BLOCKSIZE : f64 = 25.0;

pub fn to_coord(gamecoord: i32) -> f64 {
    (gamecoord as f64) * BLOCKSIZE
}

pub fn draw_block(color: Color, x: i32, y: i32, con: &Context, g: &mut G2d,){
    let gui_x = to_coord(x);
    let gui_y = to_coord(y);

    rectangle(color,[gui_x, gui_y, BLOCKSIZE, BLOCKSIZE], con.transform, g);
}

pub fn draw_rectangle(color: Color, x: i32, y: i32, width: i32, height: i32 , con: &Context, g: &mut G2d,){
    let gui_x = to_coord(x);
    let gui_y = to_coord(y);

    rectangle( color, [gui_x, gui_y, BLOCKSIZE * (width as f64), BLOCKSIZE * (height as f64)], con.transform, g );
}