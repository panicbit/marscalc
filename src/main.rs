use std::env::args;
use marscalc::Number;

fn main() {
    let a = args().nth(1).expect("arg1: missing number");
    let a = a.parse::<Number>().expect("arg1: invalid number");

    let b = args().nth(3).expect("arg3: missing number");
    let b = b.parse::<Number>().expect("arg3: invalid number");
    
    let op = args().nth(2).expect("arg2: missing operation (+, -, *)");
    
    let result = match &*op {
        "+" => a + b,
        "-" => a - b,
        "*" => a * b,
        op => panic!("invalid operation: {}", op),
    };

    println!("{}", result);
}
