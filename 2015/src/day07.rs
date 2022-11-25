use std::{collections::HashMap, error::Error};

use regex::Regex;

/**
 * Part 1
 */
pub fn part1(input: &str) -> Result<i64, Box<dyn Error>> {
    let instructions = input.lines().fold(HashMap::new(), |mut hm, ligne| {
        let (valg, vald) = ligne.split_once(" -> ").unwrap();
        hm.insert(vald, valg);
        hm
    });
    let mut calcul: HashMap<&str, u16> = HashMap::new();
    let resultat = process_one_instruction(&instructions, &mut calcul, "a");
    Ok(resultat as i64)
}

fn process_one_instruction<'a>(instructions: &HashMap<&str, &'a str>, calcul: &mut HashMap<&'a str, u16>, variable: &'a str) -> u16 {
    let operation = instructions.get(variable).unwrap();
    //println!("{} => {}", operation, variable);
    let expression = operation.split(' ').collect::<Vec<&str>>();
    let r = match expression.len() {
        3 => process_3(instructions, calcul, expression),
        2 => !get_operande(instructions, calcul, expression[1]),
        1 => get_operande(instructions, calcul, expression[0]),
        _ => panic!("expression avec trop de paramètres : {}", operation),
    };
    //println!("{} -> {}",r,variable);
    calcul.insert(variable, r);
    r
}

fn process_3<'a>(instructions: &HashMap<&str, &'a str>, calcul: &mut HashMap<&'a str, u16>, expression: Vec<&'a str>) -> u16 {
    let op1 = get_operande(instructions, calcul, expression[0]);
    let op2 = get_operande(instructions, calcul, expression[2]);
    match expression[1] {
        "AND" => op1 & op2,
        "OR" => op1 | op2,
        "LSHIFT" => op1 << op2,
        "RSHIFT" => op1 >> op2,
        _ => panic!("expression inconnue : {:?}", expression),
    }
}

fn get_operande<'a>(instructions: &HashMap<&str, &'a str>, calcul: &mut HashMap<&'a str, u16>, nom: &'a str) -> u16 {
    if let Some(&val) = calcul.get(nom) {
        val
    } else if let Ok(nombre) = nom.parse::<u16>() {
        nombre
    } else {
        process_one_instruction(instructions, calcul, nom)
    }
}

/**
 * Part 2
 */
pub fn part2(input: &str) -> Result<i64, Box<dyn Error>> {
    let p1 = part1(input);
    let rg = Regex::new(".* -> b\n").unwrap();
    let new_input = rg.replace(input, format!("{} -> b\n", p1.unwrap()));
    part1(&new_input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const ÉNONCÉ: &str = "123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&ÉNONCÉ.replace('d', "a")).unwrap(), 72);
        assert_eq!(part1(&ÉNONCÉ.replace('e', "a")).unwrap(), 507);
        assert_eq!(part1(&ÉNONCÉ.replace('f', "a")).unwrap(), 492);
        assert_eq!(part1(&ÉNONCÉ.replace('g', "a")).unwrap(), 114);
        assert_eq!(part1(&ÉNONCÉ.replace('h', "a")).unwrap(), 65412);
        assert_eq!(part1(&ÉNONCÉ.replace('i', "a")).unwrap(), 65079);
    }

    #[test]
    fn test_part2() {
        //pas vraiment applicable car il n'y a pas de a, b dans l'énoncé
    }

    #[test]
    fn real_tests() {
        // avec les vraies données
        assert_eq!(part1(include_str!("../inputs/input07.txt")).unwrap(), 16076);
        assert_eq!(part2(include_str!("../inputs/input07.txt")).unwrap(), 2797);
    }
}
