use std::io;
use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};

fn most_occurring_word(para:String) -> String {
    let mut m:HashMap<&str,u32>= HashMap::new();
    let mut word_pos:Vec<(&str,usize)> = vec![];
    let word_vec = para.split(" ").collect::<Vec<_>>();
    for i in 0..word_vec.len() {
        word_pos.push((word_vec[i],i));
    }

    for i in word_vec.iter() {
        match m.entry(i) {
            Occupied(mut view) => {*view.get_mut() += 1;},
            Vacant(view) => { view.insert(1); }
        }
    }

    let mut most:u32 = 0;
    let mut most_str = "";
    let mut dups:HashMap<u32,Vec<&str>> = HashMap::new();
    for i in word_vec {
        if m.get(i).unwrap() > &most {
            most =  *m.get(i).unwrap();
            match dups.entry(most) {
                Occupied(mut val) => {val.get_mut().push(i.clone())},
                Vacant(view) => {view.insert(vec![i]);},
            }
            most_str = i.clone();
        }
    }
    let mut most_w:u32 = 0;
    for (k,v) in dups.iter() {
        if k > &most_w {
            most_w = *k;
        }
    }
    let answer = format!("{:?}",dups.get(&most_w).unwrap());
    let answer = format!("{}",dups.get(&most_w).unwrap().get(0).unwrap());
return answer.to_string();
}

fn main() {
    let mut para = String::new();
    io::stdin().read_line(&mut para).ok().expect("read error");
    println!("{}",most_occurring_word(para));
}
