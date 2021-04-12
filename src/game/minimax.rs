use std::f32::{INFINITY, NEG_INFINITY};

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
    
    let mut pval = if player == PlayerType::AI {1} else if player == PlayerType::Human {-1} else { panic!(); };
    let depth: usize = empty_cells(field).len();
    let mm = minimax(&mut mm_field, depth, &mut pval);
    CellPos(mm[0] as u32, mm[1] as u32)
}

// actual minimax algorithm
fn minimax(field: &mut Vec<Vec<i32>>, depth: usize, pval: &mut i32) -> [f32; 3] {
    let mut best: [f32; 3];

    if *pval == 1 {
        best = [-1_f32, -1_f32, NEG_INFINITY];
    } else {
        best = [-1_f32, -1_f32, INFINITY];
    }

    if depth == 0 {
        let score: f32 = evaluate(field);
        return [-1_f32, -1_f32, score];
    }

    for cell in empty_cells_mm(field) {
        let x = cell.0;
        let y = cell.1;
        field[x as usize][y as usize] = *pval;
        let mut score = minimax(field, depth - 1, &mut -*pval);
        field[x as usize][y as usize] = 0;
        score[0] = x as f32;
        score[1] = y as f32;

        if *pval == 1 {
            if score[2] > best[2] {
                best = score;
            }
        }
        else {
            if score[2] < best[2] {
                best = score;
            }
        }
    }

    best
    
}

// helper function to evaluate scores
fn evaluate(field: &Vec<Vec<i32>>) -> f32 {
    let mut score = 0;
    match winner(&mm_field_to_field(field)) {
        Some(CellState::Cross) => score += 1,
        Some(CellState::Circle) => score -= 1,
        None => score = 0,
        _ => panic!(),
    }
    score as f32
}
