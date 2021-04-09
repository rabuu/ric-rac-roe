use gui::Gui;
use input_handling::mouse_clicked;
use piston::{Button, EventLoop, EventSettings, Events, MouseCursorEvent, PressEvent, RenderEvent};
use utils::{Coords, GameProperties};

mod gui;
mod input_handling;
mod utils;

fn main() {

    let props: GameProperties = GameProperties::new(100, 3, 3, 10);
    let mut cursor = Coords {x: -1.0, y: -1.0};

    let mut gui: Gui = Gui::new(props);

    let mut events = Events::new(EventSettings::new()).lazy(true);

    while let Some(e) = events.next(&mut gui.window) {
        if let Some(args) = e.render_args() {
            gui.game.render(&args);
        }

        e.mouse_cursor(|pos| {
            cursor = Coords { x: pos[0], y: pos[1] };
        });

        if let Some(Button::Mouse(button)) = e.press_args() {
            mouse_clicked(button, cursor, &mut gui.game, props);
        }

    }
}
