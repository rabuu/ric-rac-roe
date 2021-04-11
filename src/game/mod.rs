mod player;

use crate::{bridge, gui::front::Front};
use crate::utils::{CellPos, CellState, GameProperties};
use crate::game::player::{Player, human::Human, ai::AI};

pub struct Game {
    field: Vec<Vec<CellState>>,
    human: Human,
    ai: AI,
}

impl Game {
    pub fn new(props: GameProperties) -> Game {
        Game {
            field: vec![vec![CellState::Empty; props.camount_x as usize]; props.camount_y as usize],
            human: Human::new(),
            ai: AI::new(),
        }
    }

    pub fn cell_pressed(&mut self, pos: CellPos, front: &mut Front) {
        if self.empty_cells().iter().any(|&cell| cell == pos) {
            self.human.make_move(pos, &mut self.field);
            bridge::update_front(&self.field, front);
        }
    }

    fn empty_cells(&self) -> Vec<CellPos> {
        let mut cells: Vec<CellPos> = Vec::new();
        for i in 0..self.field.len() {
            for j in 0..self.field[i].len() {
                cells.push(CellPos(i as u32, j as u32));
            }
        }
        cells
    }

    fn winner(&self) -> Option<CellState> {
        let win_state: Vec<Vec<CellState>> = vec![
            vec![self.field[0][0], self.field[0][1], self.field[0][2]]
        ];
        for i in 0..win_state.len() {
            let line = &win_state[i];
            let (mut cr, mut ci) = (0, 0);
            for j in 0..2 {
                if line[j] == CellState::Cross {
                    cr += 1;
                } else if line[j] == CellState::Circle {
                    ci += 1;
                }
                if cr == 3 {
                    return Some(CellState::Cross);
                }
                if ci == 3 {
                    return Some(CellState::Circle);
                }
            }
        }
        return None;
    }
}
