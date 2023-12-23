use bevy::math::Vec3;
use tobj;

pub fn load_cube() -> (Vec<Vec3>, Vec<Vec3>, Vec<u32>) {
    let obj_file = "./assets/cube.obj";

    let lopts = tobj::LoadOptions {
        triangulate: true,
        single_index: true,
        ..Default::default()
    };

    let (models, _materials) = tobj::load_obj(&obj_file, &lopts).expect("Failed to OBJ load file");

    let cube = models[0].clone();
    let mesh = cube.mesh;

    let positions = mesh.positions;
    let normals = mesh.normals;
    let indices = mesh.indices;

    let mut vec3_positions: Vec<Vec3> = Vec::new();
    let mut vec3_normals: Vec<Vec3> = Vec::new();

    for position in positions.chunks(3) {
        let x = position[0];
        let y = position[1];
        let z = position[2];

        let vec3 = Vec3::new(x, y, z);
        vec3_positions.push(vec3);
    }

    for normal in normals.chunks(3) {
        let x = normal[0];
        let y = normal[1];
        let z = normal[2];

        let vec3 = Vec3::new(x, y, z);
        vec3_normals.push(vec3);
    }

    println!("{:?}", vec3_positions);
    println!{"{}", vec3_positions.len()};
    println!("{:?}", vec3_normals);
    println!{"{}", vec3_normals.len()};
    println!("{:?}", indices);

    (vec3_positions, vec3_normals, indices)
}
