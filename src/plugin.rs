use bevy::prelude::{Assets, MaterialPlugin, Plugin, Shader};

use crate::{prelude::PointsMaterial, SHADER_HANDLE};

pub struct PointsPlugin;

impl Plugin for PointsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        let mut shaders = app.world.get_resource_mut::<Assets<Shader>>().unwrap();
        shaders.set_untracked(
            SHADER_HANDLE,
            Shader::from_wgsl(include_str!("./shaders/points.wgsl"), String::new()),
        );
        app.add_plugins(MaterialPlugin::<PointsMaterial>::default());
    }
}
