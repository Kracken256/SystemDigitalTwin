use uom::si::{
    f64::*, pressure::psi, ratio::percent, thermodynamic_temperature::degree_fahrenheit,
    time::microsecond,
};

fn main() {
    let env = twin::SystemEnvironment {
        ambient_temperature: ThermodynamicTemperature::new::<degree_fahrenheit>(72.0),
        ambient_pressure: Pressure::new::<psi>(14.7),
        ambient_humidity: Ratio::new::<percent>(40.0),
    };

    let mut system = twin::System::new(env);
    let mut last = std::time::Instant::now();
    let mut last_report = std::time::Instant::now();
    let mut dt_sum = std::time::Duration::new(0, 0);
    let mut step_count = 0;

    loop {
        let now = std::time::Instant::now();
        let dt = now.duration_since(last);

        // Throttle the loop to prevent it from running too fast when dt is very small
        if dt.as_millis() < 10 {
            std::thread::sleep(std::time::Duration::from_millis(10) - dt);
            continue;
        }

        system.step(Time::new::<microsecond>(dt.as_micros() as f64));
        last = now;
        dt_sum += dt;
        step_count += 1;

        let report_dt = now.duration_since(last_report);
        if report_dt.as_secs() >= 1 {
            last_report = now;
            let report = system.report_state();
            let avg_dt = dt_sum / step_count;

            println!("{}", serde_json::to_string_pretty(&report).unwrap());
            println!(
                "Average Step Time: {:.2} ms ({} steps in {:.2} seconds)",
                avg_dt.as_secs_f64() * 1000.0,
                step_count,
                dt_sum.as_secs_f64()
            );
        }
    }
}
