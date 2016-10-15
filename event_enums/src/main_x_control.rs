use utils::Player;

#[derive(Debug)]
pub enum MainToControl<W> {
    Up(W, Player),
    Down(W, Player),
    Left(W, Player),
    Right(W, Player),
    JoyX(W, Player),
    JoyY(W, Player),
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
pub enum MainFromControl {
    Save,
}
