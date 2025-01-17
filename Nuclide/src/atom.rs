use crate::particle::Particle;

/// Shared trait for atoms 
pub trait Atom: Clone {
    /// Atomic number
    fn atomic_num(&self) -> u64;
    /// Identifying string
    fn identity(&self) -> String;
    /// Atomic mass in Daltons
    fn am(&self) -> f64;
    /// Atomic mass in kilograms
    fn am_kg(&self) -> f64;

    //fn am_ev(&self) -> f64;
    /// Mass defect or the difference between the empirical mass and the mass of the constituents, in Daltons
    fn mass_deficit(&self) -> f64;
    // Mass defect in Kilograms
    fn mass_deficit_kg(&self) -> f64;
    /// Mass defect in Joules
    fn mass_deficit_j(&self) -> f64;
    /// Mass defect in MeV  fix these, same as binding energy
    fn mass_deficit_ev(&self) -> f64;

    fn binding_energy(&self) -> f64;

    fn binding_energy_j(&self) -> f64;
    /// Spin as a i8 pair
    fn spin_parity(&self) -> (i8, i8);
    /// Electron affinity in kj/mol
    fn electron_affinity(&self) -> f64;
    /// Electron affinity in MeV
    fn electron_affinity_ev(&self) -> f64;
    ///Returns the ionization energies for all known levels. Values are in kj/mol
    fn ionization_energies(&self, level: usize) -> Option<f64>;
    ///Returns the ionization energies for all known levels. Values are in MeV
    fn ionization_energies_ev(&self, level: usize) -> Option<f64>;
    /// Returns Oganov-Tantardini values, the current best evaluation
    fn electronegativity(&self) -> f64;
    /// Mullikan electronegativity
    fn mullikan_en(&self) -> f64;
    /// Allen electronegativity
    fn allen_en(&self) -> f64;
    /// Pauling electronegativity
    fn pauling_en(&self) -> f64;
    /// Covalent radii of the first three bonds
    fn covalent_radii(&self, bond: usize) -> Option<f64>;
    /// Ionic radii
    fn ionic_radii(&self) -> f64;
    /// Van der Waal radius in crystalline structure
    fn vdr_crystal(&self) -> f64;
    /// Van der Waal radius in isolated atoms
    fn vdr_isolated(&self) -> f64;
    /// Half-life of nuclide/isomer
    fn half_life(&self) -> f64;
    /// The mean lifetime of nuclide/isomer
    fn mean_lifetime(&self) -> f64;
    /// Returns the probable decay modes as a string
    fn decay_mode(&self) -> String;
    /// Returns decay constant
    fn decay_constant(&self) -> f64;
    /// Checks if nuclide/isomer would decay in the selected time
    fn decay_time(&self, time: f64) -> bool;
    /// Decays nuclide/isomer in-place a maximum of 1 time. Returning a tuple in the form of the energy imparted to the nuclide/isomer and a vector of particles with decay energies
    fn static_decay(&mut self, time: f64) -> (f64, Vec<Particle>);
    /// Continously performs decay throughout the time selected, collecting all particles into a vector with decay energies.
    fn decay(&mut self, time: f64) -> (f64, Vec<Particle>);
}
