/// All the data structures needed to start generating a planet

use std::collections::HashMap;

/// High-level guess at what kind of planet we want. Different kinds of planets have different layers - this is a nice shorthand
pub enum PlanetType {
    Chrondite,
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
 * Chrondite planets
 */

const INNER_CORE: u32       = 0x001;
const OUTER_CORE: u32       = 0x002;
const INNTER_MANTLE: u32    = 0x004;
const ASTHENOSPHERE: u32    = 0x008;
const UPPER_MANTLE: u32     = 0x010;
const CRUST: u32            = 0x020;

/*
 * Atmospheres
 */

const TROPOSPHERE: u32      = 0x040;
const STRATOSPHERE: u32     = 0x080;
const MESOSPHERE: u32       = 0x100;
const THERMOSPHERE: u32     = 0x200;
const EXOSPHERE: u32        = 0x400;

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

impl PlanetElement {
    fn spawn_data(element : PlanetElement) -> ElementSpawnData {
        match element {
            // In atmosphere from solar winds
            PlanetElement::Hydrogen         => ElementSpawnData::new(0.8, ElementSpawnLocation::Atmosphere),
            PlanetElement::HydrogenSulfide  => ElementSpawnData::new(0.8, ElementSpawnLocation::Atmosphere),
            PlanetElement::HydrogenCyanide  => ElementSpawnData::new(0.8, ElementSpawnLocation::Atmosphere),
            PlanetElement::Water            => ElementSpawnData::new(0.8, ElementSpawnLocation::Both),
            // In atmosphere from solar winds
            PlanetElement::Helium           => ElementSpawnData::new(0.8, ElementSpawnLocation::Atmosphere),

            PlanetElement::Lithium          => ElementSpawnData::new(0.8, ElementSpawnLocation::Core),
            PlanetElement::Carbon           => ElementSpawnData::new(0.8, ElementSpawnLocation::Core),
            PlanetElement::CarbonDioxide    => ElementSpawnData::new(0.8, ElementSpawnLocation::Both),
            PlanetElement::CarbonMonoxide   => ElementSpawnData::new(0.8, ElementSpawnLocation::Atmosphere),
            PlanetElement::Methane          => ElementSpawnData::new(0.8, ElementSpawnLocation::Both),
            PlanetElement::Nitrogen         => ElementSpawnData::new(0.8, ElementSpawnLocation::Atmosphere),
            PlanetElement::Ammonia          => ElementSpawnData::new(0.8, ElementSpawnLocation::Both),
            PlanetElement::AmmoniumSulfide  => ElementSpawnData::new(0.8, ElementSpawnLocation::Atmosphere),
            PlanetElement::Neon             => ElementSpawnData::new(0.8, ElementSpawnLocation::Atmosphere),

            // In atmosphere from meteorites vaporizing, can be emitted by the planet
            PlanetElement::Sodium           => ElementSpawnData::new(0.8, ElementSpawnLocation::Both),
            PlanetElement::TableSalt        => ElementSpawnData::new(0.8, ElementSpawnLocation::Core),
            PlanetElement::Magnesium        => ElementSpawnData::new(0.8, ElementSpawnLocation::Both),
            PlanetElement::Aluminum         => ElementSpawnData::new(0.8, ElementSpawnLocation::Core),
            PlanetElement::Silica           => ElementSpawnData::new(0.8, ElementSpawnLocation::Core),
            PlanetElement::Phosphorous      => ElementSpawnData::new(0.8, ElementSpawnLocation::Both),
            PlanetElement::Sulfur           => ElementSpawnData::new(0.8, ElementSpawnLocation::Core),
            PlanetElement::SulfurDioxide    => ElementSpawnData::new(0.8, ElementSpawnLocation::Atmosphere),
            PlanetElement::Chlorine         => ElementSpawnData::new(0.8, ElementSpawnLocation::Core),
            PlanetElement::Argon            => ElementSpawnData::new(0.8, ElementSpawnLocation::Atmosphere),

            // In atmosphere from meteorites vaporizing
            PlanetElement::Potassium        => ElementSpawnData::new(0.8, ElementSpawnLocation::Both),
            // In atmosphere from meteorites vaporizing
            PlanetElement::Calcium          => ElementSpawnData::new(0.8, ElementSpawnLocation::Both),
            PlanetElement::Titanium         => ElementSpawnData::new(0.8, ElementSpawnLocation::Core),
            PlanetElement::Chromium         => ElementSpawnData::new(0.8, ElementSpawnLocation::Core),
            PlanetElement::Iron             => ElementSpawnData::new(0.8, ElementSpawnLocation::Core),
            PlanetElement::Cobalt           => ElementSpawnData::new(0.8, ElementSpawnLocation::Core),
            PlanetElement::Nickel           => ElementSpawnData::new(0.8, ElementSpawnLocation::Core),
            PlanetElement::Copper           => ElementSpawnData::new(0.8, ElementSpawnLocation::Core),
            PlanetElement::Zinc             => ElementSpawnData::new(0.8, ElementSpawnLocation::Core),
            PlanetElement::Boromine         => ElementSpawnData::new(0.8, ElementSpawnLocation::Core),

            PlanetElement::Silver           => ElementSpawnData::new(0.8, ElementSpawnLocation::Core),
            PlanetElement::Iodine           => ElementSpawnData::new(0.8, ElementSpawnLocation::Core),

            PlanetElement::Tungstein        => ElementSpawnData::new(0.8, ElementSpawnLocation::Core),
            PlanetElement::Platinum         => ElementSpawnData::new(0.8, ElementSpawnLocation::Core),
            PlanetElement::Gold             => ElementSpawnData::new(0.8, ElementSpawnLocation::Core),
            PlanetElement::Mercury          => ElementSpawnData::new(0.8, ElementSpawnLocation::Core),
            PlanetElement::Lead             => ElementSpawnData::new(0.8, ElementSpawnLocation::Core),
            PlanetElement::Radon            => ElementSpawnData::new(0.8, ElementSpawnLocation::Atmosphere),

            PlanetElement::Thorium          => ElementSpawnData::new(0.8, ElementSpawnLocation::Core),
            PlanetElement::Uranium          => ElementSpawnData::new(0.8, ElementSpawnLocation::Core),
            PlanetElement::Plutonium        => ElementSpawnData::new(0.8, ElementSpawnLocation::Core),
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
    pub is_gas_giant: bool,

    /// The radius of the planet, in meters. Ranges from one million meters - about the radius of Pluto - to 700 
    /// million meters, about ten times the radius of Jupiter
    pub radius: u64,

    /// All the layers that compose this planet
    /// 
    /// The planet's layers start at the center and go to the edge of the playet's radius. They're then integrated to get the total mass of the planet, which determines its gravity. More gravity = more atmosphere, at least initially
    pub layers: Vec<PlanetLayer>,
}