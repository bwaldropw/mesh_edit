use bevy::math::Vec3;

pub fn calculate_normals(vertices: &[Vec3], indices: &[u32]) -> Vec<Vec3> {
    let mut normals = vec![Vec3::new(0.0, 0.0, 0.0); vertices.len()];

    for chunk in indices.chunks(3) {
        if let [i0, i1, i2] = chunk {
            let v0 = vertices[*i0 as usize];
            let v1 = vertices[*i1 as usize];
            let v2 = vertices[*i2 as usize];

            let edge1 = v1 - v0;
            let edge2 = v2 - v0;

            let triangle_normal = edge1.cross(edge2).normalize();

            normals[*i0 as usize] += triangle_normal;
            normals[*i1 as usize] += triangle_normal;
            normals[*i2 as usize] += triangle_normal;
        }
    }
    
    for normal in &mut normals {
        *normal = normal.normalize();
    }

    normals
}