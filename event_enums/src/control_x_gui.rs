use ::control_x_player::ControlToPlayer;
use utils::Player;

#[derive(Debug, Clone)]
pub enum ControlToGui {
    Up(f64, Player),
    Down(f64, Player),
    Left(f64, Player),
    Right(f64, Player),
    Joy(f64, f64, Player),
}

impl From<ControlToPlayer> for ControlToGui {
    fn from(other: ControlToPlayer) -> ControlToGui {
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
        }
    }
}

#[derive(Debug)]
pub enum ControlFromGui {

}
