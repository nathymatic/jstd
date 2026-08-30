use std::fmt::Display;
use std::io::{self, Write};
use std::str::FromStr;

use crossterm::event::{self, Event, KeyEvent};
use rpassword::read_password;

/// * Provides utility functions for immediate console input and output.
pub struct Console {}

impl Console {
    /// * Prints a value to the standard output and immediately flushes the stream.
    pub fn write<T: Display>(value: T) {
        print!("{value}");
        io::stdout().flush().unwrap();
    }

    /// * Prints a value to the standard output followed by a newline character.
    pub fn write_line<T: Display>(value: T) {
        println!("{value}");
    }

    /// * Reads a line from standard input and parses the user input into the requested type `T` with trailing newlines removed.
    pub fn read<T>() -> T
    where
        T: FromStr,
    {
        Self::read_line().parse::<T>().ok().unwrap()
    }

    /// * Reads a line from standard input and returns it with trailing newlines removed.
    pub fn read_line() -> String {
        let mut input = String::new();

        io::stdin().read_line(&mut input).unwrap();

        input.trim_end().to_string()
    }

    /// * Reads and returns the next key event from standard input.
    pub fn read_key() -> io::Result<KeyEvent> {
        loop {
            if let Event::Key(key) = event::read()? {
                return Ok(key);
            }
        }
    }

    /// * Displays a prompt and parses the user input into the requested type `T`.
    pub fn input<T>(prompt: &str) -> T
    where
        T: FromStr,
    {
        Self::write(prompt);
        Self::read_line().parse::<T>().ok().unwrap()
    }

    /// * Prompts the user for a yes/no response.
    pub fn confirm(prompt: &str) -> bool {
        loop {
            let input: String = Self::input(prompt);

            match input.trim().to_lowercase().as_str() {
                "y" | "yes" => return true,
                "n" | "no" => return false,
                _ => Self::write_line("Please enter yes or no."),
            }
        }
    }

    /// * Displays a prompt and reads user input without displaying the entered characters.
    pub fn password(prompt: &str) -> io::Result<String> {
        Self::write(prompt);
        read_password()
    }

    /// * Clears the terminal screen.
    pub fn clear() {
        print!("\x1B[2J\x1B[1;1H");
    }
}

/// * Formats and prints arguments to the standard output.
#[macro_export]
macro_rules! write {
    ($($arg:tt)*) => {
        print!($($arg)*);
    }
}

/// * Formats and prints arguments to the standard output followed by a newline character.
#[macro_export]
macro_rules! write_line {
    ($($arg:tt)*) => {
        println!($($arg)*);
    };
}