use utils::Player;

#[derive(Debug)]
pub enum ScoreToFeeder<S> {
    Lose(Player, S, S),
    LoseBoth(S, S),
}

#[derive(Debug)]
pub enum ScoreFromFeeder {

}
