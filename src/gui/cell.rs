use opengl_graphics::{Texture, TextureSettings};
use graphics::{Image, rectangle::square};

use crate::props::GameProperties;
use crate::state::CellState;

pub struct Cell {
    pos_x: u32,
    pos_y: u32,

    pub coord_x: f64,
    pub coord_y: f64,

    props: GameProperties,

    state: CellState,

    pub canv: Image,
    pub textr: Texture,
}

impl Cell {
    pub fn new(pos_x: u32, pos_y: u32, props: GameProperties) -> Cell {
        Cell {
            pos_x,
            pos_y,
            coord_x: ((props.clen + props.bwidth) * pos_x) as f64,
            coord_y: ((props.clen + props.bwidth) * pos_y) as f64,
            props,
            state: CellState::Empty,
            canv: Image::new().rect(square(0.0, 0.0, props.clen as f64)),
            textr: Texture::empty(&TextureSettings::new()).unwrap(),
        }
    }

    pub fn set_state(&mut self, state: CellState) {
        let res = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("res").unwrap_or_else(|e| { panic!("Failed finding res folder: {}", e) });

        let textr: Texture;

        match state {
            CellState::Empty => textr = Texture::empty(&TextureSettings::new()).unwrap(),
            CellState::Cross => textr = Texture::from_path(res.join("cross.png"), &TextureSettings::new()).unwrap(),
            CellState::Circle => textr = Texture::from_path(res.join("circle.png"), &TextureSettings::new()).unwrap(),
        } 
        
        self.textr = textr;
    }
}
