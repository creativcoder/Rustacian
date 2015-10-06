use std::f64;
use std::ops::Add;


struct Complex {
	real:i32,
	imag:i32,
}

// operator overloading for Complex type
impl Add for Complex {
	type Output = Complex;
	fn add(self,other:Complex) -> Complex {
		Complex { real:self.real + other.real,imag:self.imag+other.imag }
	}
}

impl Complex {

	fn new(x:i32,y:i32) -> Self {
		Complex {real:x,imag:y}
	}

	fn to_string(&self) -> String {
		if self.imag < 0 {
			format!("{}{}i",self.real,self.imag)
		} else {
			format!("{}+{}i",self.real,self.imag)
		}
	}


	fn times_ten(&mut self) {
		self.real *=10;
		self.imag *=10;
	}

	fn abs(&self) -> f64 {
		f64::floor(f64::sqrt((self.real*self.real) as f64 + (self.imag*self.imag) as f64))
	}

}

fn main() {
	let comp1 = Complex::new(5,3);
	let comp2 = Complex::new(8,-8);
	let mut summation = comp1 + comp2;
	println!("{}",summation.to_string());
	summation.times_ten();
	//println!("10x -> {:?}",summation.to_string());
	println!("Abs({}) => {}",summation.to_string(),summation.abs());
}