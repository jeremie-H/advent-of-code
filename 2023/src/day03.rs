use std::{error::Error, collections::HashSet};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Nombre {
    value: i64,
    line: usize,
    start: usize,
    end: usize,
    partnumber: bool,
    stillparsing: bool,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Gear {
    line: usize,
    position: usize,
    attached_numbers: HashSet<Nombre>,
}

fn load_input(input: &str) -> (Vec<Nombre>, Vec<Vec<char>>, Vec<Gear>) {
    let mut last_char = '.';
    input.lines()
    .enumerate()
    .fold((Vec::<Nombre>::new(), Vec::<Vec<char>>::new(), Vec::<Gear>::new()),
    |(mut nombres, mut tableau, mut gears), (numero_ligne, ligne)| {
        if let Some(last) = nombres.last_mut() {
            last.stillparsing = false;
            find_and_attach_to_gears(last, &mut gears);
        }
        tableau.push(ligne
                .chars()
                .map(|c| c as char)
                .enumerate()
                .fold(Vec::<char>::new(), |mut lignetableau, (i,c)| {
                    let stillparsing = nombres.last().is_some() && nombres.last().unwrap().stillparsing;
                    match c {
                        '0'..='9' => {
                            if !stillparsing { // nouveau nombre
                                nombres.push(Nombre{value: (c as u8 - b'0') as i64, line: numero_ligne, start: i, end: i, partnumber: last_char != '.', stillparsing: true});
                            } else {
                                let lastn = nombres.last_mut().unwrap();
                                lastn.value = 10 * lastn.value + (c as u8 - b'0') as i64;
                                lastn.end = i;
                            }
                        },
                        '.' => {
                            if stillparsing {
                                nombres.last_mut().unwrap().stillparsing = false;
                                find_and_attach_to_gears(nombres.last().unwrap(), &mut gears);
                            }
                            last_char = '.';
                        },
                        car => {
                            if stillparsing {
                                nombres.last_mut().unwrap().stillparsing = false;
                                nombres.last_mut().unwrap().partnumber = true;
                            }
                            last_char = car;

                            if car == '*' {
                                let mut gear = Gear{line: numero_ligne, position: i, attached_numbers: HashSet::new()};
                                find_and_attach_numbers(&mut gear, &nombres);
                                gears.push(gear);
                            }
                        },
                    };
                    lignetableau.push(c);
                    lignetableau
                })
        );
        (nombres,tableau, gears)
    })
}

fn find_and_attach_numbers(gear: &mut Gear, nombres: &[Nombre]) {
    let c = gear.clone();
    nombres.iter().rev()
    .take_while(|n| (n.line as i64) >= (gear.line as i64)-1)
    .filter(|n| is_adjacent_number(&c, n))
    .for_each(|&n| { gear.attached_numbers.insert(n); });
}

fn find_and_attach_to_gears(nombre: &Nombre, gears: &mut [Gear]) {
    gears.iter_mut().rev()
    .take_while(|g| g.line as i64 >= (nombre.line as i64)-1)
    .filter(|g| is_adjacent_number(g, nombre))
    .for_each(|g| { g.attached_numbers.insert(*nombre); });
}

fn is_adjacent_number(gear: &Gear, n: &Nombre) -> bool {
    let line = n.line as i64;
    let gline = gear.line as i64;
    let start = n.start as i64;
    let end = n.end as i64;
    let position = gear.position as i64;
    line + 1 >= gline && gline >= line - 1 && end + 1 >= position && position >= start - 1
}

/**
 * Part 1
 */
pub fn part1(input: &str) -> Result<i64, Box<dyn Error>> {
    let (mut nombres, tableau, _) = load_input(input);
    //vérifie pour les non partnumber s'il y a des caractères spéciaux au dessus ou en dessous
    nombres.iter_mut()
    .filter(|n| !n.partnumber)
    //.inspect(|f| println!("{:?}", f))
    .for_each(|n| {
        let i = if n.start == 0 { n.start } else { n.start - 1 };
        let j = if n.end == tableau[0].len()-1 { n.end } else { n.end + 1 };
        if n.line > 0 {
            for parcours_ligne_audessus in i..=j {
                if tableau[n.line-1][parcours_ligne_audessus] != '.' {
                    n.partnumber = true;
                    break;
                }
            }
        }
        if n.partnumber { return; }
        if n.line < tableau.len()-1 {
            for parcours_ligne_audessous in i..=j {
                if tableau[n.line+1][parcours_ligne_audessous] != '.' {
                    n.partnumber = true;
                    break;
                }
            }
        }
    });
    
    Ok(nombres.iter()
    .filter(|n| n.partnumber)
    .map(|n| n.value)
    .sum())
}

/**
 * Part 2
 */
pub fn part2(input: &str) -> Result<i64, Box<dyn Error>> {
    let (_, _, gears) = load_input(input);
    Ok(gears.iter()
    .filter(|g| g.attached_numbers.len() == 2)
    .fold(0, |acc, g| { acc +  g.attached_numbers.iter().map(|n| n.value).product::<i64>() }))
}

#[cfg(test)]
mod tests {
    use super::*;

    const ÉNONCÉ: &str = "\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&ÉNONCÉ).unwrap(), 4361);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&ÉNONCÉ).unwrap(), 467835);
    }

    #[test]
    fn real_tests() {
        // avec les vraies données
        assert_eq!(part1(include_str!("../inputs/input03.txt")).unwrap(), 553825);
        assert_eq!(part2(include_str!("../inputs/input03.txt")).unwrap(), 93994191);
    }
}

