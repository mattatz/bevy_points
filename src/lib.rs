use bevy::{
    prelude::{HandleUntyped, Shader},
    reflect::TypeUuid,
};

pub mod material;
pub mod mesh;
pub mod plugin;

pub mod prelude {
    pub use crate::material::PointsMaterial;
    pub use crate::mesh::PointsMesh;
    pub use crate::plugin::PointsPlugin;
}

pub const SHADER_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 15532858032624716725);
