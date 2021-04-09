use gui::Gui;
use input_handling::mouse_clicked;
use piston::{Button, EventLoop, EventSettings, Events, PressEvent, RenderEvent};
use props::GameProperties;

mod gui;
mod props;
mod state;
mod input_handling;

fn main() {

    let props: GameProperties = GameProperties::new(100, 3, 3, 10);

    let mut gui: Gui = Gui::new(props);

    let mut events = Events::new(EventSettings::new()).lazy(true);

    while let Some(e) = events.next(&mut gui.window) {
        if let Some(args) = e.render_args() {
            gui.game.render(&args);
        }

        if let Some(Button::Mouse(button)) = e.press_args() {
            mouse_clicked(button, &mut gui.game);
        }
    }
}
