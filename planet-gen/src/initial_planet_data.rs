/// All the data structures needed to start generating a planet

/// High-level guess at what kind of planet we want. Different kinds of planets have different layers - this is a nice shorthand
#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
#[repr(C)]
pub enum PlanetType {
    Rocky,
    Gassy,
    Icy,
}

/// All the elements or compounds that a planet can be made out of, and the chances of them spawning in a layer
#[derive(Hash, Eq, PartialEq)]
pub enum PlanetElement {
    Hydrogen,
    HydrogenSulfide,
    HydrogenCyanide,
    Water,      
    Helium,

    Lithium,
    Carbon,
    CarbonMonoxide,
    CarbonDioxide,
    Methane,
    Nitrogen,
    Ammonia,
    AmmoniumSulfide,
    Neon,

    Sodium,
    TableSalt,
    Magnesium,
    Aluminum,
    Silica,
    Phosphorous,
    Sulfur,
    SulfurDioxide,
    Chlorine,
    Argon,

    Potassium,
    Calcium,
    Titanium,
    Chromium,
    Iron,
    Cobalt,
    Nickel,
    Copper,
    Zinc,
    Boromine,

    Silver,
    Iodine,

    Tungstein,
    Platinum,
    Gold,
    Mercury,
    Lead,
    Radon,

    Thorium,
    Uranium,
    Plutonium,
}

/*
 * Constants for bitfields for where an element can spawn
 */

/*
 * Types of planets an element can spawn in
 */

const ROCKY_PLANET: u32 = 0x01;
const ICY_PLANET: u32   = 0x02;
const GASSY_PLANET: u32 = 0x04;
const ATMOSPHERE: u32   = 0x08;

/// The data for how to spawn a given element
pub struct ElementSpawnData {
    /// How likely that this element will be spawned. I Monte-carlo sample the element probabilities to get the 
    /// composition of the layer. 
    /// 
    /// Composition is by volume
    pub probability: f32,

    /// The location where this element may be found. Certain elements can only be spawned in certain locations
    pub spawn_location: u32,
}

impl ElementSpawnData {
  pub fn new(probability: f32, spawn_location: u32) -> Self {
    ElementSpawnData {
      probability,
      spawn_location
    }
  }
}

/// The very initial parameters for planet generation. Essentially the seed for the planet generation algorithm
#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
#[repr(C)]
pub struct InitialPlanetParams {
    /// The distance, in kilometers, from the planet to its star
    pub distance_from_sun: u64,

    /// The number of Earth years that it takes the planet to orbit its star
    pub orbital_period: f64,

    /// The number of Earth days it takes for this planet to make one full rotation
    pub rotational_period: f64,

    /// The age of the planet when the player finds it, in Earth years. Ranges from one thousand to thirteen billion 
    /// - Can't have planets older than the universe!
    pub age: u64,

    /// Whether to generate this planet as a gas giant
    /// 
    /// Planets smaller than a certain size can't be gaseous because they don't have the necessary gravity to hold a 
    /// thick hydrogen and helium atmosphere, so this field is tied to planetary mass - but it's not directly 
    /// dependent on it
    pub planet_type: PlanetType,

    /// The radius of the planet, in meters
    pub radius: u64,
}