pub trait Minimum {
    fn cmp(self,b:Self) -> Self;
}

impl Minimum for i32 {
    fn cmp(self,b:Self) -> i32 {
        if self > b {b} else {self}
    }
}

fn vec_min<T:Minimum>(vec:Vec<T>) -> Option<T> {
    let mut min : Option<T> = None;
    for i in vec {
        min = Some(match min {
            None => i,
            Some(val) => val.cmp(i),
        });
    }
    min
}

fn main() {
    let vec:Vec<i32> = vec![2,32,5,45,23,432,4];
    println!("Minimum is {}",vec_min(vec).unwrap());
}