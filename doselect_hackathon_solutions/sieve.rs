use std::io;

fn return_primes(max:usize) -> Vec<bool> {
    let mut primes = Vec::with_capacity(max);
    let mut items_pushed = 0;
    loop {
        primes.push(true);
        items_pushed += 1;
        if items_pushed == max {
            break;
        }
    }

    primes[0] = false;
    if max > 1 {
        primes[1] = false;
    }

    for i in 0..max {
        if primes[i] {
            let mut mult = i << 1;
            while mult < max {
                primes[mult] = false;
                mult += i;
            }
        }
    }
    primes

}

fn isPrime(x: i32) -> bool {
    for i in 2..(x-1) {
        if x % i == 0 {return false;} else {}
    }
    return true
}
fn main() {
    // Declare the variables
    let mut line1 = String::new();
    let mut line2 = String::new();
    // Read the variables
    io::stdin().read_line(&mut line1).ok().expect("read error");
    io::stdin().read_line(&mut line2).ok().expect("read error");
    // parse integers
    let mut a : i32 = line1.trim().parse().ok().expect("parse error");
    let mut b : i32 = line2.trim().parse().ok().expect("parse error");
    let prime_cache = return_primes(b as usize);
    for i in (a as usize)..prime_cache.len() {
        if  prime_cache[i] {
            println!("{:?}",i);    
        }
    }
}
