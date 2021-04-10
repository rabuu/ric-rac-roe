use crate::{game::player::{Player, PlayerType}, gui::front::Front, utils::{CellPos, CellState, GameProperties}};
use crate::bridge::update_front;

mod player;

pub struct Game {
    props: GameProperties,

    p1: Player,
    p2: Player,

    field: Vec<Vec<CellState>>,
}

impl Game {
    pub fn new(props: GameProperties) -> Game {
        Game {
            props,
            p1: Player::new(PlayerType::Human),
            p2: Player::new(PlayerType::AI),
            field: vec![vec![CellState::Empty; props.camount_x as usize]; props.camount_y as usize],
        }
    }

    pub fn cell_pressed(&mut self, pos: CellPos, front: &mut Front) {
        self.field[pos.x as usize][pos.y as usize] = CellState::Cross;
        update_front(&self.field, front);
    }
}
