use crate::utils::{CellPos, CellState};

use super::Player;

pub struct Human {
    symbol: CellState,
}

impl Human {
    pub fn new() -> Human {
        Human {
            symbol: CellState::Circle,
        }
    }
}

impl Player for Human {
    fn make_move(&mut self, pos: CellPos, field: &mut Vec<Vec<CellState>>) {
        
    }
}
