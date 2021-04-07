use piston_window::types::Color;
use piston_window::*;

use crate::{draw::draw_rect, props::GameProperties};


const BORDER_COL: Color = [0.0, 0.0, 0.0, 1.0];

pub struct Game {
    props: GameProperties,
}

impl Game {
    pub fn new(props: GameProperties) -> Game {
        Game {
            props,
        }
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        draw_rect(BORDER_COL, 0, 0, self.props.winwidth, self.props.bwidth, con, g)
    }
}
