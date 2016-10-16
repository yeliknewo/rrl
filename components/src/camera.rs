use cgmath::{Matrix4, Point2, Point3, Vector3};
use specs::{Component, VecStorage};
use utils::{GfxCoord, OrthographicHelper};

#[derive(Debug)]
pub struct Camera {
    eye: Point3<GfxCoord>,
    target: Point3<GfxCoord>,
    up: Vector3<GfxCoord>,
    ortho_helper: OrthographicHelper,
    is_main: bool,
    dirty_1: bool,
    dirty_2: bool,
}

impl Camera {
    pub fn new(eye: Point3<GfxCoord>, target: Point3<GfxCoord>, up: Vector3<GfxCoord>, ortho_helper: OrthographicHelper, is_main: bool) -> Camera {
        Camera {
            eye: eye,
            target: target,
            up: up,
            ortho_helper: ortho_helper,
            is_main: is_main,
            dirty_1: true,
            dirty_2: true,
        }
    }

    pub fn set_offset(&mut self, offset: Point2<GfxCoord>) {
        self.set_eye(Point3::new(offset.x, offset.y, 2.0));
        self.set_target(Point3::new(offset.x, offset.y, 0.0));
        self.set_dirty();
    }

    fn set_eye(&mut self, eye: Point3<GfxCoord>) {
        self.eye = eye;
    }

    fn set_target(&mut self, target: Point3<GfxCoord>) {
        self.target = target;
    }

    fn get_eye(&self) -> Point3<GfxCoord> {
        self.eye
    }

    fn get_target(&self) -> Point3<GfxCoord> {
        self.target
    }

    fn get_up(&self) -> Vector3<GfxCoord> {
        self.up
    }

    pub fn set_proj(&mut self, ortho_helper: OrthographicHelper) {
        self.ortho_helper = ortho_helper;
        self.set_dirty();
    }

    pub fn get_offset(&self) -> Point2<GfxCoord> {
        Point2::new(self.get_eye().x, self.get_eye().y)
    }

    pub fn get_view(&self) -> Matrix4<GfxCoord> {
        Matrix4::look_at(self.get_eye(), self.get_target(), self.get_up())
    }

    pub fn get_proj(&self) -> Matrix4<GfxCoord> {
        self.ortho_helper.build_matrix()
    }

    pub fn is_main(&self) -> bool {
        self.is_main
    }

    pub fn screen_to_world_point(&self, screen_point: Point2<GfxCoord>) -> Point2<GfxCoord> {
        let view_depth = self.ortho_helper.get_view_depth();

        let world_point = Point2::new((((screen_point.x * 2.0) - 1.0) * view_depth) * 4.0 / 5.0 + self.get_offset().x,
                                      (((1.0 - screen_point.y) * 2.0 - 1.0) * view_depth / self.ortho_helper.get_aspect_ratio()) * 4.0 / 5.0 + self.get_offset().y);

        world_point
    }

    fn set_dirty(&mut self) {
        self.dirty_1 = true;
        self.dirty_2 = true;
    }

    pub fn take_dirty(&mut self) -> bool {
        self.dirty_1 = false;
        if self.dirty_2 {
            self.dirty_2 = false;
            return true;
        } else {
            return false;
        }
    }
}

impl Component for Camera {
    type Storage = VecStorage<Camera>;
}
