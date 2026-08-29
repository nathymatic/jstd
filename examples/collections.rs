use jstd::collections::vec;
use jstd::write_line;

fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6];
    let even = vec::filter(&numbers, |x| x % 2 == 0);
    let doubled = vec::map(&numbers, |x| x * 2);
    let found = vec::find(&numbers, |x| *x > 4);
    let has_even = vec::any(&numbers, |x| x % 2 == 0);

    write_line!("Numbers: {:?}", numbers);
    write_line!("Even: {:?}", even);
    write_line!("Doubled: {:?}", doubled);
    write_line!("Found: {:?}", found);
    write_line!("Has even: {:?}", has_even);
}
