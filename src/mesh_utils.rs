use tobj;

pub fn load_cube() -> (Vec<[f32; 3]>, Vec<[f32; 3]>, Vec<u32>) {
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

    let mut positions_array: Vec<[f32; 3]> = Vec::new();
    let mut normals_array: Vec<[f32; 3]> = Vec::new();

    for position in positions.chunks(3) {
        let x = position[0];
        let y = position[1];
        let z = position[2];

        let array = [x, y, z];
        positions_array.push(array);
    }

    for normal in normals.chunks(3) {
        let x = normal[0];
        let y = normal[1];
        let z = normal[2];

        let array = [x, y, z];
        normals_array.push(array);
    }

    println!("{:?}", positions_array);
    println! {"{}", positions_array.len()};
    println!("{:?}", normals_array);
    println! {"{}", normals_array.len()};
    println!("{:?}", indices);

    (positions_array, normals_array, indices)
}
