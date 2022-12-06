use std::{error::Error, str::{self}, thread, time::Duration};

use itertools::Itertools;

/**
 * Part 1
 */

 
pub fn part1(input: &str) -> Result<i64, Box<dyn Error>> {
    let largeur: usize = read_largeur(input);
    let mut crates: Vec<Vec<u8>> = vec![Vec::new();largeur];
    read_crates_state(input, &mut crates);
    crates_mover(input, &mut crates, false)?;
    display_last(&crates);
    Ok(1)
}

/**
 * Part 2
 */
pub fn part2(input: &str) -> Result<i64, Box<dyn Error>> {
    let largeur: usize = read_largeur(input);
    let mut crates: Vec<Vec<u8>> = vec![Vec::new();largeur];
    read_crates_state(input, &mut crates);
    crates_mover(input, &mut crates, true)?;
    display_last(&crates);
    Ok(1)
}

fn read_largeur(input: &str) -> usize {
    input.lines().skip_while(|l|!l.starts_with(" 1"))
    .map(|l| l.split_whitespace().last().unwrap())
    .filter_map(|g| g.parse::<usize>().ok())
    .next().unwrap()
}

fn display_last(crates: &Vec<Vec<u8>>) {
    let mut resultat = String::new();
    for column in crates {
        resultat.push(*column.last().unwrap() as char);
    }
    println!("📦 {}", resultat);
}

fn read_crates_state(input: &str, crates: &mut [Vec<u8>]) {
    input.lines()
    .take_while(|l| !l.starts_with(" 1"))
    .for_each(|l|{
        let tab = l.as_bytes();
        for col in 0..crates.len() {
            if tab[col*4+1] != b' '{
                crates[col].push(tab[col*4+1]);
            }
        }
    });
    for column in crates { column.reverse(); }
}

fn crates_mover(input: &str, crates: &mut [Vec<u8>], is9001: bool) -> Result<(), Box<dyn Error>> {
    let mut grue = Vec::new();
    for ligne in input.lines().skip_while(|l| !l.starts_with("move")) {
        let split = ligne.split_ascii_whitespace().collect_vec();
        let howmany = split[1].parse::<usize>()?;//move 16 from 6 to 4
        let from = split[3].parse::<usize>()?-1;
        let to = split[5].parse::<usize>()?-1;
        grue.clear();
        for _ in 0..howmany {
            if let Some(dessus) = crates[from].pop() {
                grue.push(dessus);
            }
        }
        if is9001 { grue.reverse(); }
        crates[to].append(&mut grue);
        //display_and_reset_screen(crates, ligne);
    };
    Ok(())
}

/**
 * Fonction uniquement pour s'amuser et regarder s'animer le tableau
 */
#[allow(dead_code,clippy::needless_range_loop)]
fn display_and_reset_screen(crates: &mut [Vec<u8>], mvt: &str) {
    println!("\x1b  \x1b[A");//clear screeen
    let hauteurmax = crates.iter().map(|c|c.len()).max().unwrap();
    let largeur = crates.len();
    let mut affichage = vec![String::new();hauteurmax];
    // println!("high   : {}", hauteurmax);
    // println!("length : {}", largeur);
    for i in 0..hauteurmax {
        for j in 0..largeur {
            if let Some(case) = crates[j].get(i) {
            affichage[hauteurmax-1-i].push_str(format!("[{}] ",*case as char).as_str());
            } else {
                affichage[hauteurmax-1-i].push_str("[ ] ");
            }
            
        }
    }
    
    for l in affichage{
        println!("{}",l);
    }
    println!("==========");
    println!("{}",mvt);
    thread::sleep(Duration::from_millis(20));
    
}

#[cfg(test)]
mod tests {
    use super::*;

    const ÉNONCÉ: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn test_part1() {
        assert_eq!(part1(ÉNONCÉ).unwrap(), 1);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(ÉNONCÉ).unwrap(), 1);
    }

    #[test]
    fn real_tests() {
        // avec les vraies données
        assert_eq!(part1(include_str!("../inputs/input05.txt")).unwrap(), 1);
        assert_eq!(part2(include_str!("../inputs/input05.txt")).unwrap(), 1);
    }
}
