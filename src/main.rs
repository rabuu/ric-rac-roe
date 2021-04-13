extern crate piston;

mod gui;
mod game;
mod bridge;
mod utils;

use bridge::key_pressed;
use piston::{Button, EventLoop, EventSettings, Events, MouseCursorEvent, PressEvent, RenderEvent};

use crate::gui::Gui;
use crate::game::Game;
use crate::bridge::mouse_clicked;
use crate::utils::{Coords, GameProperties};

/* MAIN */

fn main() {

    /* VARIABLES */

    // define game properties (cell length, border width, cell amount x, cell amount y)
    let props: GameProperties = GameProperties::new(100, 10, 3, 3);

    // declare cursor var
    let mut cursor = Coords(-1.0, -1.0);

    // instantiate gui and game backend
    let mut gui: Gui = Gui::new(props);
    let mut game: Game = Game::new(props);

    game.init(&mut gui.front);


    /* EVENTS */
    // catch events and pass them through
    let mut events = Events::new(EventSettings::new()).lazy(true);
    while let Some(e) = events.next(&mut gui.window) {

        // pass rendering to gui.front
        if let Some(args) = e.render_args() {
            gui.front.render(&args);
        }
        
        // grab cursor position
        e.mouse_cursor(|pos| {
            cursor = Coords(pos[0], pos[1]);
        });

        // pass mouse clicks to bridge::mouse_clicked()
        if let Some(Button::Mouse(button)) = e.press_args() {
            mouse_clicked(button, cursor, &mut game, &mut gui.front, props);
        }

        // key presses
        if let Some(Button::Keyboard(key)) = e.press_args() {
            key_pressed(key, &mut game, &mut gui.front);
        }
    }
}
