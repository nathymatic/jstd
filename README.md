# jstdlib

A collection of practical utilities for Rust.

> **jstdlib** is an early-stage utility crate focused on making common Rust tasks simpler and more convenient.

## Features

* 🖥️ **Console utilities** — convenient console input and output
* 📦 **Collection utilities** — practical helpers for slices and vectors
* 🔢 **Math utilities** — common mathematical operations

## Installation

Add `jstdlib` to your project with Cargo:

```bash
cargo add jstdlib
```

Or add it manually to your `Cargo.toml`:

```toml
[dependencies]
jstdlib = "0.1"
```

## Quick Start

### Console

Jstdlib provides convenient console input and output utilities:

```rust
use jstdlib::console::Console;

Console::write_line("Hello, world!");

let name: String = Console::input("What's your name? ");
let age: i32 = Console::input("How old are you? ");

Console::write_line(&format!("Hello, {name}!"));
Console::write_line(&format!("You are {age} years old."));
```

You can also use the formatting macros:

```rust
use jstdlib::write_line;

let name = "Someone";
let age = 21;

write_line!("Name: {name}");
write_line!("Age: {age}");
```

### Collections

Jstdlib provides functional-style utilities for working with collections:

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

Available utilities currently include:

* `filter`
* `map`
* `find`
* `any`
* `all`
* `count`
* `unique`
* `chunk`
* `partition`

### Math

Common mathematical utilities:

```rust
use jstdlib::math::{gcd, lcm, lerp};

let greatest = gcd(48, 18);
let least = lcm(12, 18);
let interpolated = lerp(0.0, 100.0, 0.25);

println!("GCD: {greatest}");
println!("LCM: {least}");
println!("Lerp: {interpolated}");
```

## Modules

| Module        | Description                        |
| ------------- | ---------------------------------- |
| `console`     | Console input and output utilities |
| `collections` | Collection and slice utilities     |
| `math`        | Common mathematical utilities      |

## Status

**Early development — v0.1.0**

Jstdlib is currently a small utility crate and its API may evolve as the project develops.

The goal is not to replace Rust's standard library, but to provide convenient abstractions for tasks where they make sense.

## License

Licensed under the MIT License.