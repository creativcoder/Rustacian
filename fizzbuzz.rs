use std::fmt::{Display, Formatter, Error};

enum FBVAL {
    FizzBuzz(u64),
    Fizz(u64),
    Buzz(u64),
    Value(u64),
}

fn buzzy(num:u64) -> FBVAL {
    match num {
        num if num % 15 == 0 => FBVAL::FizzBuzz(num),
        num if num % 3 == 0 => FBVAL::Buzz(num),
        num if num % 5 == 0 => FBVAL::Fizz(num),
        num => FBVAL::Value(num),
    }
}

impl Display for FBVAL {
    fn fmt(&self,f:&mut Formatter) -> Result<(),Error> {
        match self {
            &FBVAL::FizzBuzz(_) => write!(f,"FizzBuzz"),
            &FBVAL::Buzz(_) => write!(f,"Buzz"),
            &FBVAL::Fizz(_) => write!(f,"Fizz"),
            &FBVAL::Value(n) => write!(f,"{}",n),
        }
    }
}

fn main() {
    for n in (0..101).map(buzzy){
        println!("{}",n);
    }
}