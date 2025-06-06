use bevy_asset::{weak_handle, Handle};
use bevy_render::render_resource::Shader;

pub mod material;
pub mod mesh;
pub mod plugin;

pub mod prelude {
    pub use crate::material::PointsMaterial;
    pub use crate::mesh::PointsMesh;
    pub use crate::plugin::PointsPlugin;
}

pub const SHADER_HANDLE: Handle<Shader> = weak_handle!("efbb945e-e50e-4f70-89b5-7b7ddbf1986f");
