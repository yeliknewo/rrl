#[macro_use]
extern crate log;
#[macro_use]
pub extern crate gfx;
pub extern crate gfx_device_gl;
#[cfg(feature = "g_glutin")]
pub extern crate glutin;
#[cfg(feature = "g_glutin")]
pub extern crate gfx_window_glutin;
#[cfg(feature = "g_sdl2")]
pub extern crate sdl2;
#[cfg(feature = "g_sdl2")]
pub extern crate gfx_window_sdl;
pub extern crate find_folder;
pub extern crate image;

pub extern crate utils;

pub mod crates {
    pub use ::{find_folder, gfx, gfx_device_gl, image, utils};
    #[cfg(feature = "g_glutin")]
    pub use ::{gfx_window_glutin, glutin};
    #[cfg(feature = "g_sdl2")]
    pub use ::{gfx_window_sdl, sdl2};
    pub use utils::crates::{cgmath, getopts, rustc_serialize};
}

pub use crates::{cgmath, rustc_serialize};

pub mod pipeline;
#[cfg(feature = "g_glutin")]
pub mod rl_glutin;
#[cfg(feature = "g_sdl2")]
pub mod rl_sdl2;
pub mod shaders;
pub mod texture;

pub use pipeline::{Bundle, Packet, ProjectionData, TextureData, Vertex, make_shaders, pipe};
pub use shaders::Shaders;
pub use texture::load_texture;

pub type GlDevice = gfx_device_gl::Device;
pub type GlFactory = gfx_device_gl::Factory;
pub type GlResources = gfx_device_gl::Resources;
pub type GlCommandBuffer = gfx_device_gl::CommandBuffer;
pub type ColorFormat = gfx::format::Srgba8;
pub type DepthFormat = gfx::format::DepthStencil;
pub type OutColor = gfx::handle::RenderTargetView<GlResources, ColorFormat>;
pub type OutDepth = gfx::handle::DepthStencilView<GlResources, DepthFormat>;
pub type GlEncoder = gfx::Encoder<GlResources, GlCommandBuffer>;
pub type GlTexture = gfx::handle::ShaderResourceView<GlResources, [f32; 4]>;
pub type WindowSettings<'a> = (&'a str, u32, u32);

pub struct GfxWindow<W, T> {
    out_color: OutColor,
    out_depth: OutDepth,
    device: GlDevice,
    factory: GlFactory,
    window: W,
    extras: T,
}

#[cfg(feature = "g_glutin")]
impl<T> GfxWindow<glutin::Window, T> {
    pub fn swap_buffers(&mut self) {
        self.get_mut_window().swap_buffers().unwrap_or_else(|err| {
            panic!("{:?}",
                   err)
        });
    }
}

#[cfg(feature = "g_sdl2")]
impl<T> GfxWindow<sdl2::video::Window, T> {
    pub fn swap_buffers(&mut self) {
        self.get_mut_window().gl_swap_window();
    }
}

impl<W, T> GfxWindow<W, T> {
    pub fn new(out_color: OutColor,
               out_depth: OutDepth,
               device: GlDevice,
               factory: GlFactory,
               window: W,
               extras: T)
               -> GfxWindow<W, T> {
        GfxWindow {
            out_color: out_color,
            out_depth: out_depth,
            device: device,
            factory: factory,
            window: window,
            extras: extras,
        }
    }

    pub fn get_out_color(&self) -> &OutColor {
        &self.out_color
    }

    pub fn get_out_depth(&self) -> &OutDepth {
        &self.out_depth
    }

    pub fn get_device(&self) -> &GlDevice {
        &self.device
    }

    pub fn get_factory(&self) -> &GlFactory {
        &self.factory
    }

    pub fn get_window(&self) -> &W {
        &self.window
    }

    pub fn get_extras(&self) -> &T {
        &self.extras
    }

    pub fn get_mut_device(&mut self) -> &mut GlDevice {
        &mut self.device
    }

    pub fn get_mut_factory(&mut self) -> &mut GlFactory {
        &mut self.factory
    }

    pub fn get_mut_window(&mut self) -> &mut W {
        &mut self.window
    }

    pub fn get_mut_extras(&mut self) -> &mut T {
        &mut self.extras
    }
}
