// functions3.rs
//
// Execute `rustlings hint functions3` or use the `hint` watch subcommand for a
// hint.

// I AM OVERENGINEERED

use std::fmt::format;

fn main() {
    print_parity(9);
}

fn print_parity(num: u32) {
    for i in 1..num + 1 {
        let parity = 
            if i % 2 == 0 {
                "even"
            } else {
                "odd"
            };

        let formatted_parity = format!("{i} is{parity:.>16}");
        if i == num {
            print!("{}", formatted_parity)
        } else {
            println!("{}", formatted_parity);
        }
    }
}
