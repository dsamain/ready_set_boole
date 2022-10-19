use std::thread::panicking;

use ready_set_boole::*;


//ab|! -> a!b!&
//ab&! -> a!b!|

fn negate(node: &mut BTNode) {
    if node.value >= 'A' && node.value <= 'Z' {
        node.left = Some(Box::new(BTNode { left: None, right: None, value: node.value }));
        node.value = '!';
    } else {
        match node.value {
            '!' => {
                node.value = node.left.as_ref().unwrap().value;
                node.left = None;
            }

            '&' => {
                node.value = '|';
                negate(node.left.as_mut().unwrap());
                negate(node.right.as_mut().unwrap());
            },
            '|' => {
                node.value = '&';
                negate(node.left.as_mut().unwrap());
                negate(node.right.as_mut().unwrap());
            },
            '^' => {
                node.value = '|';
                let l = &node.left.clone();
                let r = &node.right.clone();
                node.left = Some(Box::new(BTNode { left: l.clone(), right: r.clone(), value: '&' }));
                node.right = Some(Box::new(BTNode { left: l.clone(), right: r.clone(), value: '&' }));
                let l = node.left.as_mut().unwrap();
                negate(l.left.as_mut().unwrap());
                negate(l.right.as_mut().unwrap());
                //negate(l.left.as_mut().unwrap());
                //negate(l.right.as_mut().unwrap());
            },
            _ => {}
        }
    } 
}


pub fn negation_normal_form(formula: &str) -> String {
    let mut v: Vec<BTNode> = vec![];

    for c in formula.chars() {
        if c >= 'A' && c <= 'Z' {
            v.push(BTNode {left: None, right: None, value: c});
        } else {
            match c {
                '!' => {
                    negate(v.last_mut().unwrap());
                },
                '>' => {
                    let b = v.pop().unwrap();
                    let a = v.pop().unwrap();
                    let mut node = BTNode { left: Some(Box::new(a)), right: Some(Box::new(b)), value: '|' };
                    negate(node.left.as_mut().unwrap());
                    v.push(node);
                },
                '=' => {
                    let b = v.pop().unwrap();
                    let a = v.pop().unwrap();
                    let mut node = BTNode { left: Some(Box::new(a)), right: Some(Box::new(b)), value: '^' };
                    negate(&mut node);
                    v.push(node);
                }
                '^' => {
                    let b = v.pop().unwrap();
                    let a = v.pop().unwrap();
                    v.push(BTNode {left: Some(Box::new(a)), right: Some(Box::new(b)), value: c});
                    negate(v.last_mut().unwrap());
                    negate(v.last_mut().unwrap());
                }
                _ => {
                    let b = v.pop().unwrap();
                    let a = v.pop().unwrap();
                    v.push(BTNode {left: Some(Box::new(a)), right: Some(Box::new(b)), value: c});
                }
            }
        }
        //dbg!(node_to_string(v.last().unwrap()));
    }

    if v.len() != 1 {

        panic!("Invalid formula in negation_normal_form -> v.len() = {}", v.len());
    }

    return node_to_string(v.last().unwrap());
}