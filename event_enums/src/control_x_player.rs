use ::ai_x_control::AiToControl;
use ::main_x_control::MainToControl;
use utils::Player;

#[derive(Debug, Clone)]
pub enum ControlToPlayer<W> {
    Up(W, Player),
    Down(W, Player),
    Left(W, Player),
    Right(W, Player),
    Joy(Option<W>, Option<W>, Player),
    A(Player),
    B(Player),
    X(Player),
    Y(Player),
    L1(Player),
    L2(Player),
    R1(Player),
    R2(Player),
}

impl<W> From<MainToControl<W>> for ControlToPlayer<W> {
    fn from(other: MainToControl<W>) -> ControlToPlayer<W> {
        match other {
            MainToControl::Up(amount, player) => {
                ControlToPlayer::Up(amount,
                                    player)
            }
            MainToControl::Down(amount, player) => {
                ControlToPlayer::Down(amount,
                                      player)
            }
            MainToControl::Left(amount, player) => {
                ControlToPlayer::Left(amount,
                                      player)
            }
            MainToControl::Right(amount, player) => {
                ControlToPlayer::Right(amount,
                                       player)
            }
            MainToControl::JoyX(x, player) => {
                ControlToPlayer::Joy(Some(x),
                                     None,
                                     player)
            }
            MainToControl::JoyY(y, player) => {
                ControlToPlayer::Joy(None,
                                     Some(y),
                                     player)
            }
            MainToControl::A(player) => ControlToPlayer::A(player),
            MainToControl::B(player) => ControlToPlayer::B(player),
            MainToControl::X(player) => ControlToPlayer::X(player),
            MainToControl::Y(player) => ControlToPlayer::Y(player),
            MainToControl::L1(player) => ControlToPlayer::L1(player),
            MainToControl::L2(player) => ControlToPlayer::L2(player),
            MainToControl::R1(player) => ControlToPlayer::R1(player),
            MainToControl::R2(player) => ControlToPlayer::R2(player),
        }
    }
}

impl<W> From<AiToControl<W>> for ControlToPlayer<W> {
    fn from(other: AiToControl<W>) -> ControlToPlayer<W> {
        match other {
            AiToControl::Up(amount, player) => {
                ControlToPlayer::Up(amount,
                                    player)
            }
            AiToControl::Down(amount, player) => {
                ControlToPlayer::Down(amount,
                                      player)
            }
            AiToControl::Left(amount, player) => {
                ControlToPlayer::Left(amount,
                                      player)
            }
            AiToControl::Right(amount, player) => {
                ControlToPlayer::Right(amount,
                                       player)
            }
            AiToControl::Joy(x, y, player) => {
                ControlToPlayer::Joy(Some(x),
                                     Some(y),
                                     player)
            }
            AiToControl::A(player) => ControlToPlayer::A(player),
            AiToControl::B(player) => ControlToPlayer::B(player),
            AiToControl::X(player) => ControlToPlayer::X(player),
            AiToControl::Y(player) => ControlToPlayer::Y(player),
            AiToControl::L1(player) => ControlToPlayer::L1(player),
            AiToControl::L2(player) => ControlToPlayer::L2(player),
            AiToControl::R1(player) => ControlToPlayer::R1(player),
            AiToControl::R2(player) => ControlToPlayer::R2(player),
        }
    }
}

#[derive(Debug)]
pub enum ControlFromPlayer {

}
