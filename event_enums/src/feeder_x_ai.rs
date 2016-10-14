use utils::Player;

#[derive(Debug)]
pub enum FeederToAi<S> {
    WorldState(Player, Vec<S>),
    Reward(Vec<(Player, S)>),
    RewardAndEnd(Vec<(Player, S)>),
}

#[derive(Debug)]
pub enum FeederFromAi {

}
