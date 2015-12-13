#![feature(str_char)]
use std::ops::Add;

#[derive(Debug)]
pub struct BigInt {
    val:Option<Vec<u64>>,
}

impl BigInt {
    pub fn new(precision:usize) -> Self {
        BigInt { val:Some(Vec::with_capacity(precision)) }
    }
    pub fn from_str(num_str:&str) -> Self {
        let mut val:Vec<u64> = Vec::with_capacity(1000); 
        let mut digit:u64;
        for i in num_str.chars().rev() {
            digit = i.to_string().parse::<u64>().unwrap();
            val.push(digit);
        }
        BigInt {val:Some(val) }
    }
    pub fn display(&self) {
        let mut num = self.val.clone().unwrap();
        num.reverse();
        for i in num {
            print!("{:?}",i );
        }
        println!("");
    }
}

impl Add for BigInt {
    type Output = BigInt;
    fn add(self,_rhs:BigInt) -> BigInt {
        let mut carry = 0;
        let lhs = self.val.clone().unwrap();
        let rhs = _rhs.val.clone().unwrap();
        let smaller = if lhs.len()<rhs.len() {lhs.len()} else {rhs.len()};
        let mut result = {if lhs.len() > rhs.len() {lhs.clone()}  else {rhs.clone()}};
        for i in 0..smaller {
            result[i] = (lhs[i] + rhs[i])%10 + carry;
            carry = {if lhs[i] + rhs[i] > 9 {1} else {0}};
        }
        if carry == 1 {
            result[smaller] +=1;
        }
    BigInt {val:Some(result) }
    }
}


