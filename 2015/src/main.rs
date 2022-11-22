use std::error::Error;
use colored::*;
use adventofcode::common::*;

#[allow(unused_must_use)]
fn main() {
    println!("{}", format!("Advent of Code  {} - Jérémie", 2015).on_cyan().bold());
    println!();

    alldays()
        .iter()
        .enumerate()
        .for_each(|(i,j)| {
            display_msg_head(i+1);
            let resultat=((j.0 as fn(input: &str) -> Result<i64, Box<dyn Error>>)(j.1)).unwrap_or_default();
            display_msg_result(resultat);
        });
}
