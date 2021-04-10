use piston::MouseButton;

use crate::{gui::front::Front, utils::CellState};
use crate::game::Game;
use crate::utils::{GameProperties, CellPos, Coords};

pub fn mouse_clicked(btn: MouseButton, cursor: Coords, game: &mut Game, front: &mut Front, props: GameProperties) {
    println!("{:?}", btn );
    game.cell_pressed(get_cellpos_from_coords(cursor, props), front);
}

pub fn update_front(field: &Vec<Vec<CellState>>, front: &mut Front) {
    for i in 0..field.len() {
        for j in 0..field[i].len() {
            front.update_cell(CellPos {x: i as u32, y: j as u32}, field[i][j]);
        }
    }
}

fn get_cellpos_from_coords(coords: Coords, props: GameProperties) -> CellPos  {
    CellPos {
        x: (coords.x / (props.clen + props.bwidth) as f64).floor() as u32,
        y: (coords.y / (props.clen + props.bwidth) as f64).floor() as u32,
    }
}
