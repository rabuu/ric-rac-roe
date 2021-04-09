use gui::Gui;
use piston::{Button, EventLoop, EventSettings, Events, MouseCursorEvent, PressEvent, RenderEvent, UpdateEvent};
use props::GameProperties;

mod gui;
mod props;
mod state;

fn main() {

    let props: GameProperties = GameProperties::new(100, 3, 3, 10);

    let mut gui: Gui = Gui::new(props);

    let mut events = Events::new(EventSettings::new()).lazy(true);
    while let Some(e) = events.next(&mut gui.window) {
        if let Some(args) = e.render_args() {
            gui.game.render(&args);
        }

        if let Some(Button::Mouse(button)) = e.press_args() {
            println!("Pressed mouse button '{:?}'", button);
            gui.game.update();
        }
    }


}
