//!
//! Definitions for a CPU- and GPU-side material.
//!
use crate::core::*;
use std::rc::Rc;

///
/// A CPU-side version of a [material](crate::Material).
/// Can be constructed manually or loaded via [io](crate::io).
/// Textures are assumed to be in sRGB with or without an alpha channel.
///
pub struct CPUMaterial {
    /// Name. Used for matching geometry and material.
    pub name: String,
    /// Albedo base color, also called diffuse color.
    pub albedo: Color,
    /// Texture with albedo base colors, also called diffuse color.
    pub albedo_texture: Option<CPUTexture<u8>>,
    /// A value in the range `[0..1]` specifying how metallic the material is.
    pub metallic: f32,
    /// A value in the range `[0..1]` specifying how rough the material surface is.
    pub roughness: f32,
    /// Texture containing the metallic and roughness parameters.
    pub metallic_roughness_texture: Option<CPUTexture<u8>>,

    pub occlusion_strength: f32,
    pub occlusion_texture: Option<CPUTexture<u8>>,

    pub normal_scale: f32,
    pub normal_texture: Option<CPUTexture<u8>>,
}

impl Default for CPUMaterial {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            albedo: Color::WHITE,
            albedo_texture: None,
            metallic_roughness_texture: None,
            metallic: 0.0,
            roughness: 1.0,
            normal_texture: None,
            normal_scale: 1.0,
            occlusion_texture: None,
            occlusion_strength: 1.0,
        }
    }
}

///
/// A material used for shading an object.
///
#[derive(Clone)]
pub struct Material {
    /// Name. Used for matching geometry and material.
    pub name: String,
    /// Albedo base color, also called diffuse color.
    pub albedo: Vec4,
    /// Texture with albedo base colors, also called diffuse color.
    pub albedo_texture: Option<Rc<Texture2D>>,
    /// A value in the range `[0..1]` specifying how metallic the material is.
    pub metallic: f32,
    /// A value in the range `[0..1]` specifying how rough the material surface is.
    pub roughness: f32,
    /// Texture containing the metallic and roughness parameters.
    pub metallic_roughness_texture: Option<Rc<Texture2D>>,

    pub occlusion_strength: f32,
    pub occlusion_texture: Option<Rc<Texture2D>>,

    pub normal_scale: f32,
    pub normal_texture: Option<Rc<Texture2D>>,
}

impl Material {
    ///
    /// Constructor.
    ///
    pub fn new(context: &Context, cpu_material: &CPUMaterial) -> Result<Self> {
        let albedo_texture = if let Some(ref cpu_texture) = cpu_material.albedo_texture {
            Some(Rc::new(Texture2D::new(&context, cpu_texture)?))
        } else {
            None
        };
        let metallic_roughness_texture =
            if let Some(ref cpu_texture) = cpu_material.metallic_roughness_texture {
                Some(Rc::new(Texture2D::new(&context, cpu_texture)?))
            } else {
                None
            };
        let normal_texture = if let Some(ref cpu_texture) = cpu_material.normal_texture {
            Some(Rc::new(Texture2D::new(&context, cpu_texture)?))
        } else {
            None
        };
        let occlusion_texture = if let Some(ref cpu_texture) = cpu_material.occlusion_texture {
            Some(Rc::new(Texture2D::new(&context, cpu_texture)?))
        } else {
            None
        };
        Ok(Self {
            name: cpu_material.name.clone(),
            albedo: cpu_material.albedo.to_vec4(),
            albedo_texture,
            metallic: cpu_material.metallic,
            roughness: cpu_material.roughness,
            metallic_roughness_texture,
            normal_texture,
            normal_scale: cpu_material.normal_scale,
            occlusion_texture,
            occlusion_strength: cpu_material.occlusion_strength,
        })
    }

    pub(crate) fn bind(&self, program: &Program) -> Result<()> {
        program.use_uniform_float("metallic", &self.metallic)?;
        program.use_uniform_float("roughness", &self.roughness)?;
        program.use_uniform_vec4("albedo", &self.albedo)?;
        if let Some(ref texture) = self.albedo_texture {
            program.use_texture("albedoTexture", texture.as_ref())?;
        }
        if let Some(ref texture) = self.metallic_roughness_texture {
            program.use_texture("metallicRoughnessTexture", texture.as_ref())?;
        }
        if let Some(ref texture) = self.occlusion_texture {
            program.use_uniform_float("occlusionStrength", &self.occlusion_strength)?;
            program.use_texture("occlusionTexture", texture.as_ref())?;
        }
        if let Some(ref texture) = self.normal_texture {
            program.use_uniform_float("normalScale", &self.normal_scale)?;
            program.use_texture("normalTexture", texture.as_ref())?;
        }
        Ok(())
    }
}

impl Default for Material {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            albedo: vec4(1.0, 1.0, 1.0, 1.0),
            albedo_texture: None,
            metallic: 0.0,
            roughness: 1.0,
            metallic_roughness_texture: None,
            normal_texture: None,
            normal_scale: 1.0,
            occlusion_texture: None,
            occlusion_strength: 1.0,
        }
    }
}
