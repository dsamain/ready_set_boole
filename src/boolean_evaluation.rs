pub fn eval_formula(formula: &str) -> bool {
    let mut v: Vec<bool> = vec![];
    for c in formula.chars() {
        match c {
            '0' => v.push(false),
            '1' => v.push(true),
            '&' => {
                let b = v.pop().unwrap();
                let a = v.pop().unwrap();
                v.push(a & b);
            }  
            '|' => {
                let b = v.pop().unwrap();
                let a = v.pop().unwrap();
                v.push(a | b);
            }  
            '^' => {
                let b = v.pop().unwrap();
                let a = v.pop().unwrap();
                v.push(a ^ b);
            }  
            '=' => {
                let b = v.pop().unwrap();
                let a = v.pop().unwrap();
                v.push(a == b);
            }, 
            '>' => {
                let b = v.pop().unwrap();
                let a = v.pop().unwrap();
                v.push(!a | b);
            },
            '!' => {
                let a = v.pop().unwrap();
                v.push(!a);
            },
            _ => {
                println!("Invalid character in eval_formula: {}", c);
                return false;
            }
        }
    }
    if v.len() != 1 {
        println!("Invalid formula in eval_formula: {}", formula);
        return false
    }
    v.pop().unwrap()
}
