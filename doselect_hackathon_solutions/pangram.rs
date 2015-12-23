use std::io;
fn isPangram(s: String) -> bool {
    let mut small_alpha = 0;
    let mut stripped = String::new();
    for i in s.chars() {
        if i.is_alphabetic() {
            if stripped.contains(i.to_lowercase().next().unwrap()) {
            } else {
                stripped.push(i.to_lowercase().next().unwrap());
            }   
        }
    }
    for i in stripped.chars() {
        match i {
            'a' ... 'z' => small_alpha+=1, 
            _ => {},
        }
    }
    println!("{:?}",small_alpha );
    if small_alpha == 26 {true} else {false}
}
fn main() {
    let mut phrase = String::new();
    phrase.push_str("The quick br87686576own fox #$%%$^#jumps over the lazy dog");
    if isPangram(phrase) {
        println!("Yes\n");
    }
    else {
        println!("No\n");
    } 
}
