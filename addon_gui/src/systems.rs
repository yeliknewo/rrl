use base_comps::RenderData;
use components::Gui;
use event_core::BackChannel;
use events::{FromGui, ToGui};
use specs::{Allocator, Entity, MaskedStorage, RunArg, Storage, System};
use std::ops::{Deref, DerefMut};
use utils::Delta;

pub struct GuiSystem {
    selected: Entity,
    control_back_channel: BackChannel<ToGui, FromGui>,
}

impl GuiSystem {
    pub fn new(selected: Entity, control_back_channel: BackChannel<ToGui, FromGui>) -> GuiSystem {
        GuiSystem {
            selected: selected,
            control_back_channel: control_back_channel,
        }
    }

    fn left_event<T1: Deref<Target = Allocator>, T2: Deref<Target = MaskedStorage<Gui>>, T3: DerefMut<Target = MaskedStorage<RenderData>>>(&mut self, guis: &Storage<Gui, T1, T2>, render_datas: &mut Storage<RenderData, T1, T3>) {
        let gui = guis.get(self.selected).unwrap_or_else(|| panic!("Selected had no gui"));

        if let &Some(left) = gui.get_left() {
            render_datas.get_mut(left).unwrap_or_else(|| panic!("Left had no render data")).set_tint([1.0, 1.0, 1.0, 1.0]);
            render_datas.get_mut(self.selected).unwrap_or_else(|| panic!("Selected had no render data")).reset_tint();
            self.selected = left;
        }
    }

    fn right_event<T1: Deref<Target = Allocator>, T2: Deref<Target = MaskedStorage<Gui>>, T3: DerefMut<Target = MaskedStorage<RenderData>>>(&mut self, guis: &Storage<Gui, T1, T2>, render_datas: &mut Storage<RenderData, T1, T3>) {
        let gui = guis.get(self.selected).unwrap_or_else(|| panic!("Selected had no gui"));

        if let &Some(right) = gui.get_right() {
            render_datas.get_mut(right).unwrap_or_else(|| panic!("Right had no render data")).set_tint([1.0, 1.0, 1.0, 1.0]);
            render_datas.get_mut(self.selected).unwrap_or_else(|| panic!("Selected had no render data")).reset_tint();
            self.selected = right;
        }
    }

    fn up_event<T1: Deref<Target = Allocator>, T2: Deref<Target = MaskedStorage<Gui>>, T3: DerefMut<Target = MaskedStorage<RenderData>>>(&mut self, guis: &Storage<Gui, T1, T2>, render_datas: &mut Storage<RenderData, T1, T3>) {
        let gui = guis.get(self.selected).unwrap_or_else(|| panic!("Selected had no gui"));

        if let &Some(up) = gui.get_up() {
            render_datas.get_mut(up).unwrap_or_else(|| panic!("Up had no render data")).set_tint([1.0, 1.0, 1.0, 1.0]);
            render_datas.get_mut(self.selected).unwrap_or_else(|| panic!("Selected had no render data")).reset_tint();
            self.selected = up;
        }
    }

    fn down_event<T1: Deref<Target = Allocator>, T2: Deref<Target = MaskedStorage<Gui>>, T3: DerefMut<Target = MaskedStorage<RenderData>>>(&mut self, guis: &Storage<Gui, T1, T2>, render_datas: &mut Storage<RenderData, T1, T3>) {
        let gui = guis.get(self.selected).unwrap_or_else(|| panic!("Selected had no gui"));

        if let &Some(down) = gui.get_down() {
            render_datas.get_mut(down).unwrap_or_else(|| panic!("Down had no render data")).set_tint([1.0, 1.0, 1.0, 1.0]);
            render_datas.get_mut(self.selected).unwrap_or_else(|| panic!("Selected had no render data")).reset_tint();
            self.selected = down;
        }
    }

    fn select_event<T1: Deref<Target = Allocator>, T2: Deref<Target = MaskedStorage<Gui>>, T3: DerefMut<Target = MaskedStorage<RenderData>>>(&mut self, guis: &Storage<Gui, T1, T2>, render_datas: &mut Storage<RenderData, T1, T3>) {

    }

    fn cancel_event<T1: Deref<Target = Allocator>, T2: Deref<Target = MaskedStorage<Gui>>, T3: DerefMut<Target = MaskedStorage<RenderData>>>(&mut self, guis: &Storage<Gui, T1, T2>, render_datas: &mut Storage<RenderData, T1, T3>) {

    }
}

impl System<Delta> for GuiSystem {
    fn run(&mut self, arg: RunArg, _: Delta) {
        let (guis, mut render_datas) = arg.fetch(|w| (w.read::<Gui>(), w.write::<RenderData>()));

        while let Some(event) = self.control_back_channel.try_recv_to() {
            match event {
                ToGui::Down(_amount, _player) => self.down_event(&guis, &mut render_datas),
                ToGui::Up(_amount, _player) => self.up_event(&guis, &mut render_datas),
                ToGui::Left(_amount, _player) => self.left_event(&guis, &mut render_datas),
                ToGui::Right(_amount, _player) => self.right_event(&guis, &mut render_datas),
                ToGui::Select(_amount, _player) => self.select_event(&guis, &mut render_datas),
                ToGui::Cancel(_amount, _player) => self.cancel_event(&guis, &mut render_datas),
            }
        }
    }
}
