mod player;
mod minimax;
mod helpers;

use crate::bridge::update_front; 
use crate::gui::front::Front;
use crate::utils::{CellPos, CellState, GameProperties, PlayerType};
use crate::game::player::Player;
use crate::game::minimax::call_minimax;


/* THE GAME BACKEND */

pub struct Game {
    field: Vec<Vec<CellState>>,
    props: GameProperties,

    human: Player,
    ai: Player,

    starter: bool,
    turn: bool,
}

impl Game {
    pub fn new(props: GameProperties) -> Game {
        let starter = props.args.starter;
        Game {
            field: vec![vec![CellState::Empty; props.camount_x as usize]; props.camount_y as usize],
            props,
            human: Player::new(PlayerType::Human),
            ai: Player::new(PlayerType::AI),
            starter,
            turn: starter,
        }
    }

    pub fn init(&mut self, front: &mut Front) {
        self.trigger(None, front);
    }
    
    // the actual game flow triggered by a cell press
    pub fn trigger(&mut self, pos: Option<CellPos>, front: &mut Front) {
        
        // filter only 3x3 games
        if self.field.len() != 3 || self.field.iter().any(|row| row.len() != 3) {
            panic!("3x3 only.")
        }

        update_front(&self.field, front);

        if self.turn && pos != None {
            let valid = self.human.make_move(pos.unwrap(), &mut self.field);
            update_front(&self.field, front);
            if valid { self.turn = false }
        }
        if self.turn == false {
            let valid = self.ai.make_move(call_minimax(&self.field, self.ai.ptype), &mut self.field);
            update_front(&self.field, front);
            if valid { self.turn = true; }
        }
    }

    // restart whole game
    pub fn restart(&mut self, front: &mut Front) {
        self.field = vec![vec![CellState::Empty; self.props.camount_x as usize]; self.props.camount_y as usize];
        self.turn = self.starter;
        self.init(front);
        update_front(&self.field, front);
    }
}
