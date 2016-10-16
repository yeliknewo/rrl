pub mod layers {
    use ::Layer;

    pub const GUI: Layer = 0;
}

pub mod main {
    use ::{Name, RenderType, Size, Sprite, Tint};

    pub const NAME: Name = "main.png";
    pub const SIZE: Size = &[256.0, 256.0];
    pub const DEFAULT_TINT: Tint = &[0.5, 0.5, 0.5, 1.0];
    pub const ID: RenderType = 0;

    pub const BOX: Sprite = &[32.0, 32.0, 32.0, 31.5];
    pub const TEST: Sprite = &[0.0, 0.0, 1.0, 0.5];
}
