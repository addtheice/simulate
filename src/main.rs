use std::fs::File;
use std::io;
use std::io::Write;

const KELEMENTCOUNT: usize = 200;

/// This is a mostly direct translation of the FD1D_1.1.C
/// 1D FDTD simulation in free space program found at the
/// end of chapter 1.
///
/// The major differences are that I've converted the code
/// to rust, formatted it to follow rust styling, changed
/// some of the variables to match a more 'rustonic'
/// styling and removed any unused variables.
///
/// Currently this program does not follow rust styling
/// in regards to input validation and sanitization and
/// skips all forms of error checking and modularity.
///
/// This works as an example program, but in the future
/// expanding to a real time graphing, expanded sizes,
/// and GPU processing will occur in multiple dimensions.
///
/// For now, this works as intended as an example as to
/// the technique.
///
/// As new programs are added, this file will be moved
/// into a sub folder.
fn main () {
    let mut ex: [f64; KELEMENTCOUNT] = [0.0f64; KELEMENTCOUNT];
    let mut hy: [f64; KELEMENTCOUNT] = [0.0f64; KELEMENTCOUNT];

    let t0: f64 = 0.0;
    let kc: usize = KELEMENTCOUNT/2;
    let pulse_t0: f64 = 40.0;
    const SPREAD: f64 = 12.0;
    let mut tick: f64 = 0.0;

    println!("Number Of Steps: ");
    let mut input_buffer = String::new();
    let _ = io::stdin().read_line(&mut input_buffer);
    let mut number_of_steps = input_buffer.trim().parse::<i32>().unwrap();
    println!("{}", number_of_steps);

    while number_of_steps > 0 {

        for _ in 1..number_of_steps + 1 {
            tick = tick + 1.0f64;
            // Main FDTD Loop
            // Calculate the Ex field.
            for k in 1..KELEMENTCOUNT {
                ex[k] = ex[k] + 0.5 * (hy[k-1] - hy[k]);
            }

            // Put a Gaussian pulse in the middle.
            let delta: f64 = pulse_t0 - t0;
            let pulse = (-0.5f64 * (delta/SPREAD).powf(2.0f64) ).exp();
            ex[kc] = pulse;
            println!("{0:>010} {1:>010}", delta, ex[kc]);

            // Calculate the Hy field.
            for k in 0..KELEMENTCOUNT - 1 {
                hy[k] = hy[k] + 0.5f64 * (ex[k] - ex[k+1]);
            }
        }

        // End of the Main FDTD Loop.
        // At the end of the calculation, print out
        // the Ex and Hy fields.
        for k in 0..KELEMENTCOUNT -1 {
            println!("{0:>03} {1:>010} {2:>010}", k, ex[k], hy[k]);
        }

        // Write the E field out to a file "Ex".
        let mut e_field_file = File::create("Ex.txt").unwrap();
        for k in 1..KELEMENTCOUNT {
            writeln!(e_field_file, "{:<062}", ex[k]);
        }

        // Write the H field out to a file "Hy".
        let mut h_field_file = File::create("Hy.txt").unwrap();
        for k in 1..KELEMENTCOUNT {
            writeln!(h_field_file, "{:<062}", hy[k]);
        }

        println!("Tick count = {0:>010}", tick);

        println!("Number Of Steps: ");
        let mut input_buffer = String::new();
        let _ = io::stdin().read_line(&mut input_buffer);
        number_of_steps = input_buffer.trim().parse::<i32>().unwrap();
        println!("{}", number_of_steps);
    }
}
