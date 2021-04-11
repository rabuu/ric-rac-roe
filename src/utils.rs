/* UTILS */

// struct for game properties
#[derive(Debug,Clone,Copy)]
pub struct GameProperties {
    pub clen: u32,

    pub bwidth: u32,

    pub camount_x: u32,
    pub camount_y: u32,

    pub winwidth: u32,
    pub winheight: u32,
}

impl GameProperties {
    pub fn new(clen: u32, bwidth: u32, camount_x: u32, camount_y: u32) -> GameProperties {
        GameProperties {
            clen,
            bwidth,
            camount_x,
            camount_y,
            winwidth: ((clen * camount_x) + (bwidth * (camount_x - 1))),
            winheight: ((clen * camount_y) + (bwidth * (camount_y - 1))),
        }
    }
}

// struct for cell position
#[derive(Debug,Clone,Copy,PartialEq)]
pub struct CellPos(pub u32, pub u32);

// struct for absolute coordinates on window
#[derive(Debug,Clone,Copy)]
pub struct Coords(pub f64, pub f64);

// the three states a cell can have
#[derive(Debug,Clone,Copy,PartialEq)]
pub enum CellState {
    Cross,
    Circle,
    Empty,
}

// the two types of players
#[derive(Clone,Copy,PartialEq)]
pub enum PlayerType {
    Human,
    AI,
}
