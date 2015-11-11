pub struct BigInt {
    pub data: Vec<u64>,
}

impl BigInt {
    pub fn new(x:u64) -> Self {
        if x == 0 {
            BigInt { data : vec![] }
        } else {
            BigInt { data : vec![x] }
        }
    }
    pub fn test_invariant(&self) -> bool {
        if self.data.len() == 0 {
            true
        } else {
            self.data[self.data.len()-1] != 0
        }
    }
    pub fn from_vec(mut v:Vec<u64>) -> u64 {
        let mut str_repr = String::new();
        while v.len() != 0 {
        str_repr.push_str(&v.pop().unwrap().to_string()[..]);
    }
    let num:u64 = str_repr.parse::<u64>().unwrap();
        num
    }
}

fn main() {
    let mut v:Vec<u64> = vec![7,3,3,1];
}