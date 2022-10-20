
use crate::negation_normal_form::*;
use ready_set_boole::*;

// ab&|c -> ac|bc|&
fn handle_or(node: &mut BTNode) -> bool {
    let l = node.left.as_mut().unwrap();
    let r = node.right.as_mut().unwrap();
    if l.value == '&' {
        let a = Some(Box::new(BTNode { left: l.left.clone(), right: node.right.clone(), value: '|' }));
        let b = Some(Box::new(BTNode { left: l.right.clone(), right: node.right.clone(), value: '|' }));
        node.value = '&';
        node.left = a;
        node.right = b;
        return true;
    }
    else if r.value == '&' {
        let a = Some(Box::new(BTNode { left: r.left.clone(), right: node.left.clone(), value: '|' }));
        let b = Some(Box::new(BTNode { left: r.right.clone(), right: node.left.clone(), value: '|' }));
        node.value = '&';
        node.left = a;
        node.right = b;
        return true;
    }
    return false;
}

pub fn conjunctive_normal_form(formula: &str) -> String {
    let mut formula = negation_normal_form(formula);

    loop {
        let mut v: Vec<BTNode> = vec![];
        let mut f: bool = false;

        for c in formula.chars() {
            if c >= 'A' && c <= 'Z' {
                v.push(BTNode {left: None, right: None, value: c});
            } else {
                match c {
                    '!' => {
                        let node = v.pop().unwrap();
                        v.push(BTNode {left: Some(Box::new(node)), right: None, value: '!'});
                    },
                    '|' => {
                        let b = v.pop().unwrap();
                        let a = v.pop().unwrap();
                        let mut node = BTNode { left: Some(Box::new(a)), right: Some(Box::new(b)), value: '|' };
                        f |= handle_or(&mut node);
                        v.push(node);
                    
                    },
                    _ => {
                        let b = v.pop().unwrap();
                        let a = v.pop().unwrap();
                        v.push(BTNode {left: Some(Box::new(a)), right: Some(Box::new(b)), value: c});
                    }
                }
            }
        }
        formula = node_to_string(v.last().unwrap());
        if !f {
        let mut cnt: u32 = 0;
        for c in formula.chars() {
            cnt += (c == '&') as u32;
        }
        let mut res = String::new();
        for c in formula.chars() {
            if c == '&' {
                continue;
            }
            res.push(c);
        }
        while cnt != 0 {
            res.push('&');
            cnt -= 1;
        }
        return res;
        }

    }


}