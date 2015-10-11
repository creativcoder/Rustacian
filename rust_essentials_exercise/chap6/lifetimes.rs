#[derive(Clone)]
#[derive(Debug)]
struct MagicNumber {
	num:f64,
}

#[derive(Debug)]
struct Alien {
	name:String,
	tentacles:u32,
}

// exercise chap6
impl Alien {
	fn grow_tentacles(&mut self) {
		self.tentacles+=2;
	}
}

#[derive(Debug)]
struct  Magician {
    name:&'static str,
    power:u32,
}

#[allow(unused_variables)]
fn main() {

	let n = 42;
	match n {
		ref n => println!("Got a reference {:p}",n),
	}

	let mut m = 89;
	match m {
		ref mut mr => {println!("\nGot a mutable reference {:?}",mr);*mr= 231;},
	}
	println!("\nm is now changed to {:?}",m);
	
	//getting reference via destructuring using ref keyword on LHS
	let mag = Magician {name:"Turok",power:298};
	let name_of_magician = {let Magician {name:ref ref_to_name,power:_}=mag;*ref_to_name};
	println!("{:?}",name_of_magician );

	// Ownership in action
	// al has the ownership of the Alien struct instance
	let al = Alien {name:"Predator".to_string(),tentacles:4};
	// ownership transferred to big_al
	let mut big_al = al;

	// #[1]error: al2 cannot get a immutable reference to al because the resource has been moved
	// where al currently was pointing to, the new owner is big_al
	//let al2 = &al;
	// same error as #[1]
	//al.name = "Omnivore".to_string();
	big_al.name = "Krieger".to_string();

	// exercise part of chapter 6
	println!("{:?}",big_al);
	big_al.grow_tentacles();
	println!("The tentacles upgraded to {:?}",big_al.tentacles);

}