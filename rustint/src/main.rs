extern crate rustint;

use rustint::BigInt;
use std::ops::Add;

fn main() {
   let b = BigInt::from_str("9999");
   let b2 = BigInt::from_str("99");
   let t = b + b2;
   println!("{:?}",t);
}