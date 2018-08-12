use glm::*;

use std::vec::Vec;
use std::collections::HashMap;

// Code in this file taken from https://schneide.blog/2016/07/15/generating-an-icosphere-in-c/

const X: f32 = 0.525731112119133606;
const Z: f32 = 0.850650808352039932;
const N: f32 = 0.0;

const vertices: Vec<Vector3<f32>> =
vec![
    Vector3::new(-X, N, Z), Vector3::new(X, N,  Z), Vector3::new(-X,  N, -Z), Vector3::new(X,  N, -Z),
    Vector3::new(N, Z, X), Vector3::new(N, Z, -X), Vector3::new(N, -Z,   X), Vector3::new(N, -Z, -X),
    Vector3::new(Z, X, N), Vector3::new(-Z, X, N), Vector3::new(Z, -X,   N), Vector3::new(-Z, -X,  N)
];

const triangles: Vec<Vector3<f32>> =
vec![
    Vector3::new(0,  4,  1), Vector3::new(0, 9,  4), Vector3::new(9,  5, 4), Vector3::new(4, 5, 8), Vector3::new(4, 8,  1),
    Vector3::new(8, 10,  1), Vector3::new(8, 3, 10), Vector3::new(5,  3, 8), Vector3::new(5, 2, 3), Vector3::new(2, 7,  3),
    Vector3::new(7, 10,  3), Vector3::new(7, 6, 10), Vector3::new(7, 11, 6), Vector3::new(11, 0, 6), Vector3::new(0, 1,  6),
    Vector3::new(6,  1, 10), Vector3::new(9, 0, 11), Vector3::new(9, 11, 2), Vector3::new(9, 2, 5), Vector3::new(7, 2, 11)
];

fn vertex_for_edge(lookup: HashMap<(u32, u32), u32>, vertices: Vec<Vector3<f32>>, first: u32, second: u32) -> u32 {
    let mut key = (first, second);

    if(key.0 > key.1) {
        key = (key.1, key.0);
    }

    let inserted = lookup.insert(key, vertices.Num());
    if(inserted.second) {
        let edge0 = vertices[first];
        let edge1 = vertices[second];
        let point = (edge0 + edge1);
        point.Normalize();
        vertices.Add(point);
    }

    return inserted.first->second;
}

// TriangleList subdivide(VertexList& vertices, TriangleList triangles) {
//     std::unordered_map<std::pair<int32_t, int32_t>, int32_t> lookup;
//     TriangleList result;
// 
//     for(auto&& each : triangles) {
//         int32_t mid[3];
//         for(int edge = 0; edge<3; ++edge) {
//             mid[edge] = vertex_for_edge(lookup, vertices, each.vertex[edge], each.vertex[(edge + 1) % 3]);
//         }
// 
//         result.Add({ each.vertex[0], mid[0], mid[2] });
//         result.Add({ each.vertex[1], mid[1], mid[0] });
//         result.Add({ each.vertex[2], mid[2], mid[1] });
//         result.Add({ mid[0], mid[1], mid[2] });
//     }
// 
//     return result;
// }

pub fn make_icosphere(subdivisions: u32) -> (Vec<Vector3<f32>>, Vec<u32>) {
    let mut mesh_vertices = vertices;
    let mut mesh_triangles = triangles;

    for i in 0..subdivisions {
        mesh_triangles = subdivide(mesh_vertices, mesh_triangles);
    }

    (vertices, triangles);
}
