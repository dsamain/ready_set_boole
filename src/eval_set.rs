use std::collections::BTreeSet;
use crate::negation_normal_form::*;

pub fn eval_set(formula: &str, sets: &Vec<Vec<i32>>) -> Vec<i32> {
    let mut v: Vec<BTreeSet<i32>> = vec![];

    let formula = negation_normal_form(formula);

    let global_set: BTreeSet<i32> = sets.iter().fold(BTreeSet::new(), |acc, x| acc.union(&x.iter().cloned().collect()).cloned().collect());

    for c in formula.chars() {
        if c >= 'A' && c <= 'Z' {
            v.push(sets[(c as usize) - ('A' as usize)].iter().cloned().collect());
            continue ;
        }
        match c {
            '!' => { 
                let mut a = v.pop().unwrap();
                a =  global_set.difference(&a).cloned().collect();
                v.push(a);
            }
            '|' => {
                let a = v.pop().unwrap(); 
                let b = v.pop().unwrap(); 
                v.push(a.union(&b).cloned().collect());
            }
            '&' => {
                let a = v.pop().unwrap(); 
                let b = v.pop().unwrap(); 
                v.push(a.intersection(&b).cloned().collect());
            }
            '^' => {
                let a = v.pop().unwrap(); 
                let b = v.pop().unwrap(); 

                v.push(a.symmetric_difference(&b).cloned().collect());
            },
            _ => {
            }
        }
    }
    v.pop().unwrap().into_iter().collect()
}
