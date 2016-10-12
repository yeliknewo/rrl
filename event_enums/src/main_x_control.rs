use utils::Player;

#[derive(Debug)]
pub enum MainToControl {
    Up(f64, Player),
    Down(f64, Player),
    Left(f64, Player),
    Right(f64, Player),
    JoyX(f64, Player),
    JoyY(f64, Player),
}

#[derive(Debug)]
pub enum MainFromControl {
    Save,
}
