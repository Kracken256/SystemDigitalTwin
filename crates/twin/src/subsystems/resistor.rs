use crate::{PostOffice, SignalId, Subsystem, SystemEnvironment};
use uom::si::f64::*;
use uom::si::{electric_current::ampere, electric_potential::volt, electrical_resistance::ohm};

pub struct ResistorConfig {
    pub resistance: ElectricalResistance,
    pub forward_voltage: ElectricPotential,
    pub voltage_input: SignalId,
    pub voltage_output: SignalId,
}

pub struct ResistorSubsystem {
    config: ResistorConfig,

    voltage_input: SignalId,
    current_demand: SignalId,
    voltage_output: SignalId,
}

impl ResistorSubsystem {
    pub fn new(po: &mut PostOffice, config: ResistorConfig) -> Self {
        po.register::<ElectricPotential>(config.voltage_output);

        Self {
            voltage_input: config.voltage_input,
            current_demand: SignalId::from("battery_current_demand"),
            voltage_output: config.voltage_output,
            config,
        }
    }
}

impl Subsystem for ResistorSubsystem {
    fn step(&mut self, po: &mut PostOffice, _env: &SystemEnvironment, _dt: Time) {
        // Safely get battery voltage; default to 0.0V if not yet published
        let v_bat = *po
            .read::<ElectricPotential>(self.voltage_input)
            .unwrap_or(&ElectricPotential::new::<volt>(0.0));

        // Calculate I = (V_in - V_f) / R
        let current = if v_bat > self.config.forward_voltage {
            (v_bat - self.config.forward_voltage) / self.config.resistance
        } else {
            ElectricCurrent::new::<ampere>(0.0)
        };

        let v_output = if v_bat > self.config.forward_voltage {
            self.config.forward_voltage
        } else {
            v_bat
        };

        po.accumulate::<ElectricCurrent>(self.current_demand, current);
        po.write::<ElectricPotential>(self.voltage_output, v_output);
    }

    fn report_state(&self) -> serde_json::Value {
        serde_json::json!({
            "resistance_ohms": self.config.resistance.get::<ohm>(),
            "forward_voltage_volts": self.config.forward_voltage.get::<volt>(),
        })
    }
}
