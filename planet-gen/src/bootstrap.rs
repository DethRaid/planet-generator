/// Generates the initial values for the planets: size, distance from sun, composition, etc

use rand::distributions::Normal;
use rand::Rng;
use std::cmp::{max, min};

use initial_planet_data::*;

use output_data::*;

use icosphere::make_icosphere;

use glm::*;

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
        let mut rng = rand::thread_rng();

        // Vertices around center = 5 * 2^subdivisions
        // length of edge = 2*PI*R / (5 * 2^subdivisions)
        // 5 * 2^subdivisions = 2*PI*R
        // 2^subdivisions = 2*PI*R / 5
        // subdivisions = log2(2*PI*R / 5)

        let subdivisions = (2.0 * std::f64::consts::PI * (initial_data.radius as f64) / (5.0 * 10.0)).log2();

        let mesh: (Vec<Vec3>, Vec<[usize; 3]>) = make_icosphere(min(subdivisions as u32, 5));

        let (positions, triangles) = mesh;

        // Assign tectonic plates
        // Surface area of a sphere is 4*pi*r
        // Average tectonic plate surface area = 63,759,000 km^2
        // Number of tectonis plates = ceil(4*PI*R/63759000)
        let num_plates = (4.0 * std::f64::consts::PI * (initial_data.radius as f64) / 63759000.0).ceil() as usize;
        let mut plate_centers: Vec<usize> = Vec::new();
        plate_centers.reserve(num_plates);

        for _i in 0..num_plates {
            let mut plate_idx = rng.gen_range(0, positions.len());

            while !plate_centers.contains(&plate_idx) {
                plate_idx = rng.gen_range(0, positions.len());
            }
            plate_centers.push(plate_idx);
        }

        let compare_positions = positions.clone();

        let mut vertices: Vec<Vertex> = Vec::new();
        vertices.reserve(positions.len());
        // This could be a map statement rn, but when I do more complex things for normals and UVs it won't be
        // Leaving like this for now
        for pos in positions {
            // Find the tectonic plate center that we're closest to
            // Will generate roughly circular plates but hopefully they'll squish into something interesting
            let mut closest_plate_idx = plate_centers[0];
            for i in 0..plate_centers.len() {
                let dist_to_new_center = distance(pos, compare_positions[plate_centers[i]]);
                let dist_to_old_center = distance(pos, compare_positions[plate_centers[closest_plate_idx]]);

                if dist_to_new_center < dist_to_old_center {
                    closest_plate_idx = i;
                }
            }

            let normal = normalize(pos);
            vertices.push(Vertex{ position: [pos.x, pos.y, pos.z], normal: [normal.x, normal.y, normal.z], plate_idx: closest_plate_idx as u32 });
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
