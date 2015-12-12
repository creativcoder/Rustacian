// Implements http://rosettacode.org/wiki/Count_occurrences_of_a_substring
// Author : Rahul Sharma
// Github : github.com/creativcoder

fn count_sub_string(src:&str,target:&str) -> usize {
    let mut buff = src.to_string();
    if buff.contains(target) {
      buff = buff.replace(target,"");  
    }

    (src.len() - buff.len())/target.len()
}

fn main() {
    let text = "this is three of the four";
    let sub_str = "th";
    println!("{:?}",count_sub_string(text,sub_str));
}
