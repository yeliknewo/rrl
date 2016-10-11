use utils::Player;

#[derive(Debug, Clone)]
pub enum ControlToPlayer {
    Up(f64, Player),
    Down(f64, Player),
    Left(f64, Player),
    Right(f64, Player),
    Joy(f64, f64, Player),
}

#[derive(Debug)]
pub enum ControlFromPlayer {

}
