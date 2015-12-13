extern crate rustint;

use rustint::BigInt;

fn main() {
   
   let a = BigInt::from_str("232432534634635345345");
   let b = BigInt::from_str("235433454");
   let big_lhs = BigInt::from_str("324353453243234235345345634543423243432433453453435");
   let big_rhs = BigInt::from_str("9934554345432423442423434534534534325");
   let small_lhs = BigInt::from_str("111");
   let small_rhs = BigInt::from_str("22992");
   let sum = a + b;
   let sum_big = big_lhs + big_rhs;
   let sum_small = small_lhs + small_rhs;
   println!("{}",sum);
   println!("{}",sum_big);
   println!("{}",sum_small);
}