mod player;
mod minimax;
mod helpers;

use minimax::call_minimax;

use crate::{bridge, gui::front::Front};
use crate::utils::{CellPos, CellState, GameProperties, PlayerType};

use self::{helpers::{empty_cells, winner}, player::Player};

pub struct Game {
    field: Vec<Vec<CellState>>,
    human: Player,
    ai: Player,
}

impl Game {
    pub fn new(props: GameProperties) -> Game {
        Game {
            field: vec![vec![CellState::Empty; props.camount_x as usize]; props.camount_y as usize],
            human: Player::new(PlayerType::Human),
            ai: Player::new(PlayerType::AI),
        }
    }

    pub fn cell_pressed(&mut self, pos: CellPos, front: &mut Front) {
        if self.field.len() != 3 || self.field.iter().any(|row| row.len() != 3) {
            panic!("3x3 only.")
        }
        if empty_cells(&self.field).iter().any(|&cell| cell == pos) && winner(&self.field) == None {
            self.human.make_move(pos, &mut self.field);
            bridge::update_front(&self.field, front);
            println!("Human made decision");
            if empty_cells(&self.field).len() > 0 && winner(&self.field) == None {
                self.ai.make_move(call_minimax(&self.field, self.ai.ptype), &mut self.field);
                bridge::update_front(&self.field, front);
                println!("AI made decision");
            }
        }
    }
}
