use std::io;
use std::io::prelude::*;
use std::str::FromStr;

#[derive(Copy,Clone)]
struct Bot {
    x:i32,
    y:i32,
}

impl Bot {
    fn new() -> Self {
        Bot {x:0,y:0}
    }
}

fn main() {
    let mut myBot = Bot::new();
    let mut moves:[u8;1] = [0;1];
    let mut moves_vec = Vec::<i32>::new();
    while moves[0] as char != '\n' {
        io::stdin().read(&mut moves).ok().expect("read error");
        let num = 1 + i32::from_str(&moves[0].to_string()[..]).unwrap() % 49;
        moves_vec.push(num);
    }

    let mut not_working:[u8;1] = [0;1];
    io::stdin().read(&mut not_working).ok().expect("read_error");
    let not_working = 1 + i32::from_str(&not_working[0].to_string()[..]).unwrap() % 49;
    moves_vec.pop();
    for i in moves_vec {
        match i {
            1 => { if i == not_working {} else {myBot.x +=1}},
            2 => { if i == not_working {} else {myBot.x +=-1}},    
            3 => { if i == not_working {} else {myBot.y +=1}},
            4 => { if i == not_working {} else {myBot.y +=-1}},
            _ => {},
        }

        }

    println!("{:?}\n{:?}\n",myBot.x,myBot.y);
}
