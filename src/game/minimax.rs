use crate::utils::{CellPos, CellState, PlayerType};

use super::helpers::{empty_cells, empty_cells_mm, mm_field_to_field, winner};

/* IMPLEMENTATION OF THE MINIMAX ALGORITHM */

// public function that triggers the minimax algorithm 
pub fn call_minimax(field: &Vec<Vec<CellState>>, player: PlayerType) -> CellPos {
    let mut mm_field: Vec<Vec<i32>> = Vec::new();
    for i in 0..field.len() {
        mm_field.push(Vec::new());
        for j in 0..field[i].len() {
            if field[i][j] == CellState::Cross {
                mm_field[i].push(1);
            } else if field[i][j] == CellState::Circle {
                mm_field[i].push(-1);
            } else if field[i][j] == CellState::Empty {
                mm_field[i].push(0);
            } else {
                panic!();
            }
        }
    }
    
    let pval = if player == PlayerType::AI {1} else if player == PlayerType::Human {-1} else { panic!(); };
    let depth: usize = empty_cells(field).len();
    let mm: [f32; 3] = match depth {
        9 => [0_f32, 0_f32, 0_f32],
        _ => minimax(&mut mm_field, depth, &pval)
    };
    CellPos(mm[0] as u32, mm[1] as u32)
}

// actual minimax algorithm
fn minimax(field: &mut Vec<Vec<i32>>, depth: usize, pval: &i32) -> [f32; 3] {
    // declare return var
    let mut best: [f32; 3];

    // set standard values
    if *pval == 1 {
        best = [-1_f32, -1_f32, -1000_f32];
    } else {
        best = [-1_f32, -1_f32, 1000_f32];
    }

    // if depth 0 or ther would be a winner return -1, -1 and evaluated score
    if depth == 0 || winner(&mm_field_to_field(field)) != None {
        let score: f32 = evaluate(field);
        return [-1_f32, -1_f32, score];
    }

    // go for every empty cell
    for cell in empty_cells_mm(field) {
        // store position in vars
        let x = cell.0;
        let y = cell.1;

        // set this position to the player value (1 or -1)
        field[x as usize][y as usize] = *pval;

        // with this modified field run function again recursively
        let mut score = minimax(field, depth - 1, &mut -*pval);

        // set back to empty
        field[x as usize][y as usize] = 0;

        // set return values to the position
        score[0] = x as f32;
        score[1] = y as f32;

        // check whether the score improved
        if *pval == 1 {
            if score[2] > best[2] {
                best = score;
            }
        } else {
            if score[2] < best[2] {
                best = score;
            }
        }
    }

    // return best scoring cell
    best
    
}

// helper function to evaluate scores
fn evaluate(field: &Vec<Vec<i32>>) -> f32 {
    let score;
    match winner(&mm_field_to_field(field)) {
        Some(CellState::Cross) => score = 1,
        Some(CellState::Circle) => score = -1,
        None => score = 0,
        _ => panic!(),
    }
    score as f32
}
