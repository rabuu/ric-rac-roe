use opengl_graphics::{Texture, TextureSettings};
use graphics::Image;
use graphics::rectangle::square;

use crate::utils::{CellPos, CellState, Coords, GameProperties};

/* CELL */

pub struct Cell {
    pub coords: Coords,

    pub canv: Image,
    pub textr: Texture,
}

impl Cell {
    pub fn new(pos: CellPos, props: GameProperties) -> Cell {
        Cell {
            coords: Coords (
                ((props.clen + props.bwidth) * pos.0) as f64,
                ((props.clen + props.bwidth) * pos.1) as f64
            ),
            canv: Image::new().rect(square(0.0, 0.0, props.clen as f64)),
            textr: Texture::empty(&TextureSettings::new()).unwrap(),
        }
    }
    
    // set texture to cross/circle/empty
    pub fn set_textr(&mut self, state: CellState) {
        let res = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("res").expect("Failed finding folder");

        self.textr = match state {
            CellState::Empty => Texture::empty(&TextureSettings::new()).unwrap(),
            CellState::Cross => Texture::from_path(res.join("cross.png"), &TextureSettings::new()).unwrap(),
            CellState::Circle => Texture::from_path(res.join("circle.png"), &TextureSettings::new()).unwrap(),
        };
    }
}
