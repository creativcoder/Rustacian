use std::io;

fn is_even(len:&usize) -> bool {
    if len % 2 == 0 {
        true
    } else {false}
}

fn min_edits(word1: &str, word2: &str) -> usize {
    let word1_length = word1.len() + 1;
    let word2_length = word2.len() + 1;
 
    let mut matrix = vec![vec![0]];
 
    for i in 1..word1_length { matrix[0].push(i); }
    for j in 1..word2_length { matrix.push(vec![j]); }
 
    for j in 1..word2_length {
        for i in 1..word1_length {
                let x: usize = if word1.chars().nth(i - 1) == word2.chars().nth(j - 1) {
                matrix[j-1][i-1]
            }
            else {
                let min_distance = [matrix[j][i-1], matrix[j-1][i], matrix[j-1][i-1]];
                *min_distance.iter().min().unwrap() + 1
            };
 
            matrix[j].push(x);
        }
    }
 
    matrix[word2_length-1][word1_length-1]
}



fn min_gears(s: String) -> usize {

    fn min(a:usize,b:usize) -> usize {
        if a > b {b} else {a}
    }
    let mut alternate = String::new();
    let mut alternate2 = String::new();
    
    if is_even(&s.len()) {
        
        let len = s.len();
        let last_letter = s.chars().last();
        match last_letter {
            Some(val) => {
                if val == 'C' {
                    for _ in 0..(len/2) {
                        alternate.push_str("AC");
                        alternate2.push_str("CA");
                    }
                } else if val == 'A' {
                    for _ in 0..(len/2) {
                        alternate.push_str("CA");
                        alternate2.push_str("AC");
                    }
                } 
            },
            None => {},
        }
        return min(min_edits(&s,&alternate),min_edits(&s,&alternate2));
    } else {

        let len = s.len();
        let last_letter = s.chars().last();
        match last_letter {
              Some(val) => {
                if val == 'C' {
                    for _ in 0..(len/2) {
                        alternate.push_str("CA");
                    }
                    alternate.push('C');
                } else if val == 'A' {
                    for _ in 0..(len/2) {
                        alternate.push_str("AC");
                    }
                    alternate.push('A');
                }
              },
              None => {},
          }
    }
    min_edits(&s,&alternate)
}
fn main() {
    let mut gears = String::new();
    io::stdin().read_line(&mut gears).ok().expect("read error");
    println!("{}",min_gears(gears.trim().to_string()));
}
