use std::path::Path;

use graphics::{Context, DrawState, Image, Transformed, rectangle::square};
use opengl_graphics::{GlGraphics, Texture, TextureSettings};

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
            .for_folder("res").unwrap_or_else(|e| { panic!("Failed finding res folder: {}", e) });
        let img = res.join("circle.png");
        let img = Texture::from_path(&img, &TextureSettings::new()).unwrap();
        img
    }

    pub fn draw(&self, c: Context, gl: &mut GlGraphics) {
        let image = Image::new().rect(square(0.0, 0.0, 0.0));
        let texture = Texture::from_path(Path::new("../res/cross.png"), &TextureSettings::new()).unwrap();
        image.draw(&texture, &DrawState::default(), c.transform.scale(self.props.clen as f64 / 200f64, self.props.clen as f64 / 200f64), gl);
    }
}

#[derive(Debug)]
pub enum State {
    Cross,
    Circle,
    Empty,
}
