use utils::Player;

#[derive(Debug)]
pub enum ScoreToFeeder {
    Lose(Player, f64),
    LoseBoth,
}

#[derive(Debug)]
pub enum ScoreFromFeeder {

}
