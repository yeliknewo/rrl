use ::GfxCoord;
use cgmath::{Matrix4, Ortho};

#[derive(Clone, Debug)]
pub struct OrthographicHelper {
    aspect_ratio: GfxCoord,
    ortho: Ortho<GfxCoord>,
}

impl OrthographicHelper {
    pub fn new(aspect_ratio: GfxCoord, left: GfxCoord, right: GfxCoord, near: GfxCoord, far: GfxCoord) -> OrthographicHelper {
        let bottom = left * aspect_ratio;
        let top = right * aspect_ratio;

        OrthographicHelper {
            aspect_ratio: aspect_ratio,
            ortho: Ortho {
                left: left,
                right: right,
                bottom: bottom,
                top: top,
                near: near,
                far: far,
            },
        }
    }

    pub fn get_left(&self) -> GfxCoord {
        self.ortho.left
    }

    pub fn get_right(&self) -> GfxCoord {
        self.ortho.right
    }

    pub fn get_bottom(&self) -> GfxCoord {
        self.ortho.bottom
    }

    pub fn get_top(&self) -> GfxCoord {
        self.ortho.top
    }

    pub fn get_near(&self) -> GfxCoord {
        self.ortho.near
    }

    pub fn get_far(&self) -> GfxCoord {
        self.ortho.far
    }

    pub fn get_view_depth(&self) -> GfxCoord {
        self.get_far() - self.get_near()
    }

    pub fn get_aspect_ratio(&self) -> GfxCoord {
        self.aspect_ratio
    }

    pub fn build_matrix(&self) -> Matrix4<GfxCoord> {
        Matrix4::from(self.ortho)
    }
}

impl AsRef<OrthographicHelper> for OrthographicHelper {
    fn as_ref(&self) -> &OrthographicHelper {
        self
    }
}
