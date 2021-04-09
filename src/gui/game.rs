use opengl_graphics::GlGraphics;
use piston::RenderArgs;

use crate::props::GameProperties;
use crate::gui::cell::Cell;


pub struct Game {
    props: GameProperties,
    gl: GlGraphics,
    pub cells: Vec<Vec<Cell>>,
}

impl Game {
    pub fn new(props: GameProperties, gl: GlGraphics) -> Game {
        Game {
            props,
            gl,
            cells: {
                let mut c = Vec::new();
                for i in 0..props.camount_x {
                    c.push(Vec::new());
                    for j in 0..props.camount_y {
                        c[i as usize].push(Cell::new(i, j, props));
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
                    cell.canv.draw(&cell.textr, &DrawState::default(), c.transform.trans(cell.coord_x, cell.coord_y), gl);
                }
            }
        });
    }

    pub fn update(&mut self) {
        self.cells[2][1].set_state(crate::state::CellState::Circle);
    }
}
