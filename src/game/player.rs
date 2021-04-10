pub struct Player {
    ptype: PlayerType,
}

impl Player {
    pub fn new(ptype: PlayerType) -> Player {
        Player {
            ptype,
        }
    }
}


pub enum PlayerType {
    Human,
    AI,
}
