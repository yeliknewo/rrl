use components::{Gui, RenderData};
use event::BackChannel;
use event_enums::control_x_gui::{ControlFromGui, ControlToGui};
use specs::{Allocator, Entity, MaskedStorage, RunArg, Storage, System};
use std::ops::{Deref, DerefMut};
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

    fn left_event<T1: Deref<Target = Allocator>, T2: Deref<Target = MaskedStorage<Gui>>, T3: DerefMut<Target = MaskedStorage<RenderData>>>(&mut self,
                                                                                                                                           guis: &Storage<Gui, T1, T2>,
                                                                                                                                           render_datas: &mut Storage<RenderData, T1, T3>) {
        let gui = guis.get(self.selected).unwrap_or_else(|| panic!("Selected had no gui"));

        if let &Some(left) = gui.get_left() {
            render_datas.get_mut(left).unwrap_or_else(|| panic!("Left had no render data")).set_tint([1.0, 1.0, 1.0, 1.0]);
            render_datas.get_mut(self.selected).unwrap_or_else(|| panic!("Selected had no render data")).reset_tint();
            self.selected = left;
        }
    }

    fn right_event<T1: Deref<Target = Allocator>, T2: Deref<Target = MaskedStorage<Gui>>, T3: DerefMut<Target = MaskedStorage<RenderData>>>(&mut self,
                                                                                                                                            guis: &Storage<Gui, T1, T2>,
                                                                                                                                            render_datas: &mut Storage<RenderData, T1, T3>) {
        let gui = guis.get(self.selected).unwrap_or_else(|| panic!("Selected had no gui"));

        if let &Some(right) = gui.get_right() {
            render_datas.get_mut(right).unwrap_or_else(|| panic!("Right had no render data")).set_tint([1.0, 1.0, 1.0, 1.0]);
            render_datas.get_mut(self.selected).unwrap_or_else(|| panic!("Selected had no render data")).reset_tint();
            self.selected = right;
        }
    }

    fn up_event<T1: Deref<Target = Allocator>, T2: Deref<Target = MaskedStorage<Gui>>, T3: DerefMut<Target = MaskedStorage<RenderData>>>(&mut self,
                                                                                                                                         guis: &Storage<Gui, T1, T2>,
                                                                                                                                         render_datas: &mut Storage<RenderData, T1, T3>) {
        let gui = guis.get(self.selected).unwrap_or_else(|| panic!("Selected had no gui"));

        if let &Some(up) = gui.get_up() {
            render_datas.get_mut(up).unwrap_or_else(|| panic!("Up had no render data")).set_tint([1.0, 1.0, 1.0, 1.0]);
            render_datas.get_mut(self.selected).unwrap_or_else(|| panic!("Selected had no render data")).reset_tint();
            self.selected = up;
        }
    }

    fn down_event<T1: Deref<Target = Allocator>, T2: Deref<Target = MaskedStorage<Gui>>, T3: DerefMut<Target = MaskedStorage<RenderData>>>(&mut self,
                                                                                                                                           guis: &Storage<Gui, T1, T2>,
                                                                                                                                           render_datas: &mut Storage<RenderData, T1, T3>) {
        let gui = guis.get(self.selected).unwrap_or_else(|| panic!("Selected had no gui"));

        if let &Some(down) = gui.get_down() {
            render_datas.get_mut(down).unwrap_or_else(|| panic!("Down had no render data")).set_tint([1.0, 1.0, 1.0, 1.0]);
            render_datas.get_mut(self.selected).unwrap_or_else(|| panic!("Selected had no render data")).reset_tint();
            self.selected = down;
        }
    }
}

impl System<Delta> for GuiSystem {
    fn run(&mut self,
           arg: RunArg,
           _: Delta) {
        let (guis, mut render_datas) = arg.fetch(|w| (w.read::<Gui>(), w.write::<RenderData>()));

        while let Some(event) = self.control_back_channel.try_recv_to() {
            match event {
                ControlToGui::Down(_amount, _player) => {
                    self.down_event(&guis,
                                    &mut render_datas)
                }
                ControlToGui::Up(_amount, _player) => {
                    self.up_event(&guis,
                                  &mut render_datas)
                }
                ControlToGui::Left(_amount, _player) => {
                    self.left_event(&guis,
                                    &mut render_datas)
                }
                ControlToGui::Right(_amount, _player) => {
                    self.right_event(&guis,
                                     &mut render_datas)
                }
                ControlToGui::Joy(x, y, _player) => {
                    if x.abs() > y.abs() {
                        if x > 0.0 {
                            self.right_event(&guis,
                                             &mut render_datas);
                        } else {
                            self.left_event(&guis,
                                            &mut render_datas);
                        }
                    } else {
                        if y > 0.0 {
                            self.up_event(&guis,
                                          &mut render_datas);
                        } else {
                            self.down_event(&guis,
                                            &mut render_datas);
                        }
                    }
                }
            }
        }
    }
}
