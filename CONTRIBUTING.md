# Contributing to jstdlib

thanksyou for wanting to contribute to **jstdlib**!

jstdlib is a small Rust utility crate for things that are useful enough that i don't want to rewrite them every time. it currently contains utilities for console I/O, collections, and math, with more potentially being added over time. . .

contributions are ABSOLUTELY welcome, especially when they add genuinely useful functionality or improve existing utilities without making the crate unnecessarily complicated!

## Getting Started

### Requirements

you'll need:

- [Rust](https://www.rust-lang.org/)
- Cargo

clone the repository and enter the project directory:

```powershell
git clone https://github.com/jayywashere/jstd
cd jstd
```

build the project with:

```powershell
cargo build
```

run the test suite with:

```powershell
cargo test
```

it's also a good idea to make sure the project is formatted correctly:

```powershell
cargo fmt --check
```

you can automatically format the code with:

```powershell
cargo fmt
```

## Project Structure

the main library code lives in `src/`.

utilities are organized by what they provide:

- `console` -- console input/output
- `collections` -- collection-related utilities
- `math` -- mathematical utilities

examples live in `examples/`.

when adding a new utility, put it in the module that best matches what it does. if it doesn't fit an existing module, discuss it in an issue before introducing a new module. discuss about moving things from module to module if necessary.

## What Makes a Good Contribution?

good contributions generally:

- solve a useful problem
- fit naturally into jstdlib
- keep the API reasonably simple
- follow existing Rust conventions
- avoid unnecessary dependencies
- include tests when appropriate
- don't introduce functionality that belongs in a completely different crate

jstdlib is intentionally a collection of practical utilities, so adding something just because it _can_ be added isn't necessarily a good reason to add it.

basically: **useful > huge**.

## Adding Utilities

when adding a new utility:

1. put it in the appropriate module.
2. follow the existing naming and formatting conventions.
3. keep the public API straightforward.
4. add tests for important behavior and edge cases.
5. update the README if the new functionality is user-facing.
6. add or update an example when it would make the feature easier to understand.

for collection utilities in particular, consider behavior for empty collections and unusual inputs.

for mathematical utilities, consider edge cases and make sure the behavior is well-defined for the supported types.

for other utilities, do what you must. always exhibit common sense at all times, including this.

## Dependencies

avoid adding a dependency unless it provides meaningful value.

if the functionality can reasonably be implemented using the Rust standard library without making the code significantly worse, prefer the standard library.

if a dependency is necessary, explain why it is needed in the pull request.

## API Changes

please be careful with changes to existing public APIs.

if you're planning to:

- remove a public function
- rename a public item
- change an existing function's behavior
- change a function's signature
- make a breaking change

open an issue first so the change can be discussed before you spend time implementing it.

small internal changes generally don't need prior discussion.

## Tests

run the test suite before submitting a pull request:

```powershell
cargo test
```

also make sure formatting passes:

```powershell
cargo fmt --check
```

new functionality should generally include appropriate tests.

bug fixes should ideally include a test that demonstrates the problem being fixed. (e.g., Before & After Proof)

## Documentation

if your contribution changes how users interact with jstdlib, update the relevant documentation.

for user-facing functionality, this may include:

- Rustdoc comments
- README examples
- the feature list
- examples in `examples/`

thanks-a-lot if you do include those!! it's REALLY convenient, especially for public, reusable APIs.

keep documentation concise and consistent with the rest of the project.

## Issues

before opening an issue, check whether a similar issue already exists.

### Bug Reports

when reporting a bug, include:

- what you expected to happen
- what actually happened
- a minimal example that reproduces the problem, when possible
- your Rust version
- any other information that would help reproduce the issue

a small reproducible example is much more useful than:

> it doesn't work

PLEASE, do not do that to me. 🤗

### Feature Requests

for feature requests, explain:

- what you want to add
- why it would be useful
- how you expect it to work, if you have an API idea

do keep in mind that not every proposed utility will necessarily fit the scope of jstdlib.

## Pull Requests

before opening a pull request:

1. make sure your branch is up to date.
2. make your changes.
3. run `cargo test`.
4. run `cargo fmt --check`.
5. run other beneficial commands.
6. update documentation where necessary.
7. review your own diff.
8. open the pull request with a clear description of what changed and why.

keep pull requests focused on one logical change where possible.

a pull request doesn't need to be enormous to be useful. small improvements are welcome!!

## Commit Messages

use a short, descriptive commit message that explains what changed.

for example:

```text
add collection helper for partitioning
```

or:

```text
fix console input parsing
```

please avoid commit messages that don't explain the change, such as:

```text
stuff
```

or:

```text
fix crap
```

even if the latter is technically accurate.

## Before You Submit

please make sure:

- [ ] the project builds with `cargo build` (just ensure the compiler is not arguing with you)
- [ ] tests pass with `cargo test`
- [ ] code is formatted with `cargo fmt`
- [ ] new functionality has appropriate tests
- [ ] user-facing changes have updated documentation
- [ ] the pull request has a clear description
- [ ] the change fits the purpose of jstdlib

## Final Note

jstdlib is a small project, and contributions don't need to be overly complicated.,.

if you do have an idea that you aren't sure fits the project, feel free to open an issue and discuss it first.

thanks-a-ton for contributing! 😁