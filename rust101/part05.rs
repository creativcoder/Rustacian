#[derive(Debug)]
pub struct BigInt {
    pub data: Vec<u64>,
}

impl BigInt {
    fn new(x:&str) -> Self {
        if x.len() == 0 {
            BigInt { data: vec![] }
        } else {
            let slice = x.to_string().clone();
            let mut vec_dig: Vec<u64> = Vec::new();
            for i in slice.chars().rev() {
                let parsed_digit = i.to_string().parse::<u64>().unwrap();
                vec_dig.push(parsed_digit);
            }
            BigInt { data: vec_dig }
        }
    }
    fn test_invariant(&self) -> bool {
        if self.data.len() == 0 {true} else {
            self.data[self.data.len()-1] != 0 
        }
    }
    fn add(&self,other: &BigInt) -> Self {
        let mut result:Vec<u64> = Vec::new();
        for i in 0..(self.data.len()-1) {
            result[i] = self.data[i] + other.data[i];
        }
    }
}

fn main() {
    let n = "43242353";
    let n2 = "1111";
    let big_num = BigInt::new(n);
    let big_num2 = BigInt::new(n2);
    let result = big_num.add(&big_num2);
    println!("{:?}",big_num );
}