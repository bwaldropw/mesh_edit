mod mesh_utils;
mod settings;

use std::env;

use bevy::prelude::*;
use bevy::render::mesh::{self, PrimitiveTopology};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    App::new()
        .insert_resource(Msaa::Sample4)
        // TODO fetch settings from file
        .insert_resource(settings::UserSettings {
            show_fps: true,
            orbit_sensitivity: 1.5,
            pan_sensitivity: 1.0,
            zoom_sensitivity: 0.5,
        })
        .add_plugins((DefaultPlugins, PanOrbitCameraPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut settings: ResMut<settings::UserSettings>,
) {
    
    let (positions, normals, indices) = mesh_utils::load_cube();

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.set_indices(Some(mesh::Indices::U32(indices)));
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);

    let material = StandardMaterial {
        base_color: Color::rgb(0.898, 0.918, 0.941),
        ..default()
    };

    commands.spawn(PbrBundle {
        mesh: meshes.add(mesh),
        material: materials.add(material),
        ..default()
    });

    commands.insert_resource(AmbientLight {
        color: Color::ANTIQUE_WHITE,
        brightness: 1.0,
    });

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 1.5, 5.0)),
            ..default()
        },
        PanOrbitCamera {
            orbit_sensitivity: settings.orbit_sensitivity,
            pan_sensitivity: settings.pan_sensitivity,
            zoom_sensitivity: settings.zoom_sensitivity,
            ..default()
        },
    ));
}
