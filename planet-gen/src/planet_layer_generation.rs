use initial_planet_data::*;

pub fn gen_layers(planet_type: PlanetType, radius: u64) -> Vec<PlanetLayer> {
    match planet_type {
        PlanetType::Chrondite => gen_chondrite_layers(radius),
        PlanetType::Gassy => gen_gassy_layers(radius),
        PlanetType::Icy => gen_icy_layers(radius),
    }
}

fn gen_chondrite_layers(radius: u64) -> Vec<PlanetLayer> {
    let mut layers = Vec::<PlanetLayer>::new();

    layers
}

fn gen_gassy_layers(radius: u64) -> Vec<PlanetLayer> {
    let mut layers = Vec::<PlanetLayer>::new();

    layers
}

fn gen_icy_layers(radius: u64) -> Vec<PlanetLayer> {
    let mut layers = Vec::<PlanetLayer>::new();

    layers
}

