use bevy::{prelude::{MaterialPlugin, Plugin, Shader,}, asset::load_internal_asset};

use crate::{prelude::PointsMaterial, SHADER_HANDLE};

pub struct PointsPlugin;

impl Plugin for PointsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        load_internal_asset!(app, SHADER_HANDLE, "./shaders/points.wgsl", Shader::from_wgsl);
        app.add_plugins(MaterialPlugin::<PointsMaterial>::default());
    }
}
