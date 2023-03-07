// trying out printing out wordle style output for project 

use owo_colors::{
    colors::{Black, Green, Yellow, xterm::Gray},
    OwoColorize,
};
use std::fmt;


fn format_guess( d: impl fmt::Display) -> String {
    let test = format!("{}", d.fg::<Black>().bg::<Green>());
    test
}

// write main function
fn main() {
    // print out two squares
    println!("🟦");
    let test = format_guess("string".to_ascii_uppercase());
    println!("{}", test);

}