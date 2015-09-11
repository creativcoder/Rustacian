use std::f32::consts;
static RAD : f32 = 3.14;

mod circle_util {
pub fn get_radius() -> Option<f32> {
Some(3.45)
}

}

fn main() {
let rad = ::circle_util::get_radius().unwrap_or(RAD);
println!("The Area of Circle is {:?} (m)^2",consts::PI*rad*rad);

println!("{:?}",u32::from_str_radix(&"34",10).unwrap());

}
