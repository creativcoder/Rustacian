// Transition table for a language consisting of strings having even number of zeroes ( that includes having no zeroes as well )

//____________
//S1 | 1 -> S1
//S1 | 0 -> S2
//S2 | 0 -> S1
//S2 | 1 -> S2
//------------


/*#[derive(Debug)]
enum STATE {
    S0,
    S1,
    S2,
}*/

pub mod even_zeros_dfa {

const S0:&'static str = "S0";
const S1:&'static str = "S1";
const S2:&'static str = "S2";


fn assert_state(state:&'static str,desired:&'static str) -> bool {
    state == desired
}

#[allow(unused_assignments)]
fn dfa(s:&String) -> bool {
    let mut state = S0;
    // initializing the dfa to the start state with any symbol
    if s.len() > 0 {state=S1;} else {return false;}
    for i in s.chars() {
        if i=='1' && assert_state(&state,S1)
        {state = S1;}
        else if i=='0' && assert_state(&state,S1)
        {state = S2;}
        else if i=='0' && assert_state(&state,S2)
        {state = S1;}
        else if i=='1' && assert_state(&state,S2)
        {state = S2;}
    }

    // automatic boolean return from match arm
    assert_state(&state,S1)
}

pub fn init_fsm(st:&String) {
     if dfa(st) {println!("Language accepted by the dfa");}
     else {println!("Language not recognized");}
}

#[test]
fn test_dfa() {
    let string = "100".to_string();
    if dfa(&string) {
        println!("Language accepted by the dfa");
    }else {
        println!("Language not recognized");
    }
}

}
