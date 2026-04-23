use uom::si::{
    f64::*, pressure::psi, ratio::percent, thermodynamic_temperature::degree_fahrenheit,
    time::millisecond,
};

fn main() {
    let env = twin::SystemEnvironment {
        ambient_temperature: ThermodynamicTemperature::new::<degree_fahrenheit>(72.0),
        ambient_pressure: Pressure::new::<psi>(14.7),
        ambient_humidity: Ratio::new::<percent>(40.0),
    };

    let mut system = twin::System::new(env);

    loop {
        system.step(Time::new::<millisecond>(1.0));

        let report = system.report_state();
        println!("{}", serde_json::to_string_pretty(&report).unwrap());
    }
}
