use crate::utils::{CellPos, CellState, PlayerType};

use super::helpers::{announce_winner, empty_cells, winner};

/* THE PLAYER */

pub struct Player {
    pub ptype: PlayerType,
    pub symbol: CellState,
}

impl Player {
    pub fn new(ptype: PlayerType) -> Player {
        Player {
            ptype,
            symbol: match ptype {
                PlayerType::Human => CellState::Circle,
                PlayerType::AI => CellState::Cross,
            },
        }
    }

    pub fn make_move(&mut self, pos: CellPos, field: &mut Vec<Vec<CellState>>) -> bool {
        if winner(field) == None && empty_cells(field).len() > 0 {
            if empty_cells(field).iter().any(|&cell| cell == pos) {
                field[pos.0 as usize][pos.1 as usize] = self.symbol;
                if winner(field) != None || empty_cells(field).len() == 0 {
                    announce_winner(field);
                }
                true
            } else {
                announce_winner(field);
                false
            }
        } else {
            false
        }
    }
}
