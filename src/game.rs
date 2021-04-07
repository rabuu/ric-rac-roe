use piston_window::types::Color;
use piston_window::*;

use crate::draw::draw_block;

const RED: Color = [1.0, 0.0, 0.0, 1.0];

pub struct Game {
    width: i32,
    height: i32,
}

impl Game {
    pub fn new(width: i32, height: i32) -> Game {
        Game {
            width,
            height,
        }
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        draw_block(RED, self.width / 2, self.height / 2, con, g);
    }
}
