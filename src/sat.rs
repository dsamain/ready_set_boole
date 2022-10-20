
use ready_set_boole::eval_one;
use std::collections::BTreeMap;

pub fn sat(formula: &str) -> bool {
    let mut mp: BTreeMap<char, bool> = BTreeMap::new();

    for c in formula.chars() {
        if c >= 'A' && c <= 'Z' {
            mp.insert(c, true);
        }
    }
    for mask in 0..(1 << mp.len()) {
        if eval_one(formula, &mut mp, mask) {
            return true;
        }
    }   
    false
}