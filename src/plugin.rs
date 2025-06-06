use bevy_app::Plugin;
use bevy_asset::load_internal_asset;
use bevy_pbr::MaterialPlugin;
use bevy_render::render_resource::Shader;

use crate::{prelude::PointsMaterial, SHADER_HANDLE};

pub struct PointsPlugin;

impl Plugin for PointsPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        load_internal_asset!(
            app,
            SHADER_HANDLE,
            "./shaders/points.wgsl",
            Shader::from_wgsl
        );
        app.add_plugins(MaterialPlugin::<PointsMaterial>::default());
    }
}
