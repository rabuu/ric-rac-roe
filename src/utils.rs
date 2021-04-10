#[derive(Debug,Clone,Copy)]
pub struct GameProperties {
    pub clen: u32,

    pub camount_x: u32,
    pub camount_y: u32,

    pub bwidth: u32,

    pub winwidth: u32,
    pub winheight: u32,
}

impl GameProperties {
    pub fn new(clen: u32, camount_x: u32, camount_y: u32, bwidth: u32) -> GameProperties {
        GameProperties {
            clen,
            camount_x,
            camount_y,
            bwidth,
            winwidth: ((clen * camount_x) + (bwidth * (camount_x - 1))),
            winheight: ((clen * camount_y) + (bwidth * (camount_y - 1))),
        }
    }
}

#[derive(Debug,Clone,Copy)]
pub struct CellPos {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug,Clone,Copy)]
pub struct Coords {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug,Clone,Copy)]
pub enum CellState {
    Cross,
    Circle,
    Empty,
}
