use specs::{Component, VecStorage};

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct RenderId {
    render_id_num: usize,
}

impl RenderId {
    pub fn new(render_id_num: usize) -> RenderId {
        RenderId {
            render_id_num: render_id_num,
        }
    }

    pub fn get_render_id_num(&self) -> usize {
        self.render_id_num
    }
}

impl Component for RenderId {
    type Storage = VecStorage<RenderId>;
}
