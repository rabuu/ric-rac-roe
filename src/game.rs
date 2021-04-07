use piston_window::types::Color;
use piston_window::*;

use crate::draw::draw_rect;
use crate::props::GameProperties;
use crate::cell::Cell;


const BORDER_COL: Color = [0.0, 0.0, 0.0, 1.0];

pub struct Game {
    props: GameProperties,
    cells: Vec<Cell>,
}

impl Game {
    pub fn new(props: GameProperties) -> Game {
        Game {
            props,
            cells: {
                let mut c = Vec::new();
                for i in 0..props.camount_x {
                    for j in 0..props.camount_y {
                        c.push(Cell::new(i, j, props));
                    }
                }
                c
            },
        }
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        let p = self.props;
        for i in 1..p.camount_x {
            draw_rect(BORDER_COL, i * p.clen + (i - 1) * p.bwidth, 0, p.bwidth, p.winheight, con, g);
        }
        for i in 1..p.camount_y {
            draw_rect(BORDER_COL, 0, i * p.clen + (i - 1) * p.bwidth, p.winwidth, p.bwidth, con, g);
        }
    }
}
