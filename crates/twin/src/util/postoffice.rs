use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use uom::si::f64::*;

/// A unique identifier for a specific signal (e.g., "MainEngineThrust")
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SignalId(pub &'static str);

pub struct SignalBuffer<T: Copy + Send + Sync + 'static> {
    pub front: T,
    pub back: T,
}

impl<T: Copy + Send + Sync + 'static> SignalBuffer<T> {
    pub fn flip(&mut self) {
        self.front = self.back;
    }
}

pub struct PostOffice {
    // --- Mechanical & Motion ---
    pub torque: HashMap<SignalId, Arc<RwLock<SignalBuffer<Torque>>>>,
    pub angular_velocity: HashMap<SignalId, Arc<RwLock<SignalBuffer<AngularVelocity>>>>,
    pub moment_of_inertia: HashMap<SignalId, Arc<RwLock<SignalBuffer<MomentOfInertia>>>>,
    pub force: HashMap<SignalId, Arc<RwLock<SignalBuffer<Force>>>>,
    pub velocity: HashMap<SignalId, Arc<RwLock<SignalBuffer<Velocity>>>>,
    pub acceleration: HashMap<SignalId, Arc<RwLock<SignalBuffer<Acceleration>>>>,
    pub displacement: HashMap<SignalId, Arc<RwLock<SignalBuffer<Length>>>>,
    pub strain: HashMap<SignalId, Arc<RwLock<SignalBuffer<Ratio>>>>,

    // --- Fluids & Chemical ---
    pub pressure: HashMap<SignalId, Arc<RwLock<SignalBuffer<Pressure>>>>,
    pub mass_rate: HashMap<SignalId, Arc<RwLock<SignalBuffer<MassRate>>>>,
    pub volume_rate: HashMap<SignalId, Arc<RwLock<SignalBuffer<VolumeRate>>>>,
    pub density: HashMap<SignalId, Arc<RwLock<SignalBuffer<MassDensity>>>>,
    pub viscosity: HashMap<SignalId, Arc<RwLock<SignalBuffer<DynamicViscosity>>>>,
    pub molar_concentration: HashMap<SignalId, Arc<RwLock<SignalBuffer<MolarConcentration>>>>,
    pub molar_mass: HashMap<SignalId, Arc<RwLock<SignalBuffer<MolarMass>>>>,

    // --- Thermal & Energy ---
    pub temperature: HashMap<SignalId, Arc<RwLock<SignalBuffer<ThermodynamicTemperature>>>>,
    pub energy: HashMap<SignalId, Arc<RwLock<SignalBuffer<Energy>>>>,
    pub power: HashMap<SignalId, Arc<RwLock<SignalBuffer<Power>>>>,
    pub heat_flux: HashMap<SignalId, Arc<RwLock<SignalBuffer<HeatFluxDensity>>>>,

    // --- Electrical & Logic ---
    pub voltage: HashMap<SignalId, Arc<RwLock<SignalBuffer<ElectricPotential>>>>,
    pub current: HashMap<SignalId, Arc<RwLock<SignalBuffer<ElectricCurrent>>>>,
    pub resistance: HashMap<SignalId, Arc<RwLock<SignalBuffer<ElectricalResistance>>>>,
    pub frequency: HashMap<SignalId, Arc<RwLock<SignalBuffer<Frequency>>>>,
}

impl PostOffice {
    pub fn new() -> Self {
        Self {
            torque: HashMap::new(),
            angular_velocity: HashMap::new(),
            moment_of_inertia: HashMap::new(),
            force: HashMap::new(),
            velocity: HashMap::new(),
            acceleration: HashMap::new(),
            displacement: HashMap::new(),
            strain: HashMap::new(),
            pressure: HashMap::new(),
            mass_rate: HashMap::new(),
            volume_rate: HashMap::new(),
            density: HashMap::new(),
            viscosity: HashMap::new(),
            molar_concentration: HashMap::new(),
            molar_mass: HashMap::new(),
            temperature: HashMap::new(),
            energy: HashMap::new(),
            power: HashMap::new(),
            heat_flux: HashMap::new(),
            voltage: HashMap::new(),
            current: HashMap::new(),
            resistance: HashMap::new(),
            frequency: HashMap::new(),
        }
    }

    /// Advances all signals from 'back' to 'front'.
    /// Call this once per global simulation tick.
    pub fn deliver_mail(&self) {
        // Mechanical
        self.torque.values().for_each(|b| b.write().unwrap().flip());
        self.angular_velocity
            .values()
            .for_each(|b| b.write().unwrap().flip());
        self.moment_of_inertia
            .values()
            .for_each(|b| b.write().unwrap().flip());
        self.force.values().for_each(|b| b.write().unwrap().flip());
        self.velocity
            .values()
            .for_each(|b| b.write().unwrap().flip());
        self.acceleration
            .values()
            .for_each(|b| b.write().unwrap().flip());
        self.displacement
            .values()
            .for_each(|b| b.write().unwrap().flip());
        self.strain.values().for_each(|b| b.write().unwrap().flip());

        // Fluids & Chemical
        self.pressure
            .values()
            .for_each(|b| b.write().unwrap().flip());
        self.mass_rate
            .values()
            .for_each(|b| b.write().unwrap().flip());
        self.volume_rate
            .values()
            .for_each(|b| b.write().unwrap().flip());
        self.density
            .values()
            .for_each(|b| b.write().unwrap().flip());
        self.viscosity
            .values()
            .for_each(|b| b.write().unwrap().flip());
        self.molar_concentration
            .values()
            .for_each(|b| b.write().unwrap().flip());
        self.molar_mass
            .values()
            .for_each(|b| b.write().unwrap().flip());

        // Thermal & Energy
        self.temperature
            .values()
            .for_each(|b| b.write().unwrap().flip());
        self.energy.values().for_each(|b| b.write().unwrap().flip());
        self.power.values().for_each(|b| b.write().unwrap().flip());
        self.heat_flux
            .values()
            .for_each(|b| b.write().unwrap().flip());

        // Electrical & Logic
        self.voltage
            .values()
            .for_each(|b| b.write().unwrap().flip());
        self.current
            .values()
            .for_each(|b| b.write().unwrap().flip());
        self.resistance
            .values()
            .for_each(|b| b.write().unwrap().flip());
        self.frequency
            .values()
            .for_each(|b| b.write().unwrap().flip());
    }
}
