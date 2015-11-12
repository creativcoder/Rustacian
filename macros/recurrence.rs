use std::io;

#[macro_use]
mod scanline;

macro_rules! recurrence {
    ( a[n] = $( $inits:expr ),+ ... $recur:expr ) => {0
    /*
    TODO
    */
};
}

fn main() {
    let mut range = String::new();
    scanline!(range);
    let range_numeral = range.trim().parse::<u64>().unwrap();
    let fib = recurrence!(a[n] = 0, 1 ... a[n-1] + a[n-2]);
}
