pub struct GameProperties {
    pub cwidth: u32,

    pub camount_x: u32,
    pub camount_y: u32,

    pub bwidth: u32,

    pub winwidth: u32,
    pub winheight: u32,
}

impl GameProperties {
    pub fn new(cwidth: u32, camount_x: u32, camount_y: u32, bwidth: u32) -> GameProperties {
        GameProperties {
            cwidth,
            camount_x,
            camount_y,
            bwidth,
            winwidth: ((cwidth * camount_x) + (bwidth * (camount_x - 1))),
            winheight: ((cwidth * camount_y) + (bwidth * (camount_y - 1))),
        }
    }
}
