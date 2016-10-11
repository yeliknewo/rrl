use graphics::GlEncoder;

pub enum MainToRender {
    Encoder(GlEncoder),
}

pub enum MainFromRender {
    Encoder(GlEncoder),
}
