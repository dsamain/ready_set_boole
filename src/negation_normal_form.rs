use std::{rc::Rc, ops::Deref};

#[derive(Clone)]
struct BTNode {
    pub left: Option<Box<BTNode>>,
    pub right: Option<Box<BTNode>>,
    pub value: char
}

fn negate(node: &mut BTNode) {
    if (node.value >= 'A' && node.value <= 'Z') {
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
            },
            _ => {}
        }
    } 
}

fn nodeToString(node: &BTNode) -> String {
    let mut res = String::new();
    let l = match &node.left {
        Some(l) => nodeToString(l),
        None => String::from(""),
    };
    let r = match &node.right {
        Some(r) => nodeToString(r),
        None => String::from(""),
    };
    res.push_str(&l);
    res.push_str(&r);
    res.push(node.value);
    res
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
                _ => {
                    let b = v.pop().unwrap();
                    let a = v.pop().unwrap();
                    v.push(BTNode {left: Some(Box::new(a)), right: Some(Box::new(b)), value: c});
                }
            }
        }
        //dbg!(nodeToString(v.last().unwrap()));
    }

    return nodeToString(v.last().unwrap());
}