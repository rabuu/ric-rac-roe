use crate::utils::{CellPos, CellState};

pub struct Player {
    symbol: CellState,
}

impl Player {
    pub fn new(symbol: CellState) -> Player {
        Player {
            symbol,
        }
    }

    pub fn make_move(&mut self, pos: CellPos, field: &mut Vec<Vec<CellState>>) {
        field[pos.0 as usize][pos.1 as usize] = self.symbol;
    }
}
