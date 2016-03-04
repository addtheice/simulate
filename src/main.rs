extern crate gnuplot;

use std::fs::File;
use std::io;
use std::io::Write;

#[macro_use]
mod tables;

use gnuplot::*;

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

    let kc: usize = KELEMENTCOUNT/2;
    let pulse_t0: f64 = 40.0;
    const SPREAD: f64 = 12.0;
    let mut tick: f64 = 0.0;

    print!("Number Of Steps: ");
    let _ = io::stdout().flush();
    let mut input_buffer = String::new();
    let _ = io::stdin().read_line(&mut input_buffer);
    let mut number_of_steps = input_buffer.trim().parse::<i32>().unwrap();

    while number_of_steps > 0 {

        for _ in 1..number_of_steps + 1 {
            tick = tick + 1.0f64;
            // Main FDTD Loop
            // Calculate the Ex field.
            for k in 1..KELEMENTCOUNT {
                ex[k] = ex[k] + 0.5 * (hy[k-1] - hy[k]);
            }

            // Put a Gaussian pulse in the middle.
            let delta: f64 = pulse_t0 - tick;
            let pulse = (-0.5f64 * (delta/SPREAD).powf(2.0f64) ).exp();
            ex[kc] = pulse;

            // Calculate the Hy field.
            for k in 0..KELEMENTCOUNT - 1 {
                hy[k] = hy[k] + 0.5f64 * (ex[k] - ex[k+1]);
            }
        }
        // End of the Main FDTD Loop.

        // Produce the gnuplot chart of the Ex pulse.
        let mut figure = Figure::new();
        figure.axes2d()
            .set_title("Figure 1.2", &[])
            .set_x_label("FDTD cells", &[])
            .set_y_label("Ex", &[])
            .set_size(1.0, 0.50)
            .set_pos(0.0,0.5)
            .set_y_ticks(Some((Fix(1.0), 0)), &[Mirror(false)], &[])
            .set_x_ticks(Some((Fix(20.0), 0)), &[Mirror(true)], &[])
            .lines(0..200, ex.iter(), &[Color("black")]);

        // Produce the gnuplot chart of the Hy pulse.
        figure.axes2d()
            .set_title("Figure 1.2", &[])
            .set_x_label("FDTD cells", &[])
            .set_y_label("Hy", &[])
            .set_size(1.0, 0.50)
            .set_y_ticks(Some((Fix(1.0), 0)), &[Mirror(false)], &[])
            .set_x_ticks(Some((Fix(20.0), 0)), &[Mirror(true)], &[])
            .lines(0..200, hy.iter(), &[Color("black")]);
        figure.show();

        // Write the E field out to a file "Ex".
        let mut e_field_file = File::create("Ex.txt").unwrap();
        for k in 1..KELEMENTCOUNT {
            writeln!(e_field_file, "{:<40}", ex[k]);
        }

        // Write the H field out to a file "Hy".
        let mut h_field_file = File::create("Hy.txt").unwrap();
        for k in 1..KELEMENTCOUNT {
            writeln!(h_field_file, "{:<40}", hy[k]);
        }

        println!("Tick count = {0:<4}", tick);

        print!("Number Of Steps: ");
        let _ = io::stdout().flush();
        let mut input_buffer = String::new();
        let _ = io::stdin().read_line(&mut input_buffer);
        number_of_steps = input_buffer.trim().parse::<i32>().unwrap();

    }
}
