extern crate clap;
use clap::Parser;

// add cli to trigger process image 
#[derive(Parser)]
#[clap(version = "1.0", author = "Jackie Du", about = None)]
struct Args {
    #[clap(long)]
    num1: i32,

    #[clap(long)]
    num2: i32,

    #[clap(long)]
    operator: String,
}

fn calculator(num1: i32, num2: i32, operator: &str) -> i32 {
    match operator {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        "/" => num1 / num2,
        _ => panic!("Invalid operator"),
    }
}

fn main() {
    let args = Args::parse();
    let result = calculator(args.num1 as i32, args.num2 as i32, &args.operator);
    println!("{} {} {} = {}", args.num1, args.operator, args.num2, result);
}
