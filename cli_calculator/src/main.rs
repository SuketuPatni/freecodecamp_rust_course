// use ansi_term::{Color, Style};
use std::env::{args, Args};

fn main() {
    let mut args: Args = args();
    // println!("{:?}", args);

    // the nth method return the nth index element of an iterator
    // and then advances it
    // it calls .next() on the iterator at the nth index
    let operand1 = args.nth(1).unwrap().parse::<f32>().unwrap();

    let operator = args.next().unwrap().chars().next().unwrap(); // operator must be parsed as a char

    let operand2 = args.next().unwrap().trim().parse::<f32>().unwrap();

    let result = operate(operand1, operator, operand2);

    println!("{} {} {} = {}", operand1, operator, operand2, result);

    // args are supplied by ```cargo run -- <argument>```
}

fn operate(operand1: f32, operator: char, operand2: f32) -> f32 {
    match operator {
        // for adding multiple checks, do
        /*
        '<char1>' | '<char2>' => <value expression>,
        */
        '+' => operand1 + operand2,
        '-' => operand1 - operand2,
        'x' => operand1 * operand2,
        '/' => operand1 / operand2,
        _ => {
            panic!("Operator not found!");
        }
    }
    // why didn't I put '*'? Turns out, if you put '*' as any argument,
    // it pushes all filenames of files in the working directory into the args iterator (!)
}
