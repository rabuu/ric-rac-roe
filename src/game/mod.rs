mod player;

use crate::{bridge, gui::front::Front};
use crate::utils::{CellPos, CellState, GameProperties};

use self::player::Player;

pub struct Game {
    field: Vec<Vec<CellState>>,
    human: Player,
    ai: Player,
}

impl Game {
    pub fn new(props: GameProperties) -> Game {
        Game {
            field: vec![vec![CellState::Empty; props.camount_x as usize]; props.camount_y as usize],
            human: Player::new(CellState::Circle),
            ai: Player::new(CellState::Cross),
        }
    }

    pub fn cell_pressed(&mut self, pos: CellPos, front: &mut Front) {
        if self.field.len() != 3 || self.field.iter().any(|row| row.len() != 3) {
            panic!("3x3 only.")
        }
        if self.empty_cells().iter().any(|&cell| cell == pos) && self.winner() == None {
            self.human.make_move(pos, &mut self.field);
            bridge::update_front(&self.field, front);
            println!("Human made decision");
            if self.empty_cells().len() > 0 && self.winner() == None {
                self.ai.make_move(CellPos(2, 1), &mut self.field);
                bridge::update_front(&self.field, front);
                println!("AI made decision");
            }
        }
    }

    fn empty_cells(&self) -> Vec<CellPos> {
        let mut cells: Vec<CellPos> = Vec::new();
        for i in 0..self.field.len() {
            for j in 0..self.field[i].len() {
                if self.field[i][j] == CellState::Empty {
                   cells.push(CellPos(i as u32, j as u32));
                }
            }
        }
        cells
    }

    fn winner(&self) -> Option<CellState> {
        let win_state: Vec<Vec<CellState>> = vec![
            vec![self.field[0][0], self.field[0][1], self.field[0][2]],
            vec![self.field[1][0], self.field[1][1], self.field[1][2]],
            vec![self.field[2][0], self.field[2][1], self.field[2][2]],
		    vec![self.field[0][0], self.field[1][0], self.field[2][0]],
		    vec![self.field[0][1], self.field[1][1], self.field[2][1]],
		    vec![self.field[0][2], self.field[1][2], self.field[2][2]],
		    vec![self.field[0][0], self.field[1][1], self.field[2][2]],
		    vec![self.field[2][0], self.field[1][1], self.field[0][2]],
        ];
        for i in 0..win_state.len() {
            let line = &win_state[i];
            let (mut cr, mut ci) = (0, 0);
            for j in 0..line.len() {
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
