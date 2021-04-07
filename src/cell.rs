use crate::props::GameProperties;

pub struct Cell {
    pos_x: u32,
    pos_y: u32,

    props: GameProperties,

    state: State,
}

impl Cell {
    pub fn new(pos_x: u32, pos_y: u32, props: GameProperties) -> Cell {
        Cell {
            pos_x,
            pos_y,
            props,
            state: State::Empty,
        }
    }
}

pub enum State {
    Cross,
    Circle,
    Empty,
}
