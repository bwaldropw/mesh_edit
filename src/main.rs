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
    // TODO hot reload
    // TODO mesh utils


    let vertices = [
        ([-0.5, -0.5, -0.5], [0.0, 0.0, -1.0]), // Vertex 0: Bottom-back-left
        ([0.5, -0.5, -0.5], [0.0, 0.0, -1.0]),  // Vertex 1: Bottom-back-right
        ([0.5, 0.5, -0.5], [0.0, 0.0, -1.0]),   // Vertex 2: Top-back-right
        ([-0.5, 0.5, -0.5], [0.0, 0.0, -1.0]),  // Vertex 3: Top-back-left
        ([-0.5, -0.5, 0.5], [0.0, 0.0, 1.0]),   // Vertex 4: Bottom-front-left
        ([0.5, -0.5, 0.5], [0.0, 0.0, 1.0]),    // Vertex 5: Bottom-front-right
        ([0.5, 0.5, 0.5], [0.0, 0.0, 1.0]),     // Vertex 6: Top-front-right
        ([-0.5, 0.5, 0.5], [0.0, 0.0, 1.0]),    // Vertex 7: Top-front-left
    ];

    // TODO fix normals

    let indices = vec![
        0, 2, 1, 0, 3, 2, // Back face
        4, 5, 6, 4, 6, 7, // Front face
        0, 1, 5, 0, 5, 4, // Bottom face
        2, 7, 6, 2, 3, 7, // Top face
        0, 4, 7, 0, 7, 3, // Left face
        1, 2, 6, 1, 6, 5, // Right face
    ];

    let mut positions = Vec::new();
    let mut normals = Vec::new();

    for (position, normal) in vertices.iter() {
        positions.push(*position);
        normals.push(*normal);
    }

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
