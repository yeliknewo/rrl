use components::{Camera, RenderData, RenderId, Transform};
use event::BackChannel;
use event_enums::main_x_render::{MainFromRender, MainToRender};
use gfx::Primitive;
use gfx::tex::{FilterMethod, SamplerInfo, WrapMode};
use gfx::traits::{Factory, FactoryExt};
use graphics::{Bundle, GlEncoder, GlFactory, GlTexture, OutColor, OutDepth, Packet, ProjectionData, Shaders, TextureData, make_shaders, pipe};
use specs::{RunArg, System};
use std::sync::Arc;
use utils::Delta;

pub struct RenderSystem {
    back_channel: BackChannel<MainToRender, MainFromRender>,
    out_color: OutColor,
    out_depth: OutDepth,
    bundles: Arc<Vec<Bundle>>,
    shaders: Shaders,
}

impl RenderSystem {
    pub fn new(back_channel: BackChannel<MainToRender, MainFromRender>, out_color: OutColor, out_depth: OutDepth) -> RenderSystem {
        RenderSystem {
            back_channel: back_channel,
            out_color: out_color,
            out_depth: out_depth,
            bundles: Arc::new(vec![]),
            shaders: make_shaders(),
        }
    }

    pub fn add_render(&mut self, factory: &mut GlFactory, packet: &Packet, texture: GlTexture) -> RenderId {
        let shader_set = factory.create_shader_set(self.shaders.get_vertex_shader(), self.shaders.get_fragment_shader())
            .unwrap_or_else(|err| panic!("Create Shader Set Error: {:?}", err));

        let program = factory.create_program(&shader_set)
            .unwrap_or_else(|err| panic!("Create Program Error: {:?}", err));

        let pso = factory.create_pipeline_from_program(&program, Primitive::TriangleList, packet.get_rasterizer(), pipe::new())
            .unwrap_or_else(|err| panic!("Create Pipeline from Program Error: {:?}", err));

        let sampler_info = SamplerInfo::new(FilterMethod::Scale, WrapMode::Mirror);

        let (vbuf, slice) = factory.create_vertex_buffer_with_slice(packet.get_vertices(), packet.get_indices());

        let data = pipe::Data {
            vbuf: vbuf,
            spritesheet: (texture, factory.create_sampler(sampler_info)),
            texture_data: factory.create_constant_buffer(1),
            projection_data: factory.create_constant_buffer(1),
            out_color: self.out_color.clone(),
            out_depth: self.out_depth.clone(),
        };

        let mut bundles = Arc::get_mut(&mut self.bundles).unwrap_or_else(|| panic!("Arc Shit"));

        let id = bundles.len();

        bundles.push(Bundle::new(slice, pso, data));

        RenderId::new(id)
    }

    fn render(&mut self, arg: &RunArg, mut encoder: GlEncoder) {
        use specs::Join;

        let (render_ids, transforms, cameras, mut render_datas) = arg.fetch(|w| (w.read::<RenderId>(), w.read::<Transform>(), w.read::<Camera>(), w.write::<RenderData>()));

        encoder.clear(&self.out_color, [0.0, 0.0, 0.0, 1.0]);
        encoder.clear_depth(&self.out_depth, 1.0);

        let (view, proj) = {
            let camera = {
                let mut camera_opt = None;

                for camera in (&cameras).iter() {
                    if camera.is_main() {
                        camera_opt = Some(camera);
                    }
                }

                camera_opt.unwrap_or_else(|| panic!("No Main Camera Entity"))
            };

            (camera.get_view(), camera.get_proj())
        };

        let mut datas = vec![];

        for (render_id, transform, render_data) in (&render_ids, &transforms, &mut render_datas).iter() {
            let mut projection_data = None;

            if true {
                // dirty_cam || transform.take_dirty() {
                projection_data = Some(ProjectionData {
                    model: transform.get_model().into(),
                    view: view.into(),
                    proj: proj.into(),
                });
            }

            let mut texture_data = None;

            if true {
                // render_data.take_dirty() {
                texture_data = Some(TextureData {
                    tint: render_data.get_tint(),
                    spritesheet_rect: render_data.get_spritesheet_rect(),
                    spritesheet_size: render_data.get_spritesheet_size(),
                    mirror_x: render_data.get_mirror_x(),
                    mirror_y: render_data.get_mirror_y(),
                });
            }

            datas.push((render_id.get_render_id_num(), render_data.get_layer(), texture_data, projection_data));
        }

        datas.sort_by_key(|k| k.1);

        for data in datas {
            let b = self.bundles.get(data.0).unwrap_or_else(|| panic!("No Bundle found"));

            if let Some(texture_data) = data.2 {
                encoder.update_constant_buffer(&b.get_data().texture_data, &texture_data);
            }

            if let Some(projection_data) = data.3 {
                encoder.update_constant_buffer(&b.get_data().projection_data, &projection_data);
            }

            b.encode(&mut encoder);
        }

        self.back_channel.send_from(MainFromRender::Encoder(encoder));
    }

    fn process_event(&mut self, arg: &RunArg, event: MainToRender) -> bool {
        match event {
            MainToRender::Encoder(encoder) => {
                self.render(arg, encoder);
                false
            }
        }
    }
}

impl System<Delta> for RenderSystem {
    fn run(&mut self, arg: RunArg, _: Delta) {
        let mut event = self.back_channel.try_recv_to();
        while self.process_event(&arg,
                                 match event {
                                     Some(event) => event,
                                     None => {
                                         arg.fetch(|_| {});
                                         return;
                                     }
                                 }) {
            event = self.back_channel.try_recv_to();
        }
    }
}
