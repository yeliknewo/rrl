use utils::Player;

#[derive(Debug)]
pub enum FeederToAi {
    WorldState(Player, Vec<f64>),
    Reward(Vec<(Player, i64)>),
    RewardAndEnd(Vec<(Player, i64)>),
}

#[derive(Debug)]
pub enum FeederFromAi {

}
