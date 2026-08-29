use jstdlib::console::Console;
use jstdlib::write_line;

fn main() {
    Console::write_line("JSTD Console Demo");

    let number: i32 = Console::input("Enter an integer: ");

    let decimal: f64 = Console::input("Enter a decimal: ");

    let text: String = Console::input("Enter some text: ");

    write_line!("\nYou entered:");
    write_line!("Integer: {}\nFloat: {}\nString: {}", number, decimal, text);
}
