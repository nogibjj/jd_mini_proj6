use sentiment::analyze;
extern crate clap;
use clap::Parser;


// add cli to input string
#[derive(Parser)]
#[clap(version = "1.0", author = "Jackie Du", about = None)]
struct Args {
    #[clap(long)]
    input: String,
}

fn main() {
    // Read an input in  
    let args = Args::parse();
    let input = args.input;

    // calculate the sentiment score 
    let res = analyze(input);


    // Return the sentiment score 
    println!("The sentiment score is: {:?}", res.score);

    // if there are positive words, print them 
    if !res.positive.words.is_empty() {
        println!("The positive words are: {:?}", res.positive.words);
    }
    else{
        println!("There are no positive words in the input");
    }
        
    
    if !res.negative.words.is_empty() {
        println!("The negative words are: {:?}", res.negative.words);
    }
    else {
        println!("There are no negative words in the input");
    }
}
