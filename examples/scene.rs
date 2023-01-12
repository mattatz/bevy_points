use std::f32::consts::{FRAC_PI_2, TAU};

use bevy::{
    prelude::{
        shape, AlphaMode, App, Assets, Camera3dBundle, ClearColor, Color, Commands,
        MaterialMeshBundle, Mesh, PerspectiveProjection, Quat, ResMut, Transform, Vec3,
    },
    DefaultPlugins,
};
use bevy_points::prelude::*;

const ORIGIN: Vec3 = Vec3::new(0.0, 0.0, -5.0);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PointsPlugin)
        .insert_resource(ClearColor(Color::rgb(0.01, 0.02, 0.08)))
        .add_startup_system(setup)
        // .add_system(animate)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<PointsMaterial>>,
) {
    let mut pt = PointsMesh::from(Mesh::from(shape::UVSphere {
        radius: 1.0,
        sectors: 36,
        stacks: 18,
    }));
    let n = pt.vertices.len() as f32;
    pt.colors = Some(
        pt.vertices
            .iter()
            .enumerate()
            .map(|(i, _)| {
                let t = (i as f32) / (n - 1.) * 360.;
                Color::hsl(t, 1., 0.5)
            })
            .collect(),
    );

    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(pt.into()),
        material: materials.add(PointsMaterial {
            point_size: 0.1,
            opacity: 0.5,
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
        mesh: meshes.add(
            PointsMesh::from_iter((0..n).map(|i| {
                let t01 = (i as f32) / ((n - 1) as f32);
                let r = t01 * TAU * 4.0;
                Vec3::new(r.cos(), (t01 - 0.5) * h, r.sin())
            }))
            .into(),
        ),
        material: materials.add(PointsMaterial {
            point_size: 20.0,
            opacity: 1.,
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

/*
fn animate(time: Res<Time>, mut camera: Query<&mut Transform, With<Camera>>) {
    if let Ok(mut transform) = camera.get_single_mut() {
        let t = time.elapsed_seconds() * 0.5;
        let o = Vec3::ZERO;
        let p = ORIGIN;
        let d = (p - o).normalize() * t.cos() * 3.;
        transform.translation = p + d;
    }
}
*/
