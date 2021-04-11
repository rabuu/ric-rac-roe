use crate::utils::{CellState, CellPos};

pub fn empty_cells(field: &Vec<Vec<CellState>>) -> Vec<CellPos> {
    let mut cells: Vec<CellPos> = Vec::new();
    for i in 0..field.len() {
        for j in 0..field[i].len() {
            if field[i][j] == CellState::Empty {
               cells.push(CellPos(i as u32, j as u32));
            }
        }
    }
    cells
}

pub fn empty_cells_mm(field: &Vec<Vec<i32>>) -> Vec<CellPos> {
    let mut cells: Vec<CellPos> = Vec::new();
    for i in 0..field.len() {
        for j in 0..field[i].len() {
            if field[i][j] == 0 {
               cells.push(CellPos(i as u32, j as u32));
            }
        }
    }
    cells
}

pub fn mm_field_to_field(field: &Vec<Vec<i32>>) -> Vec<Vec<CellState>> {
    let mut cells: Vec<Vec<CellState>> = Vec::new();
    for i in 0..field.len() {
        cells.push(Vec::new());
        for j in 0..field[i].len() {
            if field[i][j] == 0 {
               cells[i].push(CellState::Empty);
            } else if field[i][j] > 0 {
                cells[i].push(CellState::Cross);
            } else if field[i][j] < 0 {
                cells[i].push(CellState::Circle);
            } else { panic!(); }
        }
    }
    cells
}

pub fn winner(field: &Vec<Vec<CellState>>) -> Option<CellState> {
    let win_state: Vec<Vec<CellState>> = vec![
        vec![field[0][0], field[0][1], field[0][2]],
        vec![field[1][0], field[1][1], field[1][2]],
        vec![field[2][0], field[2][1], field[2][2]],
        vec![field[0][0], field[1][0], field[2][0]],
	vec![field[0][1], field[1][1], field[2][1]],
	vec![field[0][2], field[1][2], field[2][2]],
	vec![field[0][0], field[1][1], field[2][2]],
	vec![field[2][0], field[1][1], field[0][2]],
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
