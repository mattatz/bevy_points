use std::f32::consts::{FRAC_PI_2, TAU};

use bevy::{prelude::*, DefaultPlugins};
use bevy_points::{material::PointsShaderSettings, prelude::*};

const ORIGIN: Vec3 = Vec3::new(0.0, 0.0, -5.0);

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PointsPlugin))
        .insert_resource(ClearColor(Color::rgb(0.01, 0.02, 0.08)))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<PointsMaterial>>,
) {
    let mut pt = PointsMesh::from(Mesh::from(Sphere { radius: 1.0 }));
    pt.colors = Some(
        pt.vertices
            .iter()
            .map(|p| {
                let n = (p.normalize() + 1.0) * 0.5;
                Color::rgb_from_array(n.to_array())
            })
            .collect(),
    );

    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(pt),
        material: materials.add(PointsMaterial {
            settings: PointsShaderSettings {
                point_size: 0.1,
                opacity: 0.5,
                ..Default::default()
            },
            perspective: true,
            alpha_mode: AlphaMode::Blend,
            ..Default::default()
        }),
        transform: Transform::from_translation(Vec3::NEG_X * 1.25)
            .with_rotation(Quat::from_axis_angle(Vec3::ONE.normalize(), FRAC_PI_2)),
        ..Default::default()
    });

    let n = 320;
    let h = 3.0;
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(PointsMesh::from_iter((0..n).map(|i| {
            let t01 = (i as f32) / ((n - 1) as f32);
            let r = t01 * TAU * 4.0;
            Vec3::new(r.cos(), (t01 - 0.5) * h, r.sin())
        }))),
        material: materials.add(PointsMaterial {
            settings: PointsShaderSettings {
                point_size: 20.,
                opacity: 1.,
                ..Default::default()
            },
            perspective: false,
            circle: true,
            ..Default::default()
        }),
        transform: Transform::from_translation(Vec3::X * 1.25),
        ..Default::default()
    });

    commands.spawn(Camera3dBundle {
        projection: PerspectiveProjection {
            fov: 45.0,
            aspect_ratio: 1.,
            near: 0.1,
            far: 100.,
        }
        .into(),
        transform: Transform::from_translation(ORIGIN).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}
