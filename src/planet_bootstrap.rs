/*! 
 * \brief Generates the initial values for the planets: size, distance from sun, composition, etc
 */

/*!
 * \brief The very initial parameters for planet generation. Essentially the seed for the planet generation algorithm
 */
struct InitialPlanetParams {
    /*!
     * \brief The distance, in meters, from the planet to its star. Ranges from 0 to 10 billion, which mimics Earth's 
     * solar system
     */
    pub distance_from_sun : u64 = 0;

    /*!
     * \brief The age of the planet when the player finds it, in Earth years. Ranges from one thousand to thirteen 
     * billion - Can't have planets older than the universe!
     */
    pub age : u64 = 0;

    /*!
     * \brief Whether to generate this planet as a gas giant
     *
     * Planets smaller than a certain size can't be gaseous because they don't have the necessary gravity to hold a
     * thick hydrogen and helium atmosphere, so this field is tied to planetary mass - but it's not directly dependent 
     * on it
     */
    pub is_gas_giant : bool = false;

    /*!
     * \brief The radius of the planet, in meters. Ranges from one million meters - about the radius of pluto - to 
     */
    pub radius : u64 = 0;

    /*!
     * \brief The mass of the planet, in meters
     */
    pub mass : u64 = 0;
};