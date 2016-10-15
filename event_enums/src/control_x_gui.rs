use ::control_x_player::ControlToPlayer;
use ::main_x_control::MainToControl;
use utils::Player;

#[derive(Debug, Clone)]
pub enum ControlToGui<W> {
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

impl<W> From<ControlToPlayer<W>> for ControlToGui<W> {
    fn from(other: ControlToPlayer<W>) -> ControlToGui<W> {
        match other {
            ControlToPlayer::Up(amount, player) => {
                ControlToGui::Up(amount,
                                 player)
            }
            ControlToPlayer::Down(amount, player) => {
                ControlToGui::Down(amount,
                                   player)
            }
            ControlToPlayer::Left(amount, player) => {
                ControlToGui::Left(amount,
                                   player)
            }
            ControlToPlayer::Right(amount, player) => {
                ControlToGui::Right(amount,
                                    player)
            }
            ControlToPlayer::Joy(x, y, player) => {
                ControlToGui::Joy(x,
                                  y,
                                  player)
            }
            ControlToPlayer::A(player) => ControlToGui::A(player),
            ControlToPlayer::B(player) => ControlToGui::B(player),
            ControlToPlayer::X(player) => ControlToGui::X(player),
            ControlToPlayer::Y(player) => ControlToGui::Y(player),
            ControlToPlayer::L1(player) => ControlToGui::L1(player),
            ControlToPlayer::L2(player) => ControlToGui::L2(player),
            ControlToPlayer::R1(player) => ControlToGui::R1(player),
            ControlToPlayer::R2(player) => ControlToGui::R2(player),
        }
    }
}

impl<W> From<MainToControl<W>> for ControlToGui<W> {
    fn from(other: MainToControl<W>) -> ControlToGui<W> {
        match other {
            MainToControl::Up(amount, player) => {
                ControlToGui::Up(amount,
                                 player)
            }
            MainToControl::Down(amount, player) => {
                ControlToGui::Down(amount,
                                   player)
            }
            MainToControl::Left(amount, player) => {
                ControlToGui::Left(amount,
                                   player)
            }
            MainToControl::Right(amount, player) => {
                ControlToGui::Right(amount,
                                    player)
            }
            MainToControl::JoyX(x, player) => {
                ControlToGui::Joy(Some(x),
                                  None,
                                  player)
            }
            MainToControl::JoyY(y, player) => {
                ControlToGui::Joy(None,
                                  Some(y),
                                  player)
            }
            MainToControl::A(player) => ControlToGui::A(player),
            MainToControl::B(player) => ControlToGui::B(player),
            MainToControl::X(player) => ControlToGui::X(player),
            MainToControl::Y(player) => ControlToGui::Y(player),
            MainToControl::L1(player) => ControlToGui::L1(player),
            MainToControl::L2(player) => ControlToGui::L2(player),
            MainToControl::R1(player) => ControlToGui::R1(player),
            MainToControl::R2(player) => ControlToGui::R2(player),
        }
    }
}


#[derive(Debug)]
pub enum ControlFromGui {

}
