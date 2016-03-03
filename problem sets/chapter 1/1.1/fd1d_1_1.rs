use std::fs::File;
use std::io;
use std::io::Write;

const KELEMENTCOUNT: usize = 200;

// macro to put formatting for the delta_ex
// table in one place.
macro_rules! delta_ex_format {
    () => ("{0:<6} | {1:<40}")
}

// macro to put formatting for the k_ex_hy
// table in one place.
macro_rules! k_ex_hy_format {
    () => ("{0:<3} | {1:<40} | {2:<40}")
}

/// Macro for printing a nice table header based
/// on the formatting used to produce the tables
/// rows.
///
/// This also prints a new line before the header
/// in order to give room between the previous
/// inputs or headers.
macro_rules! print_table_header {
    ($txt:expr) => {
        println!("");
        println!("{0}", $txt);
        println!("{0}", std::iter::repeat("-").take($txt.len()).collect::<String>());
    };
    ($fmt:expr, $($arg:tt)*) => ({
        let table_row = format!($fmt, $($arg)*).to_string();

        println!("");
        println!("{0}", table_row);
        println!("{0}", std::iter::repeat("-").take(table_row.len()).collect::<String>());
    });
}

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

        print_table_header!(delta_ex_format!(), "Delta", "Ex[kc]");
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
            println!(delta_ex_format!(), delta, ex[kc]);

            // Calculate the Hy field.
            for k in 0..KELEMENTCOUNT - 1 {
                hy[k] = hy[k] + 0.5f64 * (ex[k] - ex[k+1]);
            }
        }
        // End of the Main FDTD Loop.

        // At the end of the calculation, print out
        // the Ex and Hy fields.
        print_table_header!(k_ex_hy_format!(), "k", "Ex[k]", "Hy[k]");
        for k in 0..KELEMENTCOUNT -1 {
            println!(k_ex_hy_format!(), k, ex[k], hy[k]);
        }

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