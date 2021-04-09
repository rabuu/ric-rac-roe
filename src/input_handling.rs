use piston::MouseButton;

use crate::gui::game::Game;

pub fn mouse_clicked(btn: MouseButton, game: &mut Game) {
    println!("{:?}", btn );
    game.update();
}
