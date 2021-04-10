use crate::utils::{CellPos, CellState};

use super::Player;

pub struct AI {
    symbol: CellState,
}

impl AI {
    pub fn new() -> AI {
        AI {
            symbol: CellState::Cross,
        }
    }
}

impl Player for AI {
    fn make_move(&mut self, pos: CellPos, field: &mut Vec<Vec<CellState>>) {

    }
}
