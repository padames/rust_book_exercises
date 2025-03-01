// `find_min!` will calculate the minimum of any number of arguments.
macro_rules! find_min {
    // Base case:
    ($x:expr) => ($x);
    // `$x` followed by at least one `$y,`
    ($x:expr, $($y:expr),+) => (
        // Call `find_min!` on the tail `$y`
        std::cmp::min($x, find_min!($($y),+))
    )
}

fn main() {
    println!("The minimum of (1) is {}", find_min!(1));
    println!("The minimum of (1+2, 2) is {}", find_min!(1 + 2, 2));
    println!("The minimum of (5, 2*3, 4) is {}", find_min!(5, 2 * 3, 4));
}
