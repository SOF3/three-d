use crate::core::*;
use crate::renderer::*;

///
/// Forward render pipeline which can render objects (implementing the [Object] trait).
/// Forward rendering directly draws to the given render target (for example the screen) and is therefore the same as calling [Object::render] directly.
///
pub struct ForwardPipeline {
    context: Context,
}

impl ForwardPipeline {
    ///
    /// Constructor.
    ///
    pub fn new(context: &Context) -> ThreeDResult<Self> {
        Ok(Self {
            context: context.clone(),
        })
    }

    ///
    /// Render the objects with the given surface materials and the given set of lights.
    /// Must be called in a render target render function, for example in the callback function of [Screen::write].
    ///
    #[allow(deprecated)]
    #[deprecated = "Use render_pass instead"]
    pub fn light_pass(
        &self,
        camera: &Camera,
        objects: &[(&dyn ShadedGeometry, &PhysicalMaterial)],
        ambient_light: Option<&AmbientLight>,
        directional_lights: &[&DirectionalLight],
        spot_lights: &[&SpotLight],
        point_lights: &[&PointLight],
    ) -> ThreeDResult<()> {
        for (geo, mat) in objects.iter().filter(|(g, _)| camera.in_frustum(&g.aabb())) {
            geo.render_with_lighting(
                camera,
                mat,
                LightingModel::Blinn,
                ambient_light,
                directional_lights,
                spot_lights,
                point_lights,
            )?;
        }
        Ok(())
    }

    ///
    /// Render the objects. Also avoids rendering objects outside the camera frustum and render the objects in the order given by [cmp_render_order].
    /// Must be called in a render target render function, for example in the callback function of [Screen::write].
    ///
    pub fn render_pass(
        &self,
        camera: &Camera,
        objects: &[&dyn Object],
        lights: &Lights,
    ) -> ThreeDResult<()> {
        render_pass(camera, objects, lights)
    }

    pub fn depth_pass(&self, camera: &Camera, objects: &[&dyn Object]) -> ThreeDResult<()> {
        let depth_material = DepthMaterial {
            render_states: RenderStates {
                write_mask: WriteMask::DEPTH,
                ..Default::default()
            },
            ..Default::default()
        };
        for object in objects
            .iter()
            .filter(|o| !o.is_transparent() && camera.in_frustum(&o.aabb()))
        {
            object.render_forward(&depth_material, camera, &Lights::default())?;
        }
        Ok(())
    }

    pub fn depth_pass_texture(
        &self,
        camera: &Camera,
        objects: &[&dyn Object],
    ) -> ThreeDResult<DepthTargetTexture2D> {
        let depth_texture = DepthTargetTexture2D::new(
            &self.context,
            camera.viewport().width,
            camera.viewport().height,
            Wrapping::ClampToEdge,
            Wrapping::ClampToEdge,
            DepthFormat::Depth32F,
        )?;
        depth_texture.write(Some(1.0), || self.depth_pass(&camera, objects))?;
        Ok(depth_texture)
    }
}
