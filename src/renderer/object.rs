//!
//! A collection of objects that can be rendered, for example a mesh.
//!

pub use crate::core::{AxisAlignedBoundingBox, CPUMesh};

mod model;
#[doc(inline)]
pub use model::*;

mod instanced_model;
#[doc(inline)]
pub use instanced_model::*;

mod line;
#[doc(inline)]
pub use line::*;

mod rectangle;
#[doc(inline)]
pub use rectangle::*;

mod circle;
#[doc(inline)]
pub use circle::*;

mod skybox;
#[doc(inline)]
pub use skybox::*;

mod imposters;
#[doc(inline)]
pub use imposters::*;

mod axes;
#[doc(inline)]
pub use axes::*;

mod particles;
#[doc(inline)]
pub use particles::*;

use crate::core::*;
use crate::renderer::*;

pub struct Glue<G: Geometry, M: ForwardMaterial> {
    pub geometry: G,
    pub material: M,
}

impl<G: Geometry, M: ForwardMaterial> Drawable for Glue<G, M> {
    fn render(&self, camera: &Camera, lights: &Lights) -> Result<()> {
        self.geometry.render_forward(&self.material, camera, lights)
    }
}

impl<G: Geometry, M: ForwardMaterial> Drawable for &Glue<G, M> {
    fn render(&self, camera: &Camera, lights: &Lights) -> Result<()> {
        (*self).render(camera, lights)
    }
}

impl<G: Geometry, M: ForwardMaterial> Cullable for Glue<G, M> {
    fn in_frustum(&self, camera: &Camera) -> bool {
        self.geometry.in_frustum(camera)
    }
}

impl<G: Geometry, M: ForwardMaterial> Cullable for &Glue<G, M> {
    fn in_frustum(&self, camera: &Camera) -> bool {
        (*self).in_frustum(camera)
    }
}

impl<G: Geometry, M: ForwardMaterial> Shadable for Glue<G, M> {
    fn render_forward(
        &self,
        material: &dyn ForwardMaterial,
        camera: &Camera,
        lights: &Lights,
    ) -> Result<()> {
        self.geometry.render_forward(material, camera, lights)
    }

    fn render_deferred(
        &self,
        material: &dyn DeferredMaterial,
        camera: &Camera,
        viewport: Viewport,
    ) -> Result<()> {
        self.geometry.render_deferred(material, camera, viewport)
    }
}

impl<G: Geometry, M: ForwardMaterial> Shadable for &Glue<G, M> {
    fn render_forward(
        &self,
        material: &dyn ForwardMaterial,
        camera: &Camera,
        lights: &Lights,
    ) -> Result<()> {
        (*self).render_forward(material, camera, lights)
    }

    fn render_deferred(
        &self,
        material: &dyn DeferredMaterial,
        camera: &Camera,
        viewport: Viewport,
    ) -> Result<()> {
        (*self).render_deferred(material, camera, viewport)
    }
}

impl<G: Geometry, M: ForwardMaterial> Geometry for Glue<G, M> {
    fn aabb(&self) -> &AxisAlignedBoundingBox {
        self.geometry.aabb()
    }
}
impl<G: Geometry, M: ForwardMaterial> Geometry for &Glue<G, M> {
    fn aabb(&self) -> &AxisAlignedBoundingBox {
        (*self).aabb()
    }
}

impl<G: Geometry, M: ForwardMaterial> Object for Glue<G, M> {}
impl<G: Geometry, M: ForwardMaterial> Object for &Glue<G, M> {}

pub trait Object: Drawable + Geometry {}

impl Object for &dyn Object {}

impl Geometry for &dyn Object {
    fn aabb(&self) -> &AxisAlignedBoundingBox {
        (*self).aabb()
    }
}

pub trait Geometry: Shadable + Cullable {
    fn aabb(&self) -> &AxisAlignedBoundingBox;
}

impl Geometry for &dyn Geometry {
    fn aabb(&self) -> &AxisAlignedBoundingBox {
        (*self).aabb()
    }
}

pub trait Shadable {
    fn render_forward(
        &self,
        material: &dyn ForwardMaterial,
        camera: &Camera,
        lights: &Lights,
    ) -> Result<()>;

    ///
    /// Render the geometry and surface material parameters of the object.
    /// Should not be called directly but used in a [deferred render pass](crate::DeferredPipeline::geometry_pass).
    ///
    fn render_deferred(
        &self,
        material: &dyn DeferredMaterial,
        camera: &Camera,
        viewport: Viewport,
    ) -> Result<()>;
}

impl Shadable for &dyn Shadable {
    fn render_forward(
        &self,
        material: &dyn ForwardMaterial,
        camera: &Camera,
        lights: &Lights,
    ) -> Result<()> {
        (*self).render_forward(material, camera, lights)
    }

