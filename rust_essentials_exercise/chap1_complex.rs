use std::fmt;

struct Complex {
    x: isize,
    y: isize,
}

// implementing Display Trait for Complex Structure
impl fmt::Display for Complex {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result 
    {
        write!(f,"{}+({})i", self.x, self.y)
    }
}

impl Complex {

    fn add(a:&Complex,b:&Complex) -> Complex {
        Complex{x:a.x+b.x, y:a.y+b.y}  
    } 

    fn sub(a:&Complex,b:&Complex) -> Complex {
        Complex{x:a.x-b.x, y:a.y-b.y}  
    } 

    fn mult(a:&Complex,b:&Complex) -> Complex {
        Complex {x:(a.x*b.x-a.y*b.y),y:(a.x*b.y+a.y*b.x)}
    }

    /*fn conjugate(a:Complex) -> Complex {
        
    }*/
}

fn main() {
let c1 = Complex {x:23,y:6};
    let c2 = Complex {x:5,y:8};
    let c3 = Complex::sub(&c1,&c2);
    println!("{}",c3 );
    println!("{}",c4 );
}
