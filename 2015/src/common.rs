extern crate colored;
use colored::*;
use crate::*; // day**

/**
 * affichage de l'en-tête d'un challenge
 */
pub fn display_msg_head(day: usize) {
    println!(
        "{}",
        format!("     Day {:0>2} - Part {}     ", (day + 1) / 2, (day + 1) % 2 + 1)
            .on_bright_purple()
            .white()
            .bold()
    );
}

/**
 * affichage du résultat d'un challenge
 */
pub fn display_msg_result(result: i64) {
    println!("Le résultat est : {}", format!("{}", result).green().bold());
}

type AocResult = &'static [(for<'a> fn(&'a str) -> Result<i64, Box<dyn std::error::Error>>, &'static str)];
type AocRunners<'a, T> = &'a [(fn(input: &str) -> T, &'a str)];
/**
 * definition de toutes les fonctions à exécuter
 */
pub fn alldays()-> AocResult {
    let runner: AocRunners<_> = &[
        (day01::part1, include_str!("../inputs/input01.txt")), (day01::part2, include_str!("../inputs/input01.txt")),
        (day02::part1, include_str!("../inputs/input02.txt")), (day02::part2, include_str!("../inputs/input02.txt")),
        (day03::part1, include_str!("../inputs/input03.txt")), (day03::part2, include_str!("../inputs/input03.txt")),
        (day04::part1, include_str!("../inputs/input04.txt")), (day04::part2, include_str!("../inputs/input04.txt")),
        (day05::part1, include_str!("../inputs/input05.txt")), (day05::part2, include_str!("../inputs/input05.txt")),
        (day06::part1, include_str!("../inputs/input06.txt")), (day06::part2, include_str!("../inputs/input06.txt")),
        (day07::part1, include_str!("../inputs/input07.txt")), (day07::part2, include_str!("../inputs/input07.txt")),
        (day08::part1, include_str!("../inputs/input08.txt")), (day08::part2, include_str!("../inputs/input08.txt")),
        (day09::part1, include_str!("../inputs/input09.txt")), (day09::part2, include_str!("../inputs/input09.txt")),
        (day10::part1, include_str!("../inputs/input10.txt")), (day10::part2, include_str!("../inputs/input10.txt")),
        (day11::part1, include_str!("../inputs/input11.txt")), (day11::part2, include_str!("../inputs/input11.txt")),
        (day12::part1, include_str!("../inputs/input12.txt")), (day12::part2, include_str!("../inputs/input12.txt")),
        (day13::part1, include_str!("../inputs/input13.txt")), (day13::part2, include_str!("../inputs/input13.txt")),
        (day14::part1, include_str!("../inputs/input14.txt")), (day14::part2, include_str!("../inputs/input14.txt")),
        (day15::part1, include_str!("../inputs/input15.txt")), (day15::part2, include_str!("../inputs/input15.txt")),
        (day16::part1, include_str!("../inputs/input16.txt")), (day16::part2, include_str!("../inputs/input16.txt")),
        (day17::part1, include_str!("../inputs/input17.txt")), (day17::part2, include_str!("../inputs/input17.txt")),
        (day18::part1, include_str!("../inputs/input18.txt")), (day18::part2, include_str!("../inputs/input18.txt")),
        (day19::part1, include_str!("../inputs/input19.txt")), (day19::part2, include_str!("../inputs/input19.txt")),
        (day20::part1, include_str!("../inputs/input20.txt")), (day20::part2, include_str!("../inputs/input20.txt")),
        (day21::part1, include_str!("../inputs/input21.txt")), (day21::part2, include_str!("../inputs/input21.txt")),
        (day22::part1, include_str!("../inputs/input22.txt")), (day22::part2, include_str!("../inputs/input22.txt")),
        (day23::part1, include_str!("../inputs/input23.txt")), (day23::part2, include_str!("../inputs/input23.txt")),
        (day24::part1, include_str!("../inputs/input24.txt")), (day24::part2, include_str!("../inputs/input24.txt")),
        (day25::part1, include_str!("../inputs/input25.txt")), (day25::part2, include_str!("../inputs/input25.txt")),

    ];
    runner
}