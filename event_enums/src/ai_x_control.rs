use utils::Player;

#[derive(Debug)]
pub enum AiToControl<W> {
    Up(W, Player),
    Down(W, Player),
    Left(W, Player),
    Right(W, Player),
    Joy(W, W, Player),
    A(Player),
    B(Player),
    X(Player),
    Y(Player),
    L1(Player),
    L2(Player),
    R1(Player),
    R2(Player),
}

#[derive(Debug)]
pub enum AiFromControl {

}
