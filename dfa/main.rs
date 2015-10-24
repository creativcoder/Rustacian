mod automata;

use automata::even_zeros_dfa;

fn main() {
    let string = "101111111111000".to_string();
    even_zeros_dfa::init_fsm(&string);
}
