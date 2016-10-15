use specs::{Component, Entity, VecStorage};

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Gui {
    left: Option<Entity>,
    right: Option<Entity>,
    up: Option<Entity>,
    down: Option<Entity>,
}

impl Gui {
    pub fn new(left: Option<Entity>,
               right: Option<Entity>,
               up: Option<Entity>,
               down: Option<Entity>)
               -> Gui {
        Gui {
            left: left,
            right: right,
            up: up,
            down: down,
        }
    }

    pub fn get_left(&self) -> &Option<Entity> {
        &self.left
    }

    pub fn get_right(&self) -> &Option<Entity> {
        &self.right
    }

    pub fn get_up(&self) -> &Option<Entity> {
        &self.up
    }

    pub fn get_down(&self) -> &Option<Entity> {
        &self.down
    }

    pub fn get_mut_left(&mut self) -> &mut Option<Entity> {
        &mut self.left
    }

    pub fn get_mut_right(&mut self) -> &mut Option<Entity> {
        &mut self.right
    }

    pub fn get_mut_up(&mut self) -> &mut Option<Entity> {
        &mut self.up
    }

    pub fn get_mut_down(&mut self) -> &mut Option<Entity> {
        &mut self.down
    }
}

impl Component for Gui {
    type Storage = VecStorage<Gui>;
}
