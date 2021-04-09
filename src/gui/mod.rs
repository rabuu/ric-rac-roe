extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate find_folder;

pub mod game;
mod cell;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::window::WindowSettings;

use crate::gui::game::Game;
use crate::utils::GameProperties;

pub struct Gui {
    pub game: Game,
    pub window: Window,
}

impl Gui {
    pub fn new(props: GameProperties) -> Gui {
        let opengl = OpenGL::V3_2;
        let window: Window = WindowSettings::new("ric rac roe", (props.winwidth, props.winheight))
            .exit_on_esc(true)
            .resizable(false)
            .graphics_api(opengl)
            .build()
            .unwrap_or_else(|e| { panic!("Failed to build window: {}", e) });

        let game: Game = Game::new(props, GlGraphics::new(opengl));
        
        Gui {
            game,
            window,
        }
    }
}
