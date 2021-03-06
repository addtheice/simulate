extern crate simulate;

use std::io;
use std::io::Write;

use simulate::electromagnetic_1d::*;
use simulate::charts;

const KELEMENTCOUNT: usize = 200;

/// Currently this program does not follow rust styling
/// in regards to input validation and sanitization and
/// skips all forms of error checking and modularity.
///
/// This works as an example program, but in the future
/// expanding to a real time graphing and GPU processing
/// will occur in multiple dimensions.
///
/// For now, this works as intended as an example as to
/// the technique.
fn main () {
    // Gaussian pulse information
    const SPREAD: f64 = 12.0;
    let kc: usize = KELEMENTCOUNT/2;
    let pulse_t0: f64 = 40.0;

    let mut field = ElectroMagnetic1D::new(KELEMENTCOUNT);

    // Simulation time.
    let mut tick: f64 = 0.0;

    print!("Number Of Steps: ");
    let _ = io::stdout().flush();
    let mut input_buffer = String::new();
    let _ = io::stdin().read_line(&mut input_buffer);
    let mut number_of_steps = input_buffer.trim().parse::<i32>().unwrap();

    while number_of_steps > 0 {

        for _ in 0..number_of_steps {
            tick = tick + 1.0f64;
            // Main FDTD Loop
            field.tick_ex();

            // Put a Gaussian pulse in the middle.
            let delta: f64 = pulse_t0 - tick;
            let pulse = (-0.5f64 * (delta / SPREAD).powi(2)).exp();
            field.ex[kc] = pulse;

            // Calculate the Hy field.
            field.tick_hy();
        }
        // End of the Main FDTD Loop.

        // Produce the gnuplot chart of the Ex / Hy.
        charts::chart_ex_hy(&field, &tick);

        println!("Tick count = {0:<4}", tick);

        print!("Number Of Steps: ");
        let _ = io::stdout().flush();
        let mut input_buffer = String::new();
        let _ = io::stdin().read_line(&mut input_buffer);
        number_of_steps = input_buffer.trim().parse::<i32>().unwrap();

    }
}
