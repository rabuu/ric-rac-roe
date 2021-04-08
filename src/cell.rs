use opengl_graphics::{Texture, TextureSettings};

use crate::props::GameProperties;

#[derive(Debug)]
pub struct Cell {
    pos_x: u32,
    pos_y: u32,

    props: GameProperties,

    state: State,
}

impl Cell {
    pub fn new(pos_x: u32, pos_y: u32, props: GameProperties) -> Cell {
        Cell {
            pos_x,
            pos_y,
            props,
            state: State::Empty,
        }
    }

    pub fn get_coords(&self) -> (u32, u32) {
        ((self.props.clen + self.props.bwidth) * self.pos_x,
        (self.props.clen + self.props.bwidth) * self.pos_y)
    }

    pub fn get_texture(&self) -> Texture {
        let res = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("res").unwrap();
        let img = res.join("rust.png");
        let img: Texture = Texture::from_path(&img, &TextureSettings::new()).unwrap();
        img
    }
}

#[derive(Debug)]
pub enum State {
    Cross,
    Circle,
    Empty,
}
