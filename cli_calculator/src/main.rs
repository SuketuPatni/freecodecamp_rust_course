use std::env::{args, Args};

fn main() {
    let mut args: Args = args();

    // the nth method return the nth index element of an iterator 
    // and then advances it 
    // similar to <list>.pop(n) in python
    // it calls .next() on the iterator at the nth index
    let operand1 = args.nth(1).unwrap();
    let operator = args.nth(0).unwrap();
    let operand2 = args.nth(0).unwrap();
    println!("{} {} {}", operand1, operator, operand2);
    // args are supplied by ```cargo run -- <argument>```

}
