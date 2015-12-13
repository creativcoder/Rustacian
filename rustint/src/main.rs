extern crate rustint;

use rustint::BigInt;

fn main() {
   
   let big_lhs = BigInt::from_str("324353453243234235345345634543423243432433453453435");
   let big_rhs = BigInt::from_str("9934554345432423442423434534534534325");
   let small_lhs = BigInt::from_str("111");
   let small_rhs = BigInt::from_str("22992");
   let sum_big = big_lhs + big_rhs;
   let sum_small = small_lhs + small_rhs;
   sum_big.display();
   sum_small.display();
}