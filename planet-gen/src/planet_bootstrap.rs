/// Generates the initial values for the planets: size, distance from sun, composition, etc

/// All the elements that a planet can be made out of
enum PlanetElement {
    Hydrogen,
    Helium,
    Carbon,
    Water,
    Methane,
    Ammonia,
    Iron,
    Silica,
    Nitrogen,
    SulfurDioxide,
    Argon,
    Neon,
};

/// A layer in a planet. ALl parts of the planet, including its atmosphere, are layers
struct PlanetLayer {
    /// The elemental(ish) composition of this layer. 
    pub composition : HashMap<PlanetElement, f32>;

    /// The distance from the planet's center, in meters, where the layer starts
    pub start_height : u64;

    /// The distance from the planet's center, in meters, where this layer ends
    pub end_height : u64;
}

/// The very initial parameters for planet generation. Essentially the seed for the planet generation algorithm
struct InitialPlanetParams {
    /// The distance, in meters, from the planet to its star. Ranges from 0 to 10 billion, which mimics Earth's solar
    /// system
    pub distance_from_sun : u64 = 0;

    pub orbital_period : f32 = 0;

    /// The age of the planet when the player finds it, in Earth years. Ranges from one thousand to thirteen billion 
    /// - Can't have planets older than the universe!
    pub age : u64 = 0;

    /// Whether to generate this planet as a gas giant
    /// 
    /// Planets smaller than a certain size can't be gaseous because they don't have the necessary gravity to hold a 
    /// thick hydrogen and helium atmosphere, so this field is tied to planetary mass - but it's not directly 
    /// dependent on it
    pub is_gas_giant : bool = false;

    /// The radius of the planet, in meters. Ranges from one million meters - about the radius of Pluto - to 700 
    /// million meters, about ten times the radius of Jupiter
    pub radius : u64 = 0;

    /// The mass of the planet, in meters
    pub mass : u64 = 0;

    /// All the layers that compose this planet
    pub layers : Vec<PlanetLayer>;
};