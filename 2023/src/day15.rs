use std::{str::{self},error::Error, collections::VecDeque};

/**
 * Part 1
 */
pub fn part1(input: &str) -> Result<i64, Box<dyn Error>> {
    Ok(input.split(',').map(|line| hash_part1(line.as_bytes())).sum())
    
}

fn hash_part1(line: &[u8]) -> i64 {
    line.iter().fold(0, |acc, &b| {
        ((acc + b as i64)* 17) % 256
    })
}

/**
 * Part 2
 */
pub fn part2(input: &str) -> Result<i64, Box<dyn Error>> {
    let hasmap = input.split(',')
    .fold(vec![VecDeque::new();256], |mut acc, line| {
        let (hash, value, key) = hash_part2(line.as_bytes());
        let key = String::from_utf8(key).unwrap();
        replace_add_or_remove(&mut acc, hash, key, value);
        acc
    });
    
    let focusing_power = hasmap.iter().enumerate().fold(0, |acc, (i, list)| {
        let somme_dune_box = list.iter().rev().enumerate().fold(0, |acc2, (slot, (_, focal))| {
            acc2 + (i+1) * (slot+1) * (*focal as usize)
        });
        acc + somme_dune_box
    });
    Ok(focusing_power as i64)
}

/**
 * cette fonction ajoute ou supprime une valeur dans une liste de liste de clé/valeur
 * si la valeur est None, la clé est supprimée de la liste
 * si la clé existe déjà, la valeur est remplacée
 */
fn replace_add_or_remove(acc: &mut Vec<VecDeque<(String, u8)>>, hash: usize, key: String, focal: Option<u8>) {
    if let Some((index,key_focal)) = acc[hash].iter_mut().enumerate().find(|(_, (k, _))| k == &key) {
        if let Some(focal) = focal {
            key_focal.1 = focal;
        } else {
            acc[hash].remove(index);
        }
    }
    else if let Some(focal) = focal {
        acc[hash].push_front((key, focal));
    }
}

/**
 * exemple pour line = "rn=1", cette fonction retourne :
 * hash : le hash de la ligne : 0
 * value : la valeur de la ligne : Some(1) ou None si c'est une suppression (-)
 * clef : la clé de la ligne : rn
 */
fn hash_part2(line: &[u8]) -> (usize, Option<u8>, Vec<u8>) {
    line.iter().fold((0, None, Vec::with_capacity(10)), |(mut hash, mut focal, mut clef), &caractere| {
        match caractere {
            b'-' | b'=' => focal = None,
            b'0'..=b'9' => focal = Some(caractere-b'0'),
            b'a'..=b'z' => hash = {
                clef.push(caractere);
                ((hash + caractere as usize)* 17) % 256
            },
            _ => (),
        }
        (hash, focal, clef)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const ÉNONCÉ: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&ÉNONCÉ).unwrap(), 1320);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&ÉNONCÉ).unwrap(), 145);
    }

    #[test]
    fn real_tests() {
        // avec les vraies données
        assert_eq!(part1(include_str!("../inputs/input15.txt")).unwrap(), 494980);
        assert_eq!(part2(include_str!("../inputs/input15.txt")).unwrap(), 247933);
    }
}
