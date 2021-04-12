mod player;
mod minimax;
mod helpers;

use minimax::call_minimax;

use crate::bridge::update_front; 
use crate::gui::front::Front;
use crate::utils::{CellPos, CellState, GameProperties, PlayerType};
use crate::game::helpers::{empty_cells, winner, announce_winner};
use crate::game::player::Player;


/* THE GAME BACKEND */

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
    
    // the actual game flow triggered by a cell press
    pub fn cell_pressed(&mut self, pos: CellPos, front: &mut Front) {
        // filter only 3x3 games
        if self.field.len() != 3 || self.field.iter().any(|row| row.len() != 3) {
            panic!("3x3 only.")
        }

        if empty_cells(&self.field).iter().any(|&cell| cell == pos) && winner(&self.field) == None {
            self.human.make_move(pos, &mut self.field);
            update_front(&self.field, front);
            if empty_cells(&self.field).len() > 0 && winner(&self.field) == None {
                    self.ai.make_move(call_minimax(&self.field, self.ai.ptype), &mut self.field);
                    update_front(&self.field, front);
                    if winner(&self.field) != None {
                        announce_winner(&self.field);
                    }
            } else {
                announce_winner(&self.field);
            }
        } 
    }
}
