use components::{Gui, RenderData};
use event::BackChannel;
use event_enums::control_x_gui::{ControlFromGui, ControlToGui};
use specs::{Entity, RunArg, System};
use utils::Delta;

pub struct GuiSystem {
    selected: Entity,
    control_back_channel: BackChannel<ControlToGui, ControlFromGui>,
}

impl GuiSystem {
    pub fn new(selected: Entity,
               control_back_channel: BackChannel<ControlToGui, ControlFromGui>)
               -> GuiSystem {
        GuiSystem {
            selected: selected,
            control_back_channel: control_back_channel,
        }
    }
}

impl System<Delta> for GuiSystem {
    fn run(&mut self,
           arg: RunArg,
           _: Delta) {
        let (guis, mut render_datas) = arg.fetch(|w| (w.read::<Gui>(), w.write::<RenderData>()));


    }
}
