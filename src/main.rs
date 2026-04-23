use twin::{PostOffice, Subsystem};
use uom::si::{f64::Time, time::millisecond};

fn main() {
    let mut system = twin::System::new();

    loop {
        system.step(Time::new::<millisecond>(1.0));

        let report = system.report_state();
        println!("{}", serde_json::to_string_pretty(&report).unwrap());
    }
}
