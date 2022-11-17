extern crate colored;

use colored::*;

pub fn display_msg_head(day: usize) {
    println!(
        "{}",
        format!("     Day {:0>2} - Part {}     ", (day + 1) / 2, (day + 1) % 2 + 1)
            .on_bright_purple()
            .white()
            .bold()
    );
}

pub fn display_msg_result(result: i64) {
    println!("Le résultat est : {}", format!("{}", result).green().bold());
}
