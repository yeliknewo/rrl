#[derive(Debug)]
pub enum FeederToAi<S, W> {
    WorldState(Player, Vec<W>),
    Reward(Vec<(Player, S)>),
    RewardAndEnd(Vec<(Player, S)>),
}

#[derive(Debug)]
pub enum FeederFromAi {

}
