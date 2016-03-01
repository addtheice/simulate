use std::fs::File;
use std::io;
use std::io::Write;

const KELEMENTCOUNT: usize = 200;

fn main () {
    let mut ex: [f64; KELEMENTCOUNT] = [0.0f64; KELEMENTCOUNT];
    let mut hy: [f64; KELEMENTCOUNT] = [0.0f64; KELEMENTCOUNT];

    let mut number_of_steps: i32 = 1;

    let t0: f64 = 0.0;

    let kc: usize = KELEMENTCOUNT/2;
    let pulse_t0: f64 = 40.0;
    const SPREAD: f64 = 12.0;
    let mut tick: f64 = 0.0;

    while number_of_steps > 0 {

        println!("Number Of Steps: ");
        let mut input_buffer = String::new();
        let _ = io::stdin().read_line(&mut input_buffer);
        number_of_steps = input_buffer.trim().parse::<i32>().unwrap();
        println!("{}", number_of_steps);

        for _ in 1..number_of_steps + 1 {
            tick = tick + 1.0f64;
            // Main FDTD Loop
            // Calculate the Ex field
            for k in 1..KELEMENTCOUNT {
                ex[k] = ex[k] + 0.5 * (hy[k-1] - hy[k]);
            }

            /* Put a Gaussian pulse in the middle */
            let delta: f64 = pulse_t0 - t0;
            let pulse = (-0.5f64 * (delta/SPREAD).powf(2.0f64) ).exp();
            ex[kc] = pulse;
            println!("{0:>010} {1:>010}", delta, ex[kc]);

            /* Calculate the Hy field */
            for k in 0..KELEMENTCOUNT - 1 {
                hy[k] = hy[k] + 0.5f64 * (ex[k] - ex[k+1]);
            }
        }

        /* End of the Main FDTD Loop */
        /* At the end of the calculation, print out
        the Ex and Hy fields */
        for k in 0..KELEMENTCOUNT -1 {
            println!("{0:>03} {1:>010} {2:>010}", k, ex[k], hy[k]);
        }

        /* Write the E field out to a file "Ex" */
        let mut e_field_file = File::create("Ex.txt").unwrap();
        for k in 1..KELEMENTCOUNT {
            writeln!(e_field_file, "{:<062}", ex[k]);
        }

        /* Write the H field out to a file "Hy" */
        let mut h_field_file = File::create("Hy.txt").unwrap();
        for k in 1..KELEMENTCOUNT {
            writeln!(h_field_file, "{:<062}", hy[k]);
        }

        println!("Tick count = {0:>010}", tick);
    }
}
