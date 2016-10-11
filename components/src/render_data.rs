use specs::{Component, VecStorage};

#[derive(Debug)]
pub struct RenderData {
    tint: [f32; 4],
    layer: u8,
    spritesheet_rect: &'static [f32; 4],
    spritesheet_size: &'static [f32; 2],
    mirror_x: bool,
    mirror_y: bool,
    dirty_1: bool,
}

impl RenderData {
    pub fn new(layer: u8,
               tint: [f32; 4],
               spritesheet_rect: &'static [f32; 4],
               spritesheet_size: &'static [f32; 2])
               -> RenderData {
        RenderData {
            tint: tint,
            layer: layer,
            spritesheet_rect: spritesheet_rect,
            spritesheet_size: spritesheet_size,
            mirror_x: false,
            mirror_y: false,
            dirty_1: true,
        }
    }

    pub fn set_layer(&mut self, layer: u8) {
        self.layer = layer;
        self.set_dirty();
    }

    pub fn set_mirrors(&mut self, x: bool, y: bool) {
        self.mirror_x = x;
        self.mirror_y = y;
        self.set_dirty();
    }

    pub fn set_mirror_x(&mut self, mirror: bool) {
        self.mirror_x = mirror;
        self.set_dirty();
    }

    pub fn set_mirror_y(&mut self, mirror: bool) {
        self.mirror_y = mirror;
        self.set_dirty();
    }

    pub fn set_spritesheet_rect(&mut self, spritesheet_rect: &'static [f32; 4]) {
        self.spritesheet_rect = spritesheet_rect;
        self.set_dirty();
    }

    pub fn set_tint(&mut self, tint: [f32; 4]) {
        self.tint = tint;
        self.set_dirty();
    }

    pub fn get_layer(&self) -> u8 {
        self.layer
    }

    pub fn get_mirror_x(&self) -> bool {
        self.mirror_x
    }

    pub fn get_mirror_y(&self) -> bool {
        self.mirror_y
    }

    pub fn get_tint(&self) -> [f32; 4] {
        self.tint.clone()
    }

    pub fn get_spritesheet_rect(&self) -> [f32; 4] {
        self.spritesheet_rect.clone()
    }

    pub fn get_spritesheet_size(&self) -> [f32; 2] {
        self.spritesheet_size.clone()
    }

    fn set_dirty(&mut self) {
        self.dirty_1 = true;
    }

    pub fn take_dirty(&mut self) -> bool {
        if self.dirty_1 {
            self.dirty_1 = false;
            true
        } else {
            false
        }
    }
}

impl Component for RenderData {
    type Storage = VecStorage<RenderData>;
}
