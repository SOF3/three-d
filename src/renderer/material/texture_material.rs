use crate::core::*;
use crate::renderer::*;
use std::rc::Rc;

#[derive(Clone)]
pub struct TextureMaterial {
    pub texture: Rc<Texture2D>,
}

impl Paint for TextureMaterial {
    fn fragment_shader_source(
        &self,
        _ambient_light: Option<&AmbientLight>,
        _directional_lights: &[&DirectionalLight],
        _spot_lights: &[&SpotLight],
        _point_lights: &[&PointLight],
    ) -> String {
        include_str!("../object/shaders/mesh_texture.frag").to_owned()
    }
    fn bind(
        &self,
        program: &Program,
        _camera: &Camera,
        _ambient_light: Option<&AmbientLight>,
        _directional_lights: &[&DirectionalLight],
        _spot_lights: &[&SpotLight],
        _point_lights: &[&PointLight],
    ) -> Result<()> {
        program.use_texture("tex", &*self.texture)
    }
    fn render_states(&self) -> RenderStates {
        if self.texture.is_transparent() {
            RenderStates {
                write_mask: WriteMask::COLOR,
                blend: Blend::TRANSPARENCY,
                ..Default::default()
            }
        } else {
            RenderStates::default()
        }
    }
}
