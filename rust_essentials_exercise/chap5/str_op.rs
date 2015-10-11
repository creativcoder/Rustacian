struct Str {
	data:Vec<char>,
	len:u32,
}

impl Str {
	fn new() -> Self {
		Str { data:Vec::new(),len:0 }
	}
	fn push(&mut self,letter:char) {
		self.data.push(letter);
	}
	fn len(&self) -> u32 {
		{self.len}
	}
	
	fn display(self) {
		for i in self.data {
			print!("{}",i);
		}
	}
 }

fn main() {
	let mut s1 = Str::new();
	s1.push('r');
	s1.push('u');
	s1.push('s');
	s1.push('t');
	s1.display();

}