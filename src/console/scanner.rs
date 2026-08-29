use super::Console;

/// * Defines the behavior when parsing invalid user input.
pub enum ScanMode {
    /// * Panics immediately upon receiving invalid input.
    Once,
    ///* Prompts the user continuously until valid input is received.
    Retry,
}

/// * Provides utility functions for reading and parsing command-line inputs.
pub struct Scanner {}
impl Scanner {
    /// * Reads a line from standard input and parses it into an `i32` integer.
    pub fn next_int() -> i32 {
        Console::read_line().parse().unwrap()
    }

    /// * Reads a line from standard input and parses it into an `i32` integer based on the specified `ScanMode`.
    pub fn next_int_with(mode: ScanMode) -> i32 {
        loop {
            let input = Console::read_line();

            match input.parse::<i32>() {
                Ok(value) => return value,
                Err(_) => match mode {
                    ScanMode::Once => panic!("Invalid integer."),
                    ScanMode::Retry => Console::write_line("Invalid integer. Try again."),
                },
            }
        }
    }

    /// * Reads a line from standard input and returns it directly as a `String`.
    pub fn next_string() -> String {
        Console::read_line()
    }

    /// * Reads a line from standard input and parses it into an `f64` float.
    pub fn next_float() -> f64 {
        Console::read_line().parse().unwrap()
    }

    /// * Reads a line from standard input and parses it into an `f64` float based on the specified `ScanMode`.
    pub fn next_float_with(mode: ScanMode) -> f64 {
        loop {
            let input = Console::read_line();

            match input.parse::<f64>() {
                Ok(value) => return value,
                Err(_) => match mode {
                    ScanMode::Once => panic!("Invalid float."),
                    ScanMode::Retry => Console::write_line("Invalid float. Try again."),
                },
            }
        }
    }
}
