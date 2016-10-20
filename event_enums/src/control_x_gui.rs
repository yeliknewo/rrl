use utils::Player;

#[derive(Debug, Clone)]
pub enum ControlToGui {
    Up(bool, Player),
    Down(bool, Player),
    Left(bool, Player),
    Right(bool, Player),
    Select(bool, Player),
    Cancel(bool, Player),
}

#[derive(Debug)]
pub enum ControlFromGui {

}
