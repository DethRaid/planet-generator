use rand::distributions::Normal;
use rand::Rng;

use initial_planet_data::*;

/// Generates the initial values for the planets: size, distance from sun, composition, etc

impl Default for InitialPlanetParams {
    fn default() -> Self {
        InitialPlanetParams {
            distance_from_sun: 0,
            orbital_period: 0.0,
            rotational_period: 0.0,
            age: 0,
            planet_type: PlanetType::Gassy,
            radius: 0,
        }
    }
}

/// Radius of Jupiter, in meters
const RADIUS_OF_JUPITER: u64 = 69911000;

impl InitialPlanetParams {
    /// Randonly generates a new planet, using distributions from real-world data
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();

        let distance_from_sun_distro = Normal::new(1264789393.0, 1621734335.0);
        let orbital_period_distro = Normal::new(236.5964884, 1417.389156);
        let rotational_period_distro = Normal::new(40.52946268, 84.13874671);
        let radius_distro = Normal::new(0.35966285, 0.411208526);

        let radius = RADIUS_OF_JUPITER * rng.sample(radius_distro) as u64;

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
            radius: radius,
        }
    }
}
