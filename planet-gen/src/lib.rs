#![feature(extern_prelude)]

/// The functions in this file generate a planet, psuedo-randomly. While realism is a strong concern, I have made many
/// assumptions where little or no real-world data was available, and have broken from reality a bit to generate 
/// intersting, unique planets. E.G. IRL a gas giant in one solar system has little to no effect on the possibility of
/// a gas giant in a far-off solar system, but 
/// 
/// When a new procedural planet is spawned, the generation happens on a separate thread. Simple parameters such as 
/// distance from the planet's star and basic shape are generated immediately so that the player can see something 
/// useful, the the complex planet generation happens on a separate thread. 
/// 
/// What is that generation? Wellllllllll...
/// 
/// First, I look at the basic parameters. Things such as distance from the sun, gaseousness, etc are taken into 
/// consideration. Gassy planets are just gassy, they get some spin and that's that. They have a small core, but that's
/// pretty boring and maybe it's a liquid and liquid are hard to land on - so I don't worry too much about that
/// 
/// Solid planets are more interesting. Based on their size, I calcualte gravitational heating and the longevity of 
/// their cores - which, combined with the age of the planet, informs the amount of atmosphere the planet gets. I 
/// simulate basic weather patterns for the duration of the atmosphere's lifetime - oceans (low areas) heat up slower
/// than the land (or vice versa - will confirm), which informs wind patterns, which informs cloud patterns - although I 
/// also consider mountains and other terrain features blocking clouds/rainfall. Then I simulate rainfall, which 
/// determines weathering and biomes
/// 
/// While I'm simulating weather, I'm also simulating plate techtonics. This is all done iteratively (will determine 
/// perfect time increment for maximum planet quality vs generation time) like in real life. Basicaly, plate techtonics
/// is simulated by generating plates - randomly, although imma try to keep the plate size consistent across planet 
/// sizes - then plates are assigned a movement direction randomly (unless research comes up with a better answer). 
/// They're moved along their movement direction, which may result in new faults if a plate is pulled in multiple 
/// directions. Subduction/sublimation/whatever it's called when mountains form is fully simulated with each iteration,
/// so that the terrain can inform weather patterns and whatnot. The weather patterns can and will changes as a planet 
/// changes
/// 
/// Rainfall and latitude determines biome. Earth is used as a model since we have literally no other data points for 
/// this sort of thing. Fauna and flora are based on what one might expect on Earth - areas closer to the equator have 
/// more colorful flora and fauna, colors get more muted near the poles. None of this "the whole planet is a single 
/// biome" bs - going from one pole to another should have lots of interesting variation, as should circumnavigating a 
/// planet. If I get bored enough I may try to simulate basic evolution, with flora and fauna randomly gaining or 
/// losing traits, but that's a ways off (if at all)
/// 
/// At this point we can (maybe) place a civilization on this planet. Civilizations are largely based on what generates
/// interesting planets. There's various probabilities of reaching each stage of evolution - plants, fish, reptiles, 
/// mammals and birds, bipeds, Type I civilization, Type II, Type III... etc. Type I civs and later are pretty 
/// interesting because they (like our human civilization) have a lot of chances to fail. They could fail to climate 
/// change or war, and each has a different impact on the planet. More climate change means more extreme weather on the 
/// planet, whilce war leaves behind scars upon the terrain. These scars are based on the tech level of the 
/// civilization, with Type I leaving behind irradiated craters from nuclear bombs, Type II creating gorges that reach 
/// the lower crust, and Type III shattering entier planets.
/// 
/// Every time a civilization falls, there's chance for another one to rise again. Each fallen civilization halves the 
/// time until the next civilization rises - a civilization builds upon its predecessors
///

extern crate rand;

pub mod planet_bootstrap;

mod initial_planet_data;

use initial_planet_data::InitialPlanetParams;

#[no_mangle]
pub extern "C" fn gen_planet() {
    let planet = InitialPlanetParams::new();
}
