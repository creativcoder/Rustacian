#![feature(str_char)]


use std::ops::Add;

#[derive(Debug)]
pub struct BigInt {
    val:Option<Vec<u64>>,
}

impl BigInt {
    fn new() -> Self {
        BigInt { val:Some(Vec::with_capacity(1000)) }
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
}

impl Add for BigInt {
    type Output = BigInt;
    fn add(self,_rhs:BigInt) -> BigInt {
        let mut carry = 0;
        let mut result = BigInt::new();
        let lhs = self.val.clone().unwrap();
        let rhs = _rhs.val.clone().unwrap();
        if rhs.len() <= lhs.len() {
            for i in 0..rhs.len() {
                if let Some(ref mut val) = result.val {
                    val[i] = ((lhs[i]+rhs[i])%10);
                    carry = (lhs[i]+rhs[i])/10;
                    val.push(carry);
                }
            }
        } else {
            for i in 0..lhs.len() {
                if let Some(ref mut val) = result.val {
                    val[i] = ((lhs[i]+rhs[i])%10);
                    carry = (lhs[i]+rhs[i])/10;
                    val.push(carry);
                }
            }
        }
    result    
    }
}