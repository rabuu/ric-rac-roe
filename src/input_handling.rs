use piston::MouseButton;

use crate::gui::front::Front;
use crate::utils::{GameProperties, CellState, CellPos, Coords};

pub fn mouse_clicked(btn: MouseButton, cursor: Coords, game: &mut Front, props: GameProperties) {
    println!("{:?}", btn );
    game.update_cell(get_cellpos_from_coords(cursor, props), CellState::Circle);
}

fn get_cellpos_from_coords(coords: Coords, props: GameProperties) -> CellPos  {
    CellPos {
        x: (coords.x / (props.clen + props.bwidth) as f64).floor() as u32,
        y: (coords.y / (props.clen + props.bwidth) as f64).floor() as u32,
    }
}
