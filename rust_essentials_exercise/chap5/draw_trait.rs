struct S1 {
    id:u32
}

struct S2 {
    id:f32
}

trait Draw {
	fn draw(&self);
}


impl Draw for S1 {
	fn draw(&self) {
		println!("***{:?}***",self.id);
	}
}

impl Draw for S2 {
	fn draw(&self) {
		println!("***{:?}***",self.id);
	}
}

// imposing a trait constraint on this function bounded to only Draw implementing types
fn draw_object <T:Draw> (obj:T) {
	obj.draw();
}


fn main() {
	let s = S1 {id:34};
	s.draw();
	let any = S2 {id:56.34};
	// generic draw_object function that works for only types that implement the Draw trait.
	draw_object(any);
}