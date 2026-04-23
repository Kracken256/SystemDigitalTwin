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

    pub fn register_torque(&mut self, id: SignalId) -> Arc<RwLock<SignalBuffer<Torque>>> {
        let buffer = Arc::new(RwLock::new(SignalBuffer {
            front: Torque::new::<uom::si::torque::newton_meter>(0.0),
            back: Torque::new::<uom::si::torque::newton_meter>(0.0),
        }));
        self.torque.insert(id, buffer.clone());
        buffer
    }

    pub fn read_torque(&self, id: SignalId) -> Option<Torque> {
        self.torque.get(&id).map(|b| b.read().unwrap().front)
    }

    pub fn write_torque(&self, id: SignalId, value: Torque) {
        self.torque.get(&id).unwrap().write().unwrap().back = value;
    }

    pub fn register_angular_velocity(
        &mut self,
        id: SignalId,
    ) -> Arc<RwLock<SignalBuffer<AngularVelocity>>> {
        let buffer = Arc::new(RwLock::new(SignalBuffer {
            front: AngularVelocity::new::<uom::si::angular_velocity::radian_per_second>(0.0),
            back: AngularVelocity::new::<uom::si::angular_velocity::radian_per_second>(0.0),
        }));
        self.angular_velocity.insert(id, buffer.clone());
        buffer
    }

    pub fn read_angular_velocity(&self, id: SignalId) -> Option<AngularVelocity> {
        self.angular_velocity
            .get(&id)
            .map(|b| b.read().unwrap().front)
    }

    pub fn write_angular_velocity(&self, id: SignalId, value: AngularVelocity) {
        self.angular_velocity
            .get(&id)
            .unwrap()
            .write()
            .unwrap()
            .back = value;
    }

    pub fn register_moment_of_inertia(
        &mut self,
        id: SignalId,
    ) -> Arc<RwLock<SignalBuffer<MomentOfInertia>>> {
        let buffer = Arc::new(RwLock::new(SignalBuffer {
            front: MomentOfInertia::new::<uom::si::moment_of_inertia::kilogram_square_meter>(0.0),
            back: MomentOfInertia::new::<uom::si::moment_of_inertia::kilogram_square_meter>(0.0),
        }));
        self.moment_of_inertia.insert(id, buffer.clone());
        buffer
    }

    pub fn read_moment_of_inertia(&self, id: SignalId) -> Option<MomentOfInertia> {
        self.moment_of_inertia
            .get(&id)
            .map(|b| b.read().unwrap().front)
    }

    pub fn write_moment_of_inertia(&self, id: SignalId, value: MomentOfInertia) {
        self.moment_of_inertia
            .get(&id)
            .unwrap()
            .write()
            .unwrap()
            .back = value;
    }

    pub fn register_force(&mut self, id: SignalId) -> Arc<RwLock<SignalBuffer<Force>>> {
        let buffer = Arc::new(RwLock::new(SignalBuffer {
            front: Force::new::<uom::si::force::newton>(0.0),
            back: Force::new::<uom::si::force::newton>(0.0),
        }));
        self.force.insert(id, buffer.clone());
        buffer
    }

    pub fn read_force(&self, id: SignalId) -> Option<Force> {
        self.force.get(&id).map(|b| b.read().unwrap().front)
    }

    pub fn write_force(&self, id: SignalId, value: Force) {
        self.force.get(&id).unwrap().write().unwrap().back = value;
    }

    pub fn register_velocity(&mut self, id: SignalId) -> Arc<RwLock<SignalBuffer<Velocity>>> {
        let buffer = Arc::new(RwLock::new(SignalBuffer {
            front: Velocity::new::<uom::si::velocity::meter_per_second>(0.0),
            back: Velocity::new::<uom::si::velocity::meter_per_second>(0.0),
        }));
        self.velocity.insert(id, buffer.clone());
        buffer
    }

    pub fn read_velocity(&self, id: SignalId) -> Option<Velocity> {
        self.velocity.get(&id).map(|b| b.read().unwrap().front)
    }

    pub fn write_velocity(&self, id: SignalId, value: Velocity) {
        self.velocity.get(&id).unwrap().write().unwrap().back = value;
    }

    pub fn register_acceleration(
        &mut self,
        id: SignalId,
    ) -> Arc<RwLock<SignalBuffer<Acceleration>>> {
        let buffer = Arc::new(RwLock::new(SignalBuffer {
            front: Acceleration::new::<uom::si::acceleration::meter_per_second_squared>(0.0),
            back: Acceleration::new::<uom::si::acceleration::meter_per_second_squared>(0.0),
        }));
        self.acceleration.insert(id, buffer.clone());
        buffer
    }

    pub fn read_acceleration(&self, id: SignalId) -> Option<Acceleration> {
        self.acceleration.get(&id).map(|b| b.read().unwrap().front)
    }

    pub fn write_acceleration(&self, id: SignalId, value: Acceleration) {
        self.acceleration.get(&id).unwrap().write().unwrap().back = value;
    }

    pub fn register_displacement(&mut self, id: SignalId) -> Arc<RwLock<SignalBuffer<Length>>> {
        let buffer = Arc::new(RwLock::new(SignalBuffer {
            front: Length::new::<uom::si::length::meter>(0.0),
            back: Length::new::<uom::si::length::meter>(0.0),
        }));
        self.displacement.insert(id, buffer.clone());
        buffer
    }

    pub fn read_displacement(&self, id: SignalId) -> Option<Length> {
        self.displacement.get(&id).map(|b| b.read().unwrap().front)
    }

    pub fn write_displacement(&self, id: SignalId, value: Length) {
        self.displacement.get(&id).unwrap().write().unwrap().back = value;
    }

    pub fn register_strain(&mut self, id: SignalId) -> Arc<RwLock<SignalBuffer<Ratio>>> {
        let buffer = Arc::new(RwLock::new(SignalBuffer {
            front: Ratio::new::<uom::si::ratio::ratio>(0.0),
            back: Ratio::new::<uom::si::ratio::ratio>(0.0),
        }));
        self.strain.insert(id, buffer.clone());
        buffer
    }

    pub fn read_strain(&self, id: SignalId) -> Option<Ratio> {
        self.strain.get(&id).map(|b| b.read().unwrap().front)
    }

    pub fn write_strain(&self, id: SignalId, value: Ratio) {
        self.strain.get(&id).unwrap().write().unwrap().back = value;
    }

    pub fn register_pressure(&mut self, id: SignalId) -> Arc<RwLock<SignalBuffer<Pressure>>> {
        let buffer = Arc::new(RwLock::new(SignalBuffer {
            front: Pressure::new::<uom::si::pressure::pascal>(0.0),
            back: Pressure::new::<uom::si::pressure::pascal>(0.0),
        }));
        self.pressure.insert(id, buffer.clone());
        buffer
    }

    pub fn read_pressure(&self, id: SignalId) -> Option<Pressure> {
        self.pressure.get(&id).map(|b| b.read().unwrap().front)
    }

    pub fn write_pressure(&self, id: SignalId, value: Pressure) {
        self.pressure.get(&id).unwrap().write().unwrap().back = value;
    }

    pub fn register_mass_rate(&mut self, id: SignalId) -> Arc<RwLock<SignalBuffer<MassRate>>> {
        let buffer = Arc::new(RwLock::new(SignalBuffer {
            front: MassRate::new::<uom::si::mass_rate::kilogram_per_second>(0.0),
            back: MassRate::new::<uom::si::mass_rate::kilogram_per_second>(0.0),
        }));
        self.mass_rate.insert(id, buffer.clone());
        buffer
    }

    pub fn read_mass_rate(&self, id: SignalId) -> Option<MassRate> {
        self.mass_rate.get(&id).map(|b| b.read().unwrap().front)
    }

    pub fn write_mass_rate(&self, id: SignalId, value: MassRate) {
        self.mass_rate.get(&id).unwrap().write().unwrap().back = value;
    }

    pub fn register_volume_rate(&mut self, id: SignalId) -> Arc<RwLock<SignalBuffer<VolumeRate>>> {
        let buffer = Arc::new(RwLock::new(SignalBuffer {
            front: VolumeRate::new::<uom::si::volume_rate::cubic_meter_per_second>(0.0),
            back: VolumeRate::new::<uom::si::volume_rate::cubic_meter_per_second>(0.0),
        }));
        self.volume_rate.insert(id, buffer.clone());
        buffer
    }

    pub fn read_volume_rate(&self, id: SignalId) -> Option<VolumeRate> {
        self.volume_rate.get(&id).map(|b| b.read().unwrap().front)
    }

    pub fn write_volume_rate(&self, id: SignalId, value: VolumeRate) {
        self.volume_rate.get(&id).unwrap().write().unwrap().back = value;
    }

    pub fn register_density(&mut self, id: SignalId) -> Arc<RwLock<SignalBuffer<MassDensity>>> {
        let buffer = Arc::new(RwLock::new(SignalBuffer {
            front: MassDensity::new::<uom::si::mass_density::kilogram_per_cubic_meter>(0.0),
            back: MassDensity::new::<uom::si::mass_density::kilogram_per_cubic_meter>(0.0),
        }));
        self.density.insert(id, buffer.clone());
        buffer
    }

    pub fn read_density(&self, id: SignalId) -> Option<MassDensity> {
        self.density.get(&id).map(|b| b.read().unwrap().front)
    }

    pub fn write_density(&self, id: SignalId, value: MassDensity) {
        self.density.get(&id).unwrap().write().unwrap().back = value;
    }

    pub fn register_viscosity(
        &mut self,
        id: SignalId,
    ) -> Arc<RwLock<SignalBuffer<DynamicViscosity>>> {
        let buffer = Arc::new(RwLock::new(SignalBuffer {
            front: DynamicViscosity::new::<uom::si::dynamic_viscosity::pascal_second>(0.0),
            back: DynamicViscosity::new::<uom::si::dynamic_viscosity::pascal_second>(0.0),
        }));
        self.viscosity.insert(id, buffer.clone());
        buffer
    }

    pub fn read_viscosity(&self, id: SignalId) -> Option<DynamicViscosity> {
        self.viscosity.get(&id).map(|b| b.read().unwrap().front)
    }

    pub fn write_viscosity(&self, id: SignalId, value: DynamicViscosity) {
        self.viscosity.get(&id).unwrap().write().unwrap().back = value;
    }

    pub fn register_molar_concentration(
        &mut self,
        id: SignalId,
    ) -> Arc<RwLock<SignalBuffer<MolarConcentration>>> {
        let buffer = Arc::new(RwLock::new(SignalBuffer {
            front: MolarConcentration::new::<uom::si::molar_concentration::mole_per_cubic_meter>(
                0.0,
            ),
            back: MolarConcentration::new::<uom::si::molar_concentration::mole_per_cubic_meter>(
                0.0,
            ),
        }));
        self.molar_concentration.insert(id, buffer.clone());
        buffer
    }

    pub fn read_molar_concentration(&self, id: SignalId) -> Option<MolarConcentration> {
        self.molar_concentration
            .get(&id)
            .map(|b| b.read().unwrap().front)
    }

    pub fn write_molar_concentration(&self, id: SignalId, value: MolarConcentration) {
        self.molar_concentration
            .get(&id)
            .unwrap()
            .write()
            .unwrap()
            .back = value;
    }

    pub fn register_molar_mass(&mut self, id: SignalId) -> Arc<RwLock<SignalBuffer<MolarMass>>> {
        let buffer = Arc::new(RwLock::new(SignalBuffer {
            front: MolarMass::new::<uom::si::molar_mass::kilogram_per_mole>(0.0),
            back: MolarMass::new::<uom::si::molar_mass::kilogram_per_mole>(0.0),
        }));
        self.molar_mass.insert(id, buffer.clone());
        buffer
    }

    pub fn read_molar_mass(&self, id: SignalId) -> Option<MolarMass> {
        self.molar_mass.get(&id).map(|b| b.read().unwrap().front)
    }

    pub fn write_molar_mass(&self, id: SignalId, value: MolarMass) {
        self.molar_mass.get(&id).unwrap().write().unwrap().back = value;
    }

    pub fn register_temperature(
        &mut self,
        id: SignalId,
    ) -> Arc<RwLock<SignalBuffer<ThermodynamicTemperature>>> {
        let buffer = Arc::new(RwLock::new(SignalBuffer {
            front: ThermodynamicTemperature::new::<uom::si::thermodynamic_temperature::kelvin>(0.0),
            back: ThermodynamicTemperature::new::<uom::si::thermodynamic_temperature::kelvin>(0.0),
        }));
        self.temperature.insert(id, buffer.clone());
        buffer
    }

    pub fn read_temperature(&self, id: SignalId) -> Option<ThermodynamicTemperature> {
        self.temperature.get(&id).map(|b| b.read().unwrap().front)
    }

    pub fn write_temperature(&self, id: SignalId, value: ThermodynamicTemperature) {
        self.temperature.get(&id).unwrap().write().unwrap().back = value;
    }

    pub fn register_energy(&mut self, id: SignalId) -> Arc<RwLock<SignalBuffer<Energy>>> {
        let buffer = Arc::new(RwLock::new(SignalBuffer {
            front: Energy::new::<uom::si::energy::joule>(0.0),
            back: Energy::new::<uom::si::energy::joule>(0.0),
        }));
        self.energy.insert(id, buffer.clone());
        buffer
    }

    pub fn read_energy(&self, id: SignalId) -> Option<Energy> {
        self.energy.get(&id).map(|b| b.read().unwrap().front)
    }

    pub fn write_energy(&self, id: SignalId, value: Energy) {
        self.energy.get(&id).unwrap().write().unwrap().back = value;
    }

    pub fn register_power(&mut self, id: SignalId) -> Arc<RwLock<SignalBuffer<Power>>> {
        let buffer = Arc::new(RwLock::new(SignalBuffer {
            front: Power::new::<uom::si::power::watt>(0.0),
            back: Power::new::<uom::si::power::watt>(0.0),
        }));
        self.power.insert(id, buffer.clone());
        buffer
    }

    pub fn read_power(&self, id: SignalId) -> Option<Power> {
        self.power.get(&id).map(|b| b.read().unwrap().front)
    }

    pub fn write_power(&self, id: SignalId, value: Power) {
        self.power.get(&id).unwrap().write().unwrap().back = value;
    }

    pub fn register_heat_flux(
        &mut self,
        id: SignalId,
    ) -> Arc<RwLock<SignalBuffer<HeatFluxDensity>>> {
        let buffer = Arc::new(RwLock::new(SignalBuffer {
            front: HeatFluxDensity::new::<uom::si::heat_flux_density::watt_per_square_meter>(0.0),
            back: HeatFluxDensity::new::<uom::si::heat_flux_density::watt_per_square_meter>(0.0),
        }));
        self.heat_flux.insert(id, buffer.clone());
        buffer
    }

    pub fn read_heat_flux(&self, id: SignalId) -> Option<HeatFluxDensity> {
        self.heat_flux.get(&id).map(|b| b.read().unwrap().front)
    }

    pub fn write_heat_flux(&self, id: SignalId, value: HeatFluxDensity) {
        self.heat_flux.get(&id).unwrap().write().unwrap().back = value;
    }

    pub fn register_voltage(
        &mut self,
        id: SignalId,
    ) -> Arc<RwLock<SignalBuffer<ElectricPotential>>> {
        let buffer = Arc::new(RwLock::new(SignalBuffer {
            front: ElectricPotential::new::<uom::si::electric_potential::volt>(0.0),
            back: ElectricPotential::new::<uom::si::electric_potential::volt>(0.0),
        }));
        self.voltage.insert(id, buffer.clone());
        buffer
    }

    pub fn read_voltage(&self, id: SignalId) -> Option<ElectricPotential> {
        self.voltage.get(&id).map(|b| b.read().unwrap().front)
    }

    pub fn write_voltage(&self, id: SignalId, value: ElectricPotential) {
        self.voltage.get(&id).unwrap().write().unwrap().back = value;
    }

    pub fn register_current(&mut self, id: SignalId) -> Arc<RwLock<SignalBuffer<ElectricCurrent>>> {
        let buffer = Arc::new(RwLock::new(SignalBuffer {
            front: ElectricCurrent::new::<uom::si::electric_current::ampere>(0.0),
            back: ElectricCurrent::new::<uom::si::electric_current::ampere>(0.0),
        }));
        self.current.insert(id, buffer.clone());
        buffer
    }

    pub fn read_current(&self, id: SignalId) -> Option<ElectricCurrent> {
        self.current.get(&id).map(|b| b.read().unwrap().front)
    }

    pub fn write_current(&self, id: SignalId, value: ElectricCurrent) {
        self.current.get(&id).unwrap().write().unwrap().back = value;
    }

    pub fn register_resistance(
        &mut self,
        id: SignalId,
    ) -> Arc<RwLock<SignalBuffer<ElectricalResistance>>> {
        let buffer = Arc::new(RwLock::new(SignalBuffer {
            front: ElectricalResistance::new::<uom::si::electrical_resistance::ohm>(0.0),
            back: ElectricalResistance::new::<uom::si::electrical_resistance::ohm>(0.0),
        }));
        self.resistance.insert(id, buffer.clone());
        buffer
    }

    pub fn read_resistance(&self, id: SignalId) -> Option<ElectricalResistance> {
        self.resistance.get(&id).map(|b| b.read().unwrap().front)
    }

    pub fn write_resistance(&self, id: SignalId, value: ElectricalResistance) {
        self.resistance.get(&id).unwrap().write().unwrap().back = value;
    }

    pub fn register_frequency(&mut self, id: SignalId) -> Arc<RwLock<SignalBuffer<Frequency>>> {
        let buffer = Arc::new(RwLock::new(SignalBuffer {
            front: Frequency::new::<uom::si::frequency::hertz>(0.0),
            back: Frequency::new::<uom::si::frequency::hertz>(0.0),
        }));
        self.frequency.insert(id, buffer.clone());
        buffer
    }

    pub fn read_frequency(&self, id: SignalId) -> Option<Frequency> {
        self.frequency.get(&id).map(|b| b.read().unwrap().front)
    }

    pub fn write_frequency(&self, id: SignalId, value: Frequency) {
        self.frequency.get(&id).unwrap().write().unwrap().back = value;
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
