use std::process::exit;

use piston::{Key, MouseButton};

use crate::gui::front::Front;
use crate::game::Game;
use crate::utils::{GameProperties, CellPos, CellState, Coords};

/* BRIDGE BETWEEN GUI AND BACKEND */

// send mouse clicks from main to backend
pub fn mouse_clicked(btn: MouseButton, cursor: Coords, game: &mut Game, front: &mut Front, props: GameProperties) {
    if btn == MouseButton::Left {
        game.trigger(Some(get_cellpos_from_coords(cursor, props)), front);
    }
}

// tranfer key presses
pub fn key_pressed(key: Key, game: &mut Game, front: &mut Front) {
    if key == Key::Q {
        exit(0);
    }

    if key == Key::R {
        game.restart(front);
    }
}

// update front/redraw all cells according to a field
pub fn update_front(field: &Vec<Vec<CellState>>, front: &mut Front) {
    for i in 0..field.len() {
        for j in 0..field[i].len() {
            front.update_cell(CellPos(i as u32, j as u32), field[i][j]);
        }
    }
}


/* HELPER FUNCTIONS */

// calculate cell position from absolute coordinates
fn get_cellpos_from_coords(coords: Coords, props: GameProperties) -> CellPos  {
    CellPos (
        (coords.0 / (props.clen + props.bwidth) as f64).floor() as u32,
        (coords.1 / (props.clen + props.bwidth) as f64).floor() as u32,
    )
}
