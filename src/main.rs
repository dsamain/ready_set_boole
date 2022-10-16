mod adder;
mod multiplier;
mod gray_code;
mod boolean_evaluation;
mod truth_table;
mod negation_normal_form;

use adder::*;
use multiplier::*;
use gray_code::*;
use boolean_evaluation::*;
use truth_table::*;
use negation_normal_form::*;

const N: u32 = 7;

fn main() {

    println!("\n__________ADDER__________\n");
    for _ in 0..N {
        let x: u32 = rand::random::<u16>() as u32;
        let y: u32 = rand::random::<u16>() as u32;
        println!("{} + {} = {} : {}", x, y, adder(x, y), if x + y == adder(x, y) {"OK"} else {"KO"});
    }
    println!("\n__________MULTIPLIER__________\n");
    for _ in 0..N {
        let x: u32 = rand::random::<u16>() as u32;
        let y: u32 = rand::random::<u16>() as u32;
        println!("{} * {} = {} : {}", x, y, multiplier(x, y), if x * y == multiplier(x, y) {"OK"} else {"KO"});
    }
    println!("\n__________GRAY_CODE__________\n");
    for i in 0..=15 {
        let res = gray_code(i);
        println!("gray_code {}: {}", i, format!("{res:b}"));
    }
    println!("\n__________Boolean_evaluation__________");
    let formula = "10&";
    println!("\n{} : {}", formula, eval_formula(formula));
    let formula = "10|";
    println!("{} : {}", formula, eval_formula(formula));
    let formula = "11>";
    println!("{} : {}", formula, eval_formula(formula));
    let formula = "10=";
    println!("{} : {}", formula, eval_formula(formula));
    let formula = "1011||=";
    println!("{} : {}", formula, eval_formula(formula));

    println!("\n__________Truth_table__________\n");
    let formula = "AB&C|";
    println!("{formula}:");
    print_truth_table(formula);
    let formula = "AB&C&";
    println!("\n{formula}:");
    print_truth_table(formula);
    let formula = "AB^C^";
    println!("\n{formula}:");
    print_truth_table(formula);

    println!("\n__________Negation_normal_form__________\n");

    let formula = "AB|C&";
    let res = negation_normal_form(formula);
    println!("formula truth table: ({})", formula);
    print_truth_table(formula);
    println!();
    println!("negation form truth table: ({})", res);
    print_truth_table(res.as_str());
    println!();

    let formula = "AB&!";
    let res = negation_normal_form(formula);
    println!("formula truth table: ({})", formula);
    print_truth_table(formula);
    println!();
    println!("negation form truth table: ({})", res);
    print_truth_table(res.as_str());
    println!();

    let formula = "AB|C&!!";
    let res = negation_normal_form(formula);
    println!("formula truth table: ({})", formula);
    print_truth_table(formula);
    println!();
    println!("negation form truth table: ({})", res);
    print_truth_table(res.as_str());
    println!();
    
    let formula = "AB^!";
    let res = negation_normal_form(formula);
    println!("formula truth table: ({})", formula);
    print_truth_table(formula);
    println!();
    println!("negation form truth table: ({})", res);
    print_truth_table(res.as_str());
    println!();

    let formula = "AB>!";
    let res = negation_normal_form(formula);
    println!("formula truth table: ({})", formula);
    print_truth_table(formula);
    println!();
    println!("negation form truth table: ({})", res);
    print_truth_table(res.as_str());
    println!();

    let formula = "AB=!";
    let res = negation_normal_form(formula);
    println!("formula truth table: ({})", formula);
    print_truth_table(formula);
    println!();
    println!("negation form truth table: ({})", res);
    print_truth_table(res.as_str());
    println!();

    let formula = "AB=";
    let res = negation_normal_form(formula);
    println!("formula truth table: ({})", formula);
    print_truth_table(formula);
    println!();
    println!("negation form truth table: ({})", res);
    print_truth_table(res.as_str());
    println!();

    println!("enter an expression:");
    let mut formula = String::new();
    std::io::stdin().read_line(&mut formula);
    formula.pop();
    let formula = formula.as_str();
    let res = negation_normal_form(formula);
    println!("formula truth table: ({})", formula);
    print_truth_table(formula);
    println!();
    println!("negation form truth table: ({})", res);
    print_truth_table(res.as_str());
    println!();
}
