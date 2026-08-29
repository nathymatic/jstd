/// * Returns the greatest common divisor of two integers.
pub fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let rem = a % b;
        a = b;
        b = rem;
    }

    a.abs()
}

/// * Returns the least common multiple of two integers.
pub fn lcm(a: i64, b: i64) -> i64 {
    if a == 0 || b == 0 {
        return 0;
    }

    (a / gcd(a, b) * b).abs()
}

/// * Performs linear interpolation between `a` and `b`.
pub fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a + (b - a) * t
}
