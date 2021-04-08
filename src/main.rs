extern crate piston_window;
extern crate find_folder;

mod game;
mod draw;
mod props;
mod cell;

use piston_window::*;
use piston_window::types::Color;
use props::GameProperties;

use crate::game::Game;

const BG_COLOR: Color = [1f32, 1f32, 1f32, 1f32];

fn main() {
    let opengl = OpenGL::V3_2;
    let props: GameProperties = GameProperties::new(200, 3, 3, 10);

    let mut window: PistonWindow = WindowSettings::new("ric rac roe", (props.winwidth, props.winheight))
        .exit_on_esc(true)
        .resizable(false)
        .graphics_api(opengl)
        .build()
        .unwrap_or_else(|e| { panic!("Failed to build PistonWindow: {}", e) });

    let game = Game::new(props);

    let res = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("res").unwrap_or_else(|e| { panic!("Failed finding res folder: {}", e) });
    let rust = res.join("rust.png");
    let rust: G2dTexture = Texture::from_path(&mut window.create_texture_context(), &rust, Flip::None, &TextureSettings::new()).unwrap();

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, _d| {
            clear(BG_COLOR, g);
            game.draw(&c, g);
            image(&rust, c.transform, g);
        });
    }
}
