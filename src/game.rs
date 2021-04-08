use opengl_graphics::GlGraphics;
use piston::{RenderArgs, UpdateArgs};

use crate::props::GameProperties;
use crate::cell::Cell;


pub struct Game {
    props: GameProperties,
    gl: GlGraphics,
    cells: Vec<Cell>,
}

impl Game {
    pub fn new(props: GameProperties, gl: GlGraphics) -> Game {
        Game {
            props,
            gl,
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

    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BG_COLOR: [f32; 4] = [1.0; 4];
        const BORDER_COLOR: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        let p = self.props;
        let cell = &self.cells[2];

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BG_COLOR, gl);

            // draw grid
            for i in 1..p.camount_x {
                rectangle(BORDER_COLOR, [(i * p.clen + (i - 1) * p.bwidth) as f64, 0f64, p.bwidth as f64, p.winheight as f64], c.transform, gl);
            }
            for i in 1..p.camount_y {
                rectangle(BORDER_COLOR, [0f64, (i * p.clen + (i - 1) * p.bwidth) as f64, p.winwidth as f64, p.bwidth as f64], c.transform, gl);
            }
            
            image(&cell.get_texture(), c.transform.trans(cell.get_coords().0 as f64, cell.get_coords().1 as f64), gl);
        });
    }

    pub fn update(&mut self, _args: &UpdateArgs) {

    }
}
