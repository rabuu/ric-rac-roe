extern crate piston_window;

mod game;
mod draw;

use piston_window::*;
use piston_window::types::Color;

use crate::game::Game;
use crate::draw::to_coord_u32;

const BG_COLOR: Color = [0.5, 0.5, 0.5, 1.0];

fn main() {
    let (width, height) = (9, 9);

    let mut window: PistonWindow =
        WindowSettings::new("ric rac roe", [to_coord_u32(width), to_coord_u32(height)])
        .exit_on_esc(true).build().unwrap();

    let game = Game::new(width, height);

    while let Some(event) = window.next() {
        window.draw_2d(&event, |c, g, _| {
            clear(BG_COLOR, g);
            game.draw(&c, g);
        });
    }
}
