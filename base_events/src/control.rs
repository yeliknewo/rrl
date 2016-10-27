use utils::Player;

#[derive(Debug, Clone)]
pub enum ToControl<C: Clone> {
    JoyX(C, Player, bool),
    JoyY(C, Player, bool),
}

#[derive(Debug, Clone)]
pub enum FromControl<C: Clone> {
    Save,
    Joy(Option<C>, Option<C>, Player),
    Up(C, Player),
    Down(C, Player),
    Right(C, Player),
    Left(C, Player),
    A(Player),
    B(Player),
    X(Player),
    Y(Player),
    L1(Player),
    L2(Player),
    R1(Player),
    R2(Player),
}

impl<'a, C> From<&'a ToControl<C>> for FromControl<C>
    where C: Clone
{
    fn from(other: &'a ToControl<C>) -> FromControl<C> {
        match other {
            &ToControl::JoyX(ref x, ref player, _) => FromControl::Joy(Some(x.clone()), None, player.clone()),
            &ToControl::JoyY(ref y, ref player, _) => FromControl::Joy(None, Some(y.clone()), player.clone()),
        }
    }
}
