use opengl_graphics::{Texture, TextureSettings};

use crate::props::GameProperties;
use crate::state::CellState;

#[derive(Debug)]
pub struct Cell {
    pos_x: u32,
    pos_y: u32,

    props: GameProperties,

    state: CellState,
}

impl Cell {
    pub fn new(pos_x: u32, pos_y: u32, props: GameProperties) -> Cell {
        Cell {
            pos_x,
            pos_y,
            props,
            state: CellState::Empty,
        }
    }

    pub fn get_coords(&self) -> (u32, u32) {
        ((self.props.clen + self.props.bwidth) * self.pos_x,
        (self.props.clen + self.props.bwidth) * self.pos_y)
    }

    pub fn get_texture(&self) -> Texture {
        let res = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("res").unwrap_or_else(|e| { panic!("Failed finding res folder: {}", e) });
        let img = res.join("circle.png");
        let img = Texture::from_path(&img, &TextureSettings::new()).unwrap();
        img
    }
}
