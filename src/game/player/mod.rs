use crate::utils::{CellPos, CellState};

pub mod human;
pub mod ai;


pub trait Player {
    fn make_move(&mut self, pos: CellPos, field: &mut Vec<Vec<CellState>>);
}
