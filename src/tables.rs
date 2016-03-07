use std::iter;

use electromagnetic_1d::*;

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
        println!("{0}", iter::repeat("-").take($txt.len()).collect::<String>());
    };
    ($fmt:expr, $($arg:tt)*) => ({
        let table_row = format!($fmt, $($arg)*).to_string();

        println!("");
        println!("{0}", table_row);
        println!("{0}", iter::repeat("-").take(table_row.len()).collect::<String>());
    });
}

pub fn print_ex_hy_1d(field: &ElectroMagnetic1D) {
     print_table_header!(k_ex_hy_format!(), "k", "Ex[k]", "Hy[k]");
     for k in 0..field.len() {
         println!(k_ex_hy_format!(), k, field.ex[k], field.hy[k]);
     }
}
