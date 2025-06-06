use bevy_asset::Asset;
use bevy_color::LinearRgba;
use bevy_pbr::{Material, MAX_CASCADES_PER_LIGHT, MAX_DIRECTIONAL_LIGHTS};
use bevy_reflect::TypePath;
use bevy_render::{
    alpha::AlphaMode,
    mesh::Mesh,
    render_resource::{AsBindGroup, ShaderDefVal, ShaderType},
};

use crate::SHADER_HANDLE;

#[derive(Debug, Clone, Copy, ShaderType)] // ShaderType
pub struct PointsShaderSettings {
    pub point_size: f32,
    pub opacity: f32,
    pub color: LinearRgba,
}

impl Default for PointsShaderSettings {
    fn default() -> Self {
        Self {
            point_size: 1.,
            opacity: 1.,
            color: Default::default(),
        }
    }
}

#[derive(AsBindGroup, Debug, Clone, Copy, TypePath, Asset)]
#[bind_group_data(PointsMaterialKey)]
pub struct PointsMaterial {
    #[uniform(0)]
    pub settings: PointsShaderSettings,
    pub depth_bias: f32,
    pub alpha_mode: AlphaMode,
    pub use_vertex_color: bool,
    pub perspective: bool,
    pub circle: bool,
}

impl Default for PointsMaterial {
    fn default() -> Self {
        Self {
            settings: PointsShaderSettings::default(),
            depth_bias: 0.,
            alpha_mode: Default::default(),
            use_vertex_color: true,
            perspective: true,
            circle: false,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct PointsMaterialKey {
    use_vertex_color: bool,
    perspective: bool,
    circle: bool,
}

impl From<&PointsMaterial> for PointsMaterialKey {
    fn from(material: &PointsMaterial) -> Self {
        PointsMaterialKey {
            use_vertex_color: material.use_vertex_color,
            perspective: material.perspective,
            circle: material.circle,
        }
    }
}

impl Material for PointsMaterial {
    fn vertex_shader() -> bevy_render::render_resource::ShaderRef {
        bevy_render::render_resource::ShaderRef::Handle(SHADER_HANDLE.clone())
    }

    fn fragment_shader() -> bevy_render::render_resource::ShaderRef {
        bevy_render::render_resource::ShaderRef::Handle(SHADER_HANDLE.clone())
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }

    fn depth_bias(&self) -> f32 {
        self.depth_bias
    }

    fn specialize(
        _pipeline: &bevy_pbr::MaterialPipeline<Self>,
        descriptor: &mut bevy_render::render_resource::RenderPipelineDescriptor,
        layout: &bevy_render::mesh::MeshVertexBufferLayoutRef,
        key: bevy_pbr::MaterialPipelineKey<Self>,
    ) -> Result<(), bevy_render::render_resource::SpecializedMeshPipelineError> {
        descriptor.primitive.cull_mode = None;

        let mut shader_defs = vec![];
        let mut vertex_attributes = vec![
            Mesh::ATTRIBUTE_POSITION.at_shader_location(0),
            Mesh::ATTRIBUTE_UV_0.at_shader_location(1),
        ];

        // CAUTION: To fix compilation errors in WGSL, the definitions of lights need to be resolved.
        shader_defs.push(ShaderDefVal::UInt(
            "MAX_DIRECTIONAL_LIGHTS".to_string(),
            MAX_DIRECTIONAL_LIGHTS as u32,
        ));
        shader_defs.push(ShaderDefVal::UInt(
            "MAX_CASCADES_PER_LIGHT".to_string(),
            MAX_CASCADES_PER_LIGHT as u32,
        ));

        if key.bind_group_data.use_vertex_color && layout.0.contains(Mesh::ATTRIBUTE_COLOR) {
            shader_defs.push(ShaderDefVal::from("VERTEX_COLORS"));
            vertex_attributes.push(Mesh::ATTRIBUTE_COLOR.at_shader_location(2));
        }
        if key.bind_group_data.perspective {
            shader_defs.push(ShaderDefVal::from("POINT_SIZE_PERSPECTIVE"));
        }
        if key.bind_group_data.circle {
            shader_defs.push(ShaderDefVal::from("POINT_SHAPE_CIRCLE"));
        }

        let vertex_layout = layout.0.get_layout(&vertex_attributes)?;
        descriptor.vertex.buffers = vec![vertex_layout];
        descriptor.vertex.shader_defs = shader_defs.clone();
        if let Some(fragment) = &mut descriptor.fragment {
            fragment.shader_defs = shader_defs;
        }

        Ok(())
    }
}
