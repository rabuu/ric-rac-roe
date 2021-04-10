use crate::gui::front::Front;
use crate::utils::{CellPos, CellState, GameProperties};
use crate::bridge::update_front;

pub struct Game {
    props: GameProperties,

    field: Vec<Vec<CellState>>,
}

impl Game {
    pub fn new(props: GameProperties) -> Game {
        Game {
            props,
            field: vec![vec![CellState::Empty; props.camount_x as usize]; props.camount_y as usize],
        }
    }

    pub fn cell_pressed(&mut self, pos: CellPos, front: &mut Front) {
        self.field[pos.0 as usize][pos.1 as usize] = CellState::Cross;
        update_front(&self.field, front);
    }
}
