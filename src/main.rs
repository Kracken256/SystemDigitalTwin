use clap::Parser;
use uom::si::{
    f64::*, pressure::psi, ratio::percent, thermodynamic_temperature::degree_fahrenheit,
    time::microsecond,
};

/// Digital Twin Simulation Runner
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Telemetry output interval in simulated seconds (e.g., 1.0 for every 1 simulated second)
    #[arg(short, long, default_value_t = 1.0)]
    report_interval: f64,

    /// Fixed simulation timestep in microseconds (e.g., 1000.0 for 1ms steps)
    #[arg(short, long, default_value_t = 1000.0)]
    dt: f64,
}

fn main() {
    let args = Args::parse();

    let report_interval = std::time::Duration::from_secs_f64(args.report_interval);
    let fixed_dt_duration = std::time::Duration::from_micros(args.dt as u64);
    let fixed_dt_uom = Time::new::<microsecond>(args.dt);

    let env = twin::SystemEnvironment {
        ambient_temperature: ThermodynamicTemperature::new::<degree_fahrenheit>(72.0),
        ambient_pressure: Pressure::new::<psi>(14.7),
        ambient_humidity: Ratio::new::<percent>(40.0),
    };

    let mut system = twin::System::new(env);

    let mut last_clock = std::time::Instant::now();
    let mut last_report = std::time::Instant::now();
    let mut accumulator = std::time::Duration::ZERO;

    let mut total_sim_time = std::time::Duration::ZERO;
    let mut step_count: u64 = 0;

    loop {
        let now = std::time::Instant::now();
        let frame_time = now.duration_since(last_clock);
        last_clock = now;

        accumulator += frame_time;

        while accumulator >= fixed_dt_duration {
            system.step(fixed_dt_uom);

            accumulator -= fixed_dt_duration;
            total_sim_time += fixed_dt_duration;
            step_count += 1;
        }

        if now.duration_since(last_report) >= report_interval {
            last_report = now;
            let report = system.report_state();

            let output_object = serde_json::json!({
                "simulation": {
                    "total_steps": step_count,
                    "simulated_time_s": total_sim_time.as_secs_f64(),
                },
                "system": report,
            });

            println!("{}", serde_json::to_string_pretty(&output_object).unwrap());
        }

        if accumulator < fixed_dt_duration {
            std::thread::yield_now();
        }
    }
}
