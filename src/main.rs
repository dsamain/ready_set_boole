mod adder;
mod multiplier;
mod gray_code;
mod boolean_evaluation;
mod truth_table;
mod negation_normal_form;
mod conjunctive_normal_form;
mod sat;
mod powerset;
mod eval_set;
mod curve;

use colored::Colorize;
use adder::*;
use multiplier::*;
use gray_code::*;
use boolean_evaluation::*;
use truth_table::*;
use negation_normal_form::*;
use conjunctive_normal_form::*;
use ready_set_boole::*;
use sat::*;
use powerset::*;
use eval_set::*;
use curve::*;

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

    for _ in 0..10 {
        let formula = generate_formula();
        let res = negation_normal_form(formula.as_str());
        println!("{} -> {} {}", formula.bright_black(), res.blue(), if compare_formula(formula.as_str(), res.as_str()) {format!("OK").green()} else {format!("KO").red()});
    }

    println!("\n__________Conjonctive_normal_form__________\n");

    //let formula = "AB!&CD!&|AF&AB|^|A=BCD&|!>AB!&CD!&|AF&AB|^|A=BCD&|!>|";
    //let res = "DC!|A!|B!|DC!|A!|B!||DC!|A!|B!|BA!|A!|B!||DC!|A!|B!|AA!|A!|B!||DC!|A!|B!|FA!|A!|B!||DC!|A!|B!|AB!|A!|B!||DC!|A!|B!|D!A|A!F!||A|B!||DC!|A!|B!|CB!|A!F!||A|B!||DC!|A!|B!|CA|A!F!||A|B!||DC!|A!|B!|D!A|AB||A|B!||DC!|A!|B!|DC!|A!|C!D!|||BA!|A!|B!|DC!|A!|B!||BA!|A!|B!|BA!|A!|B!||BA!|A!|B!|AA!|A!|B!||BA!|A!|B!|FA!|A!|B!||BA!|A!|B!|AB!|A!|B!||BA!|A!|B!|D!A|A!F!||A|B!||BA!|A!|B!|CB!|A!F!||A|B!||BA!|A!|B!|CA|A!F!||A|B!||BA!|A!|B!|D!A|AB||A|B!||BA!|A!|B!|DC!|A!|C!D!|||AA!|A!|B!|DC!|A!|B!||AA!|A!|B!|BA!|A!|B!||AA!|A!|B!|AA!|A!|B!||AA!|A!|B!|FA!|A!|B!||AA!|A!|B!|AB!|A!|B!||AA!|A!|B!|D!A|A!F!||A|B!||AA!|A!|B!|CB!|A!F!||A|B!||AA!|A!|B!|CA|A!F!||A|B!||AA!|A!|B!|D!A|AB||A|B!||AA!|A!|B!|DC!|A!|C!D!|||FA!|A!|B!|DC!|A!|B!||FA!|A!|B!|BA!|A!|B!||FA!|A!|B!|AA!|A!|B!||FA!|A!|B!|FA!|A!|B!||FA!|A!|B!|AB!|A!|B!||FA!|A!|B!|D!A|A!F!||A|B!||FA!|A!|B!|CB!|A!F!||A|B!||FA!|A!|B!|CA|A!F!||A|B!||FA!|A!|B!|D!A|AB||A|B!||FA!|A!|B!|DC!|A!|C!D!|||AB!|A!|B!|DC!|A!|B!||AB!|A!|B!|BA!|A!|B!||AB!|A!|B!|AA!|A!|B!||AB!|A!|B!|FA!|A!|B!||AB!|A!|B!|AB!|A!|B!||AB!|A!|B!|D!A|A!F!||A|B!||AB!|A!|B!|CB!|A!F!||A|B!||AB!|A!|B!|CA|A!F!||A|B!||AB!|A!|B!|D!A|AB||A|B!||AB!|A!|B!|DC!|A!|C!D!|||D!A|A!F!||A|B!|DC!|A!|B!||D!A|A!F!||A|B!|BA!|A!|B!||D!A|A!F!||A|B!|AA!|A!|B!||D!A|A!F!||A|B!|FA!|A!|B!||D!A|A!F!||A|B!|AB!|A!|B!||D!A|A!F!||A|B!|D!A|A!F!||A|B!||D!A|A!F!||A|B!|CB!|A!F!||A|B!||D!A|A!F!||A|B!|CA|A!F!||A|B!||D!A|A!F!||A|B!|D!A|AB||A|B!||D!A|A!F!||A|B!|DC!|A!|C!D!|||CB!|A!F!||A|B!|DC!|A!|B!||CB!|A!F!||A|B!|BA!|A!|B!||CB!|A!F!||A|B!|AA!|A!|B!||CB!|A!F!||A|B!|FA!|A!|B!||CB!|A!F!||A|B!|AB!|A!|B!||CB!|A!F!||A|B!|D!A|A!F!||A|B!||CB!|A!F!||A|B!|CB!|A!F!||A|B!||CB!|A!F!||A|B!|CA|A!F!||A|B!||CB!|A!F!||A|B!|D!A|AB||A|B!||CB!|A!F!||A|B!|DC!|A!|C!D!|||CA|A!F!||A|B!|DC!|A!|B!||CA|A!F!||A|B!|BA!|A!|B!||CA|A!F!||A|B!|AA!|A!|B!||CA|A!F!||A|B!|FA!|A!|B!||CA|A!F!||A|B!|AB!|A!|B!||CA|A!F!||A|B!|D!A|A!F!||A|B!||CA|A!F!||A|B!|CB!|A!F!||A|B!||CA|A!F!||A|B!|CA|A!F!||A|B!||CA|A!F!||A|B!|D!A|AB||A|B!||CA|A!F!||A|B!|DC!|A!|C!D!|||D!A|AB||A|B!|DC!|A!|B!||D!A|AB||A|B!|BA!|A!|B!||D!A|AB||A|B!|AA!|A!|B!||D!A|AB||A|B!|FA!|A!|B!||D!A|AB||A|B!|AB!|A!|B!||D!A|AB||A|B!|D!A|A!F!||A|B!||D!A|AB||A|B!|CB!|A!F!||A|B!||D!A|AB||A|B!|CA|A!F!||A|B!||D!A|AB||A|B!|D!A|AB||A|B!||D!A|AB||A|B!|DC!|A!|C!D!|||DC!|A!|C!D!||DC!|A!|B!||DC!|A!|C!D!||BA!|A!|B!||DC!|A!|C!D!||AA!|A!|B!||DC!|A!|C!D!||FA!|A!|B!||DC!|A!|C!D!||AB!|A!|B!||DC!|A!|C!D!||D!A|A!F!||A|B!||DC!|A!|C!D!||CB!|A!F!||A|B!||DC!|A!|C!D!||CA|A!F!||A|B!||DC!|A!|C!D!||D!A|AB||A|B!||DC!|A!|C!D!||DC!|A!|C!D!|||&&&&&&&&&&&&&&&&&&";
    //println!("{} -> {} {}", formula.bright_black(), res.blue(), if compare_formula(formula, res) {format!("OK").green()} else {format!("KO").red()});
    for _ in 0..10 {
        let formula = generate_formula();
        let res = conjunctive_normal_form(formula.as_str());
        println!("{} -> {} {}", formula.bright_black(), res.blue(), if compare_formula(formula.as_str(), res.as_str()) {format!("OK").green()} else {format!("KO").red()});
    }

    println!("\n__________SAT__________\n");
    println!("{}", format!("valid formula: ").bold().italic());
    for _ in 0..10 {
        let formula = generate_formula();
        println!("sat({}) -> {}", formula.green(), sat(formula.as_str()));
    }
    println!("\n{}", format!("unvalid formula: ").bold().italic());
    let formula = "A!A&".to_string();
    println!("sat({}) -> {}", formula.red(), sat(formula.as_str()));
    let formula = "AB&AB&!&".to_string();
    println!("sat({}) -> {}", formula.red(), sat(formula.as_str()));
    let formula = "AB>AB!&&".to_string();
    println!("sat({}) -> {}", formula.red(), sat(formula.as_str()));
    let formula = "AB=AB!&&".to_string();
    println!("sat({}) -> {}", formula.red(), sat(formula.as_str()));

    println!("\n__________Powerset__________\n");

    let set = vec![1, 2, 3];
    let ret: Vec<Vec<i32>> = powerset(&set);
    println!("powerset({:?}) -> {:?}", set, ret);

    let set: Vec<i32> = vec![];
    let ret: Vec<Vec<i32>> = powerset(&set);
    println!("powerset({:?}) -> {:?}", set, ret);

    println!("\n__________Set_evaluation__________\n");

    let sets = vec![vec![1, 2, 3], vec![3, 4, 5], vec![1, 3, 5, 42]];
    let formula = "AB|C&";
    let ret: Vec<i32> = eval_set(formula, &sets);
    println!("eval_set({:?}, {:?}) -> {:?}", formula, sets, ret);

    let sets = vec![vec![1, 2, 3], vec![3, 4, 5], vec![1, 3, 5, 42]];
    let formula = "AB^!C&";
    let ret: Vec<i32> = eval_set(formula, &sets);
    println!("eval_set({:?}, {:?}) -> {:?}", formula, sets, ret);

    let sets = vec![vec![1, 2, 3], vec![42]];
    let formula = "A!!";
    let ret: Vec<i32> = eval_set(formula, &sets);
    println!("eval_set({:?}, {:?}) -> {:?}", formula, sets, ret);

    println!("\n__________map__________\n");


    let p = (42, 96);
    let ret = format!("{:?}", map(p.0, p.1)).green();
    let ret2 = format!("{:?}", reverse_map(map(p.0, p.1))).blue();
    println!("{} {:?} -> {}", format!("map").bold().italic(), p, ret);
    println!("{} {} -> {}", format!("reverse map").bold().italic(), map(p.0, p.1), ret2);

    let p = (41, 96);
    let ret = format!("{:?}", map(p.0, p.1)).green();
    let ret2 = format!("{:?}", reverse_map(map(p.0, p.1))).blue();
    println!();
    println!("{} {:?} -> {}", format!("map").bold().italic(), p, ret);
    println!("{} {} -> {}", format!("reverse map").bold().italic(), map(p.0, p.1), ret2);

    for _ in 0..5 {
        let p = (rand::random::<u16>(), rand::random::<u16>());
        let ret = format!("{:?}", map(p.0, p.1)).green();
        let ret2 = format!("{:?}", reverse_map(map(p.0, p.1))).blue();
        println!();
        println!("{} {:?} -> {}", format!("map").bold().italic(), p, ret);
        println!("{} {} -> {}", format!("reverse map").bold().italic(), map(p.0, p.1), ret2);
    }
    
}
