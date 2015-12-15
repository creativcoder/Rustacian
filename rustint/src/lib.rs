#![feature(str_char)]
use std::ops::Add;
use std::ops::Sub;
use std::fmt;

#[derive(Debug)]
pub struct BigInt {
    val:Option<Vec<u64>>,
    sign:char,
}

impl BigInt {

    pub fn new(precision:usize) -> Self {
        BigInt { val:Some(Vec::with_capacity(precision)),sign:'+' }
    }

    pub fn from_str(num_str:&str) -> Self {
        let mut val:Vec<u64> = Vec::new();
        /*let mut sign:char = '+';*/
        let mut digit:u64;
        for i in num_str.chars().rev() {
            digit = i.to_string().parse::<u64>().unwrap();
            val.push(digit);
        }
        while(val.last()==Some(&0)){
            val.pop();
        }
        println!("{:?}",val );
        BigInt { val: Some(val),sign:'+' }
    }
}

impl Add for BigInt {
    type Output = BigInt;
    fn add(self,_rhs:BigInt) -> BigInt {
        let mut carry = 0;
        let lhs = self.val.clone().unwrap();
        let rhs = _rhs.val.clone().unwrap();
        let smaller = if lhs.len()<rhs.len() {lhs.len()} else {rhs.len()};
        let mut result = {if lhs.len() >= rhs.len() {lhs.clone()}  else {rhs.clone()}};
        for i in 0..smaller {
            result[i] = (lhs[i] + rhs[i])%10 + carry;
            carry = {if lhs[i] + rhs[i] > 9 {1} else {0}};
        }
        if carry == 1 {
            result[smaller] +=1;
        }
    BigInt {val:Some(result),sign:'+' }
    }
}

impl Sub for BigInt {
    type Output = BigInt;
    fn sub(self,_rhs:BigInt) -> BigInt {
        let mut borrow = 0;
        let mut lhs = self.val.clone().unwrap();
        let mut rhs = _rhs.val.clone().unwrap();
        let lhs_len = lhs.len();
        let mut sign = '+';
        let rhs_len = rhs.len();
        if lhs_len < rhs_len {
            let swp = lhs;
            lhs = rhs;
            rhs = swp;
            sign = '-';
        }
        let smaller = if lhs.len()<rhs.len() {lhs.len()} else {rhs.len()};
        let mut result = {if lhs.len() > rhs.len() {lhs.clone()}  else {rhs.clone()}};
        for i in (0..smaller) {
            if(rhs[i] > lhs[i]) {
                result[i] = (lhs[i]+(lhs[i+1]*10)) - rhs[i];
                borrow = 1;
                lhs[i+1] -=borrow;
            } else {
                result[i] = lhs[i] - rhs[i];    
            }
        }
        if(borrow==1){
            result[smaller] -= 1;
        }
    BigInt {val:Some(result),sign:sign }
    }
}

impl fmt::Display for BigInt {
    fn fmt(&self,f:&mut fmt::Formatter) -> fmt::Result {
        let num = self.val.clone().unwrap();
        let mut sum_result = String::new();
        for i in num.iter().rev() {
            sum_result.push_str(&i.to_string()[..]);
        }
    write!(f, "{}{}", 
        {if self.sign == '-' {"-"} else {""} },
        sum_result
        ) 
    }
    
}



