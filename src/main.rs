extern crate piston_window;

mod game;
mod draw;
mod props;

use piston_window::*;
use piston_window::types::Color;
use props::GameProperties;

use crate::game::Game;

const BG_COLOR: Color = [0.5, 0.5, 0.5, 1.0];

fn main() {
    let props: GameProperties = GameProperties::new(100, 3, 3, 10);

    let mut window: PistonWindow = WindowSettings::new("ric rac roe", (props.winwidth, props.winheight))
        .exit_on_esc(true)
        .resizable(false)
        .build()
        .unwrap_or_else(|e| { panic!("Failed to build PistonWindow: {}", e) });

    let game = Game::new(props);

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, _d| {
            game.draw(&c, g);
            clear(BG_COLOR, g);
        });
    }
}
