use opengl_graphics::GlGraphics;
use piston::RenderArgs;

use crate::gui::cell::Cell;
use crate::utils::{GameProperties, CellState, CellPos};

/* THE ACTUAL FRONTEND */

pub struct Front {
    props: GameProperties,
    gl: GlGraphics,
    pub cells: Vec<Vec<Cell>>,
}

impl Front {
    pub fn new(props: GameProperties, gl: GlGraphics) -> Front {
        Front {
            props,
            gl,
            cells: {
                let mut c = Vec::new();
                for i in 0..props.camount_x {
                    c.push(Vec::new());
                    for j in 0..props.camount_y {
                        c[i as usize].push(Cell::new(CellPos(i, j), props));
                    }
                }
                c
            },
        }
    }
    
    // rendering
    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BG_COLOR: [f32; 4] = [1.0; 4];
        const BORDER_COLOR: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        let p = self.props;
        let cells = &mut self.cells;

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
            
            // draw cells
            for i in 0..cells.len() {
                for cell in &cells[i] {
                    cell.canv.draw(&cell.textr, &DrawState::default(), c.transform.trans(cell.coords.0, cell.coords.1), gl);
                }
            }
        });
    }

    // update a single cell
    pub fn update_cell(&mut self, pos: CellPos, state: CellState) {
        self.cells[pos.0 as usize][pos.1 as usize].set_textr(state);
    }
}
