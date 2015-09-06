fn read_num() -> f32 {
   3.20434f32 
}
fn main() {
    let num = read_num();
    let formatted_num = format!("{n:+07.*}",2,n=num);
    println!("{:?}",formatted_num);
}
