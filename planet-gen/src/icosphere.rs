use glm::*;

use std::vec::Vec;
use std::collections::HashMap;

// Code in this file taken from https://schneide.blog/2016/07/15/generating-an-icosphere-in-c/

const X: f32 = 0.525731112119133606;
const Z: f32 = 0.850650808352039932;
const N: f32 = 0.0;

pub fn make_icosphere(subdivisions: u32) -> (Vec<Vec3>, Vec<[usize; 3]>) {
    let original_vertices: Vec<Vec3> =
    vec![
        Vec3::new(-X, N, Z), Vec3::new(X, N,  Z), Vec3::new(-X,  N, -Z), Vec3::new(X,  N, -Z),
        Vec3::new(N, Z, X), Vec3::new(N, Z, -X), Vec3::new(N, -Z,   X), Vec3::new(N, -Z, -X),
        Vec3::new(Z, X, N), Vec3::new(-Z, X, N), Vec3::new(Z, -X,   N), Vec3::new(-Z, -X,  N)
    ];

    let original_triangles: Vec<[usize; 3]> =
    vec![
        [0,  4,  1], [0, 9,  4], [9,  5, 4], [4, 5,  8], [4, 8,  1],
        [8, 10,  1], [8, 3, 10], [5,  3, 8], [5, 2,  3], [2, 7,  3],
        [7, 10,  3], [7, 6, 10], [7, 11, 6], [11, 0, 6], [0, 1,  6],
        [6,  1, 10], [9, 0, 11], [9, 11, 2], [9, 2,  5], [7, 2, 11]
    ]; 

    let mut mesh_vertices = original_vertices;
    let mut mesh_triangles = original_triangles;

    for _i in 0..subdivisions {
        mesh_triangles = subdivide(&mut mesh_vertices, mesh_triangles);
    }

    (mesh_vertices, mesh_triangles)
}

fn vertex_for_edge(lookup: &mut HashMap<(usize, usize), usize>, vertices: &mut Vec<Vec3>, first: usize, second: usize) -> usize {
    let mut key = (first, second);

    if key.0 > key.1 {
        key = (key.1, key.0);
    }

    let existing_val = lookup.insert(key, vertices.len());
    let mut ret_val: usize = 0;

    match existing_val {
        None => {
            // The map did not have the key present. Nothing was inserted
            let edge0 = vertices[first];
            let edge1 = vertices[second];
            let point = normalize(edge0 + edge1);
            vertices.push(point);

            ret_val = lookup[&key];
        },
        Some(x) =>  {
            ret_val = x;
        }
    }

    ret_val
}

fn subdivide(vertices: &mut Vec<Vec3>, triangles: Vec<[usize; 3]>) -> Vec<[usize; 3]> {
    let mut lookup: HashMap<(usize, usize), usize> = HashMap::new();

    let mut output_indices: Vec<[usize; 3]> = Vec::new();

    for triangle in triangles {
        let mut mid: [usize; 3] = [0, 0, 0];
        for edge in 0..3 {
            mid[edge] = vertex_for_edge(&mut lookup, vertices, triangle[edge], triangle[(edge + 1) % 3]);
        }

        output_indices.push([triangle[0], mid[0], mid[2]]);
        output_indices.push([triangle[1], mid[1], mid[0]]);
        output_indices.push([triangle[2], mid[2], mid[1]]);
        output_indices.push([ mid[0], mid[1], mid[2]]);
    }

    return output_indices;
}
