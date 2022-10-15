
//| A | B | C | = |$
//|---|---|---|---|$
//| 0 | 0 | 0 | 0 |$
//| 0 | 0 | 1 | 1 |$
//| 0 | 1 | 0 | 0 |$
//| 0 | 1 | 1 | 1 |$
//| 1 | 0 | 0 | 0 |$
//| 1 | 0 | 1 | 1 |$
//| 1 | 1 | 0 | 1 |$
//| 1 | 1 | 1 | 1 |$

use crate::boolean_evaluation::*;
use std::collections::BTreeMap;

fn eval_one(formula: &str, mp: &mut BTreeMap<char, bool>, mask: u32) -> bool{
        let mut i: u8 = 0;
        let len: u8 = mp.len() as u8;
        let mut t = String::from(formula);
        for (key, val) in mp.iter_mut() {
            *val = (mask >> (len - i - 1)) & 1 == 1;
            t = t.replace(*key, if *val { "1" } else { "0" });
            i += 1;
        }
        eval_formula(t.as_str())
}

pub fn print_truth_table(formula: &str) {

    let mut mp: BTreeMap<char, bool> = BTreeMap::new();

    let mut t: String = String::from(formula);
    for c in 'A'..='Z' {
        t = t.replace(c, "1");
    }

    for c in formula.chars() {
        if c >= 'A' && c <= 'Z' {
            mp.insert(c, true);
        }
    }

    for (key, _) in mp.iter() {
        print!("| {} ", *key);
    }
    println!("| = |");
    for _ in 0..mp.len() {
        print!("|---");
    }
    println!("|---|");

    for mask in 0..(1 << mp.len()) {
        let res: u8 = eval_one(formula, &mut mp, mask) as u8;

        for (_, val) in mp.iter() {
            print!("| {} ", if *val { "1" } else { "0" });
        }
        println!("| {} |", res);
    }
}