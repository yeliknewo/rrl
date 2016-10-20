use utils::Player;

#[derive(Debug)]
pub enum ToScore {

}

#[derive(Debug)]
pub enum FromScore {

}

#[derive(Debug)]
pub enum ToFeeder {

}

#[derive(Debug)]
pub enum FromFeeder {

}

#[derive(Debug)]
pub enum ToAi<S, W> {
    WorldState(Player, Vec<W>),
    Reward(Vec<(Player, S)>),
    RewardAndEnd(Vec<(Player, S)>),
    Save,
}

pub enum FromAi<F> {
    Up(F, Player),
    Down(F, Player),
    Left(F, Player),
    Right(F, Player),
    Joy(F, F, Player),
    A(Player),
    B(Player),
    X(Player),
    Y(Player),
    L1(Player),
    L2(Player),
    R1(Player),
    R2(Player),
    Start(Player),
    Select(Player),
}
