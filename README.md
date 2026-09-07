# jstdlib

> some practical Rust utilities because i got tired of writing the same crap over and over

## About

basically, this is my little Rust utility crate for things i find useful enough to not want to rewrite every time

it currently has utilities for console input/output, collections, and math, with more stuff probably getting added as i go

## Features

* `console` -- convenient console input and output
* `collections` -- useful helpers for vectors and slices
* `math` -- common mathematical utilities

## Requirements

* Rust
* Cargo

## Installation

```bash
cargo add jstdlib
```

or add it manually to your `Cargo.toml`:

```toml
[dependencies]
jstdlib = "0.1.1"
```

## Usage

### Console

```rust
use jstdlib::console::Console;

Console::write_line("Hello, world!");

let name: String = Console::input("What's your name? ");
let age: i32 = Console::input("How old are you? ");

Console::write_line(&format!("Hello, {name}!"));
Console::write_line(&format!("You are {age} years old."));
```

you can also use the formatting macros if you wanna keep things concise or don't wanna use the ugly &format!() boilerplate:

```rust
use jstdlib::write_line;

let name = "Someone";
let age = 21;

write_line!("Name: {name}");
write_line!("Age: {age}");
```

### Collections

```rust
use jstdlib::collections::vec;

let numbers = vec![1, 2, 3, 4, 5, 6];

let even = vec::filter(&numbers, |x| x % 2 == 0);
let doubled = vec::map(&numbers, |x| x * 2);
let found = vec::find(&numbers, |x| *x > 4);

println!("{even:?}");
println!("{doubled:?}");
println!("{found:?}");
```

currently available:

* `filter`
* `map`
* `find`
* `any`
* `all`
* `count`
* `unique`
* `chunk`
* `partition`

i might've genuinely missed some, just check (if you want)

### Math

```rust
use jstdlib::math::{gcd, lcm, lerp};

let greatest = gcd(48, 18);
let least = lcm(12, 18);
let interpolated = lerp(0.0, 100.0, 0.25);

println!("GCD: {greatest}");
println!("LCM: {least}");
println!("Lerp: {interpolated}");
```

## License

See [LICENSE](LICENSE).
