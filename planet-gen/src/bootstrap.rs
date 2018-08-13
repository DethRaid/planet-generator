/// Generates the initial values for the planets: size, distance from sun, composition, etc

use rand::distributions::Normal;
use rand::Rng;
use std::cmp::max;

use initial_planet_data::*;

use output_data::*;

use icosphere::make_icosphere;

use glm::normalize;

/// Radius of Jupiter, in meters
const RADIUS_OF_JUPITER: u64 = 69911000;

impl InitialPlanetParams {
    /// Randonly generates a new planet, using distributions from real-world data
    pub fn gen_random() -> Self {
        let mut rng = rand::thread_rng();

        let distance_from_sun_distro = Normal::new(1264789393.0, 1621734335.0);
        let orbital_period_distro = Normal::new(236.5964884, 1417.389156);
        let rotational_period_distro = Normal::new(40.52946268, 84.13874671);
        let radius_distro = Normal::new(0.35966285, 0.411208526);

        let radius_raw = (RADIUS_OF_JUPITER as f64 * rng.sample(radius_distro)) as u64;

        let planet_type = match rng.gen_range(0, 3) {
            0 => PlanetType::Rocky,
            1 => PlanetType::Gassy,
            _ => PlanetType::Icy,
        };
        
        InitialPlanetParams {
            distance_from_sun: rng.sample(distance_from_sun_distro) as u64,
            orbital_period: rng.sample(orbital_period_distro),
            rotational_period: rng.sample(rotational_period_distro),
            age: rng.gen_range(1000000, 13500000000),
            planet_type: planet_type,
            radius: match planet_type {
                PlanetType::Gassy => max(RADIUS_OF_JUPITER, radius_raw),
                _ => radius_raw
                }
        }
    }
}

impl Planet {
    pub fn initialize(initial_data: InitialPlanetParams) -> Self {
        // Vertices around center = 5 * 2^subdivisions
        // length of edge = 2*PI*R / (5 * 2^subdivisions)
        // 5 * 2^subdivisions = 2*PI*R
        // 2^subdivisions = 2*PI*R / 5
        // subdivisions = log2(2*PI*R / 5)

        let subdivisions = (2.0 * std::f64::consts::PI * (initial_data.radius as f64) / 5.0).log2();

        let mesh = make_icosphere(subdivisions as u32);

        let (positions, triangles) = mesh;

        let mut vertices: Vec<Vertex> = Vec::new();
        // This could me a map statement rn, but when I do more complex things for normals and UVs it won't be
        // Leaving like this for now
        for pos in positions {
            let normal = normalize(pos);
            vertices.push(Vertex{ position: [pos.x, pos.y, pos.z], normal: [normal.x, normal.y, normal.z] });
        }

        let mut ue4_triangles: Vec<i32> = Vec::new();
        for tri in triangles {
            ue4_triangles.push(tri[0] as i32);
            ue4_triangles.push(tri[1] as i32);
            ue4_triangles.push(tri[2] as i32);
        }

        let ret_val = Planet {
            radius: initial_data.radius,
            distance_from_sun: initial_data.distance_from_sun,

            vertices: vertices.as_mut_ptr(),
            num_vertices: vertices.len() as u32,
            
            indices: ue4_triangles.as_mut_ptr(),
            num_indices: ue4_triangles.len() as u32,
        };

        std::mem::forget(vertices);
        std::mem::forget(ue4_triangles);

        ret_val
    }
}
