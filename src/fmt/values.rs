//! Value reporting macro.

/// Report a set of value and their associated identifiers.
#[macro_export]
macro_rules! values {
    ($($val: expr),*) => {
        {
            $(
                println!("{:>name_width$} : {:<val_width$}", stringify!($val), $val, name_width = 14, val_width = 15);
            )*
        }
    };
}

// /// Print to equally spaced name-value pairs in columns.
// #[macro_export]
// macro_rules! values {
//     ($num_cols: expr, $total_width: expr, $($val: expr),*) => {
//         {
//             use std::fmt::Write;

//             let col_width = $total_width / $num_cols;
//             let name_width = col_width - 2;
//             let val_width = col_width - 1;

//             let mut string = String::new();
//             $(write!(string, "{:>name_width$} : {:<val_width$}", $val, stringify!($val), name_width = name_width, val_width = val_width).expect("Failed to write to string buffer.");)*

//             string
//         }
//     };
// }
