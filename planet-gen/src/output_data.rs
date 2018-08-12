/// The data structures that represent a finalized solar system

use std::vec::Vec;

use glm::Vector3;

#[repr(C)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
}

#[repr(C)]
pub struct Planet {
    pub radius: u64,

    pub distance_from_sun: u64,

    pub vertices: *mut Vertex,
    pub num_vertices: u32,

    pub indices: *mut i32,
    pub num_indices: u32,
}

/// Represents a star in a solar system
#[derive(Serialize, Deserialize, Debug)]
#[repr(C)]
pub struct Star {
    /// The mass of the star, in kg
    /// 
    /// Stars are assumed to be like the Sun, e.g. 71% hydrogen, 28% helium, and 1% something lame. From the mass and 
    /// the densities of those elements we can get the size, the heat, and the color
    pub mass: u64,
}

/// A solar system, in all its glory
#[repr(C)]
pub struct SolarSystem {
    /// All the stars in this solar system. Probably just one entry, but binary star systems are supported
    pub stars: Vec<Star>,

    /// All the planets in this solar system
    pub planets: Vec<Planet>,
}