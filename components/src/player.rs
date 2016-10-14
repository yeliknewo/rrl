use specs::{Component, VecStorage};
use utils::Player;

pub struct CompPlayer {
    player: Player,
}

impl CompPlayer {
    pub fn new(player: Player) -> CompPlayer {
        CompPlayer {
            player: player,
        }
    }

    pub fn get_player(&self) -> Player {
        self.player
    }

    pub fn get_mut_player(&mut self) -> &mut Player {
        &mut self.player
    }
}

impl Component for CompPlayer {
    type Storage = VecStorage<CompPlayer>;
}
