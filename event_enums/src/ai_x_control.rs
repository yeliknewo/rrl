use utils::Player;

#[derive(Debug)]
pub enum AiToControl<W> {
    Up(W, Player),
    Down(W, Player),
    Left(W, Player),
    Right(W, Player),
    Joy(W, W, Player),
}

#[derive(Debug)]
pub enum AiFromControl {

}
