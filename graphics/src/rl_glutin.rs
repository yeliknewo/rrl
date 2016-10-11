use glutin::{self, WindowBuilder};
use gfx_window_glutin;

use super::{WindowSettings, GfxWindow};

pub type Window = glutin::Window;
pub type Extras = ();

pub fn build_window(window_settings: WindowSettings) -> GfxWindow<Window, Extras> {
    let (title, width, height) = window_settings;

    let builder = WindowBuilder::new()
        .with_title(title)
        .with_dimensions(width, height)
        .with_vsync();

    let (window, device, factory, out_color, out_depth) = gfx_window_glutin::init(builder);

    GfxWindow::new(out_color, out_depth, device, factory, window, ())
}
