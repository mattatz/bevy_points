pub mod material;
pub mod mesh;
pub mod plugin;

pub mod prelude {
    pub use crate::material::PointsMaterial;
    pub use crate::mesh::PointsMesh;
    pub use crate::plugin::PointsPlugin;
}
