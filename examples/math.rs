use jstd::math::{gcd, lcm, lerp};
use jstd::write_line;

fn main() {
    let greatest = gcd(48, 18);
    let least = lcm(12, 18);
    let interpolated = lerp(0.0, 100.0, 0.25);

    write_line!("GCD: {}\nLCM: {}\nLerp: {}", greatest, least, interpolated);
}
