mod settings;

use bevy::prelude::*;
use bevy::render::mesh::{self, PrimitiveTopology};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
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
    settings: Res<settings::UserSettings>,
) {
    // TODO import mesh
    let vertices = [
        ([0.0, 0.0, 0.0], [0.0, 1.0, 0.0], [1.0, 1.0]),
        ([1.0, 2.0, 1.0], [0.0, 1.0, 0.0], [1.0, 1.0]),
        ([2.0, 0.0, 0.0], [0.0, 1.0, 0.0], [1.0, 1.0]),
    ];

    let indices = mesh::Indices::U32(vec![0, 2, 1, 0, 3, 2]);

    let mut positions = Vec::new();
    let mut normals = Vec::new();

    for (position, normal, _uv) in vertices.iter() {
        positions.push(*position);
        normals.push(*normal);
    }

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.set_indices(Some(indices));
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);

    commands.spawn(PbrBundle {
        mesh: meshes.add(mesh),
        material: materials.add(Color::rgb(0.898, 0.918, 0.941).into()),
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
