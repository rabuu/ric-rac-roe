use crate::utils::{CellPos, CellState, PlayerType};

/* THE PLAYER */

pub struct Player {
    pub ptype: PlayerType,
    pub symbol: CellState,
}

impl Player {
    pub fn new(ptype: PlayerType) -> Player {
        Player {
            ptype,
            symbol: if ptype == PlayerType::Human {
                CellState::Circle
            } else if ptype == PlayerType::AI {
                CellState::Cross
            } else {
                panic!("Must be human or AI");
            },
        }
    }

    pub fn make_move(&mut self, pos: CellPos, field: &mut Vec<Vec<CellState>>) {
        field[pos.0 as usize][pos.1 as usize] = self.symbol;
    }
}
