use opengl_graphics::{Texture, TextureSettings};
use graphics::{Image, rectangle::square};

use crate::utils::{CellPos, CellState, Coords, GameProperties};

pub struct Cell {
    pos: CellPos,

    pub coords: Coords,

    props: GameProperties,

    state: CellState,

    pub canv: Image,
    pub textr: Texture,
}

impl Cell {
    pub fn new(pos: CellPos, props: GameProperties) -> Cell {
        Cell {
            pos,
            coords: Coords {x: ((props.clen + props.bwidth) * pos.x) as f64,
                            y: ((props.clen + props.bwidth) * pos.y) as f64},
            props,
            state: CellState::Empty,
            canv: Image::new().rect(square(0.0, 0.0, props.clen as f64)),
            textr: Texture::empty(&TextureSettings::new()).unwrap(),
        }
    }

    pub fn set_state(&mut self, state: CellState) {
        let res = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("res").unwrap_or_else(|e| { panic!("Failed finding res folder: {}", e) });

        self.textr = match state {
            CellState::Empty => Texture::empty(&TextureSettings::new()).unwrap(),
            CellState::Cross => Texture::from_path(res.join("cross.png"), &TextureSettings::new()).unwrap(),
            CellState::Circle => Texture::from_path(res.join("circle.png"), &TextureSettings::new()).unwrap(),
        };
    }
}
