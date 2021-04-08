extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate find_folder;

mod game;
mod props;
mod cell;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, UpdateEvent};
use piston::window::WindowSettings;

use crate::props::GameProperties;
use crate::game::Game;

// const BG_COLOR: [f32; 4] = [1f32, 1f32, 1f32, 1f32];

fn main() {
    let opengl = OpenGL::V3_2;
    let props: GameProperties = GameProperties::new(200, 3, 3, 10);

    let mut window: Window = WindowSettings::new("ric rac roe", (props.winwidth, props.winheight))
        .exit_on_esc(true)
        .resizable(false)
        .graphics_api(opengl)
        .build()
        .unwrap_or_else(|e| { panic!("Failed to build window: {}", e) });

    let mut game = Game::new(props, GlGraphics::new(opengl));

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            game.render(&args);
        }

        if let Some(args) = e.update_args() {
            game.update(&args);
        }
    }
}
