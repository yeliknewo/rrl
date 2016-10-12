use utils::Player;

#[derive(Debug)]
pub enum ScoreToFeeder {
    Lose(Player, f64, f64),
    LoseBoth(f64, f64),
}

#[derive(Debug)]
pub enum ScoreFromFeeder {

}
