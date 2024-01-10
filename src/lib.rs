use bevy::prelude::{Handle, Shader};

pub mod material;
pub mod mesh;
pub mod plugin;

pub mod prelude {
    pub use crate::material::PointsMaterial;
    pub use crate::mesh::PointsMesh;
    pub use crate::plugin::PointsPlugin;
}

pub const SHADER_HANDLE: Handle<Shader> = Handle::weak_from_u128(15532858032624716725);
