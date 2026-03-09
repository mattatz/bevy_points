use bevy::{asset::embedded_asset, prelude::MaterialPlugin};

use crate::prelude::PointsMaterial;

pub struct PointsPlugin;

impl bevy::prelude::Plugin for PointsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        embedded_asset!(app, "shaders/points.wgsl");
        app.add_plugins(MaterialPlugin::<PointsMaterial>::default());
    }
}
