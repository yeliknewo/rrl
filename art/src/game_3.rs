pub mod layers {
    use ::Layer;

    pub const TILES: Layer = 0;
    pub const PLAYER: Layer = 1;
}

pub mod main {
    use ::{Name, Size, Tint, RenderType, Sprite};

    pub const NAME: Name = "main.png";
    pub const SIZE: Size = &[256.0, 256.0];
    pub const DEFAULT_TINT: Tint = &[0.5, 0.5, 0.5, 1.0];
    pub const ID: RenderType = 0;

    pub const PLAYER_1_STAND: Sprite = &[0.0, 0.0, 32.0, 31.5];
    pub const TEST: Sprite = &[0.0, 0.0, 1.0, 0.5];
}
