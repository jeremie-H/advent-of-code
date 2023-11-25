use adventofcode::common::*;
use colored::*;
use std::error::Error;

#[allow(unused_must_use)]
fn main() {
    println!("{}", "🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄".to_string().on_black().bold());
    println!("{}", format!("🎄 Advent of Code {} 🎅 🎄", 2023).on_black().bold());
    println!("{}", "🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄".to_string().on_black().bold());
    println!();

    alldays().iter().enumerate().for_each(|(i, j)| {
        display_msg_head(i + 1);
        let resultat = ((j.0 as fn(input: &str) -> Result<i64, Box<dyn Error>>)(j.1)).unwrap_or_default();
        display_msg_result(resultat);
    });
}