    fn render_deferred(
        &self,
        material: &dyn DeferredMaterial,
        camera: &Camera,
        viewport: Viewport,
    ) -> Result<()> {
        (*self).render_deferred(material, camera, viewport)
    }
}

impl Shadable for &dyn Geometry {
    fn render_forward(
        &self,
        material: &dyn ForwardMaterial,
        camera: &Camera,
        lights: &Lights,
    ) -> Result<()> {
        (*self).render_forward(material, camera, lights)
    }

    fn render_deferred(
        &self,
        material: &dyn DeferredMaterial,
        camera: &Camera,
        viewport: Viewport,
    ) -> Result<()> {
        (*self).render_deferred(material, camera, viewport)
    }
}

impl Shadable for &dyn Object {
    fn render_forward(
        &self,
        material: &dyn ForwardMaterial,
        camera: &Camera,
        lights: &Lights,
    ) -> Result<()> {
        (*self).render_forward(material, camera, lights)
    }

    fn render_deferred(
        &self,
        material: &dyn DeferredMaterial,
        camera: &Camera,
        viewport: Viewport,
    ) -> Result<()> {
        (*self).render_deferred(material, camera, viewport)
    }
}

pub trait Cullable {
    fn in_frustum(&self, camera: &Camera) -> bool;
}

impl Cullable for &dyn Cullable {
    fn in_frustum(&self, camera: &Camera) -> bool {
        (*self).in_frustum(camera)
    }
}

impl Cullable for &dyn Geometry {
    fn in_frustum(&self, camera: &Camera) -> bool {
        (*self).in_frustum(camera)
    }
}

impl Cullable for &dyn Object {
    fn in_frustum(&self, camera: &Camera) -> bool {
        (*self).in_frustum(camera)
    }
}

pub trait Drawable {
    ///
    /// Render the object.
    /// Must be called in a render target render function,
    /// for example in the callback function of [Screen::write](crate::Screen::write).
    ///
    fn render(&self, camera: &Camera, lights: &Lights) -> Result<()>;
}

impl Drawable for &dyn Drawable {
    fn render(&self, camera: &Camera, lights: &Lights) -> Result<()> {
        (*self).render(camera, lights)
    }
}

impl Drawable for &dyn Object {
    fn render(&self, camera: &Camera, lights: &Lights) -> Result<()> {
        (*self).render(camera, lights)
    }
}

// 2D

pub trait Geometry2D: Shadable2D + Cullable2D {}

impl Geometry2D for &dyn Geometry2D {}

pub trait Shadable2D {
    fn render_forward(&self, material: &dyn ForwardMaterial, viewport: Viewport) -> Result<()>;
}

impl Shadable2D for &dyn Shadable2D {
    fn render_forward(&self, material: &dyn ForwardMaterial, viewport: Viewport) -> Result<()> {
        (*self).render_forward(material, viewport)
    }
}

impl Shadable2D for &dyn Geometry2D {
    fn render_forward(&self, material: &dyn ForwardMaterial, viewport: Viewport) -> Result<()> {
        (*self).render_forward(material, viewport)
    }
}

pub trait Cullable2D {
    fn in_frustum(&self, viewport: Viewport) -> bool;
}

impl Cullable2D for &dyn Cullable2D {
    fn in_frustum(&self, viewport: Viewport) -> bool {
        (*self).in_frustum(viewport)
    }
}

impl Cullable2D for &dyn Geometry2D {
    fn in_frustum(&self, viewport: Viewport) -> bool {
        (*self).in_frustum(viewport)
    }
}

#[deprecated]
pub trait ShadedGeometry: Geometry {
    ///
    /// Render the geometry and surface material parameters of the object.
    /// Should not be called directly but used in a [deferred render pass](crate::DeferredPipeline::geometry_pass).
    ///
    #[deprecated = "Use 'render_deferred' instead"]
    fn geometry_pass(&self, camera: &Camera, viewport: Viewport, material: &Material)
        -> Result<()>;
    ///
    /// Render the object shaded with the given lights using physically based rendering (PBR).
    /// Must be called in a render target render function, for example in the callback function of [Screen::write].
    /// Will render transparent if the material contain an albedo color with alpha value below 255 or if the albedo texture contain an alpha channel (ie. the format is [Format::RGBA]),
    /// you only need to render the model after all solid models.
    ///
    #[deprecated = "Use 'render_forward' instead"]
    fn render_with_lighting(
        &self,
        camera: &Camera,
        material: &Material,
        lighting_model: LightingModel,
        ambient_light: Option<&AmbientLight>,
        directional_lights: &[&DirectionalLight],
        spot_lights: &[&SpotLight],
        point_lights: &[&PointLight],
    ) -> Result<()>;
}
