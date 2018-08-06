use std::collections::HashMap;

/// Generates the initial values for the planets: size, distance from sun, composition, etc

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
    Ammonia,
    AmmoniumSulfide,
    Neon,

    Sodiun,
    Magnesium,
    Aluminum,
    Silica,
    Phosphorous,
    Sulfur,
    SulfurDioxide,
    Chlorine,
    Argon,

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

#[derive(Hash, Eq, PartialEq)]
pub enum ElementSpawnLocation {
    Atmosphere,
    Core,
    Both,
}

/// The data for how to spawn a given element
pub struct ElementSpawnData {
    /// How likely that this element will be spawned. I Monte-carlo sample the element probabilities to get the 
    /// composition of the layer. 
    /// 
    /// Composition is by volume
    pub probability: f32,

    /// The location where this element may be found. Certain elements can only be spawned in certain locations
    pub spawn_location: ElementSpawnLocation,
}

impl PlanetElement {
    fn spawn_data(element : PlanetElement) -> ElementSpawnData {
        match element {
            PlanetElement::Hydrogen => ElementSpawnData { probability: 0.8, spawn_location: ElementSpawnLocation::Atmosphere },
            PlanetElement::HydrogenSulfide => (),
            PlanetElement::HydrogenCyanide => (),
            PlanetElement::Water => (),
            PlanetElement::Helium => (),
            PlanetElement::Lithium => (),
            PlanetElement::Carbon => (),
            PlanetElement::CarbonDioxide => (),
            PlanetElement::CarbonMonoxide => (),
            PlanetElement::Methane => (),
            PlanetElement::Ammonia => (),
            PlanetElement::AmmoniumSulfide => (),
            PlanetElement::Neon => (),
            PlanetElement::Sodiun => (),
            PlanetElement::Magnesium => (),
            PlanetElement::Aluminum => (),
            PlanetElement::Silica => (),
            PlanetElement::Phosphorous => (),
            PlanetElement::Sulfur => (),
            PlanetElement::SulfurDioxide => (),
            PlanetElement::Chlorine => (),
            PlanetElement::Argon => (),
            PlanetElement::Calcium => (),
            PlanetElement::Titanium => (),
            PlanetElement::Chromium => (),
            PlanetElement::Iron => (),
            PlanetElement::Cobalt => (),
            PlanetElement::Nickel => (),
            PlanetElement::Copper => (),
            PlanetElement::Zinc => (),
            PlanetElement::Boromine => (),
            PlanetElement::Silver => (),
            PlanetElement::Iodine => (),
            PlanetElement::Tungstein => (),
            PlanetElement::Platinum => (),
            PlanetElement::Gold => (),
            PlanetElement::Mercury => (),
            PlanetElement::Lead => (),
            PlanetElement::Radon => (),
            PlanetElement::Thorium => (),
            PlanetElement::Uranium => (),
            PlanetElement::Plutonium => (),
        }
    }
}

/// A layer in a planet. ALl parts of the planet, including its atmosphere, are layers
pub struct PlanetLayer {
    /// The elemental(ish) composition of this layer. 
    pub composition: HashMap<PlanetElement, f32>,

    /// The distance from the planet's center, in meters, where the layer starts
    pub start_height: u64,

    /// The distance from the planet's center, in meters, where this layer ends
    pub end_height: u64,
}

/// The very initial parameters for planet generation. Essentially the seed for the planet generation algorithm
pub struct InitialPlanetParams {
    /// The distance, in meters, from the planet to its star. Ranges from 0 to 10 billion, which mimics Earth's solar
    /// system
    pub distance_from_sun: u64,

    /// The number of Earth years that it takes the planet to orbit its star
    pub orbital_period: f32,

    /// The number of Earth days it takes for this planet to make one full rotation
    pub rotational_period: f32,

    /// The age of the planet when the player finds it, in Earth years. Ranges from one thousand to thirteen billion 
    /// - Can't have planets older than the universe!
    pub age: u64,

    /// Whether to generate this planet as a gas giant
    /// 
    /// Planets smaller than a certain size can't be gaseous because they don't have the necessary gravity to hold a 
    /// thick hydrogen and helium atmosphere, so this field is tied to planetary mass - but it's not directly 
    /// dependent on it
    pub is_gas_giant: bool,

    /// The radius of the planet, in meters. Ranges from one million meters - about the radius of Pluto - to 700 
    /// million meters, about ten times the radius of Jupiter
    pub radius: u64,

    /// The mass of the planet, in meters
    pub mass: u64,

    /// All the layers that compose this planet
    /// 
    /// The planet's layers start at the center and go to the edge of the playet's radius. They're then integrated to get the total mass of the planet, which determines its gravity. More gravity = more atmosphere, at least initially
    pub layers: Vec<PlanetLayer>,
}

impl Default for InitialPlanetParams {
    fn default() -> Self {
        InitialPlanetParams {
            distance_from_sun: 0,
            orbital_period: 0.0,
            rotational_period: 0.0,
            age: 0,
            is_gas_giant: false,
            radius: 0,
            mass: 0,
            layers: Vec::new()
        }
    }
}
