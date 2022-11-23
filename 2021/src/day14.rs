use itertools::Itertools;
use std::{collections::HashMap, error::Error, str::FromStr};

/**
 * Helps
 * je décompose tout
 NNCB

NN NC CB

NCNBCHB

BB -> (N) BN & NB ->     ->     =>     ->
BC -> (B) BB & BC ->     -> +1B =>  1B -> 
BH -> (H) BH & HH ->     ->     =>     ->
BN -> (B) BB & BN ->     ->     =>     ->
CB -> (H) CH & HB ->  1H ->     =>     ->
CC -> (N) CN & NC ->     ->     =>     ->
CH -> (B) CB & BH ->     -> +1B =>  1B ->
CN -> (C) CC & CN ->     -> +1C =>  1C ->
HB -> (C) HC & CB ->     -> +1C =>  1C ->
HC -> (B) HB & BC ->     ->     =>     ->
HH -> (N) HN & NH ->     ->     =>     ->
HN -> (C) HC & CN ->     ->     =>     ->
NB -> (B) NB & BB ->     -> +1B =>  1B ->
NC -> (B) NB & BC ->  1B -> +1B =>  2B ->
NH -> (C) NC & CH ->     ->     =>     ->
NN -> (C) NC & CN ->  1C ->     =>     ->

NNCB + 1H, 1B, 1C
=>B:2 C:2 H:1 N:2
=> NCNBCHB

(B:2 C:2 H:1 N:2) + 1B+1B+1C+1C+2B => 4B 2C
=>B:6 C:4 H:1 N:2
=> NBCCNBBBCBHCB

 */

/**
 * Part 1
 */
pub fn part1(input: &str) -> Result<i64, Box<dyn Error>> {
    let (gabarit, mut alphabet, insertions) = init_structures(input);
    let mut step_array = init_step_array(&insertions, gabarit);

    (0..10).for_each(|_| {
        step_array = loop_sur_insertions(&insertions, &mut alphabet, &step_array);
    });

    display_alphabet(&alphabet);
    let result = alphabet.values().max().unwrap_or(&0i64) - alphabet.values().min().unwrap_or(&0i64);
    Ok(result)
}

/**
 * Part 2
 */
pub fn part2(input: &str) -> Result<i64, Box<dyn Error>> {
    let (gabarit, mut alphabet, insertions) = init_structures(input);
    let mut step_array = init_step_array(&insertions, gabarit);

    (0..40).for_each(|_| {
        //display_step(&step_array);
        step_array = loop_sur_insertions(&insertions, &mut alphabet, &step_array);
        //display_alphabet(&alphabet);
    });

    display_alphabet(&alphabet);

    let result = alphabet.values().max().unwrap_or(&0i64) - alphabet.values().min().unwrap_or(&0i64);
    Ok(result)
}

/**
 * initialise les 3 structures
 */
fn init_structures<'a>(input: &'a str) -> (&'a str, HashMap<char, i64>, HashMap<&'a str, &'a str>) {
    let (gabarit, insertions) = input.split_once("\n\n").unwrap();
    let alphabet = gabarit.chars().fold(HashMap::new(), |mut acc, lettre| {
        let l = acc.entry(lettre).or_insert(0i64);
        *l += 1;
        acc
    });
    let insertions = insertions.lines().fold(HashMap::new(), |mut hash, ligne| {
        hash.insert(ligne.split_once(" -> ").unwrap().0, ligne.split_once(" -> ").unwrap().1);
        hash
    });
    (gabarit, alphabet, insertions)
}

/**
 * initialise le step_array avec les valeurs données au gabarit d'input
 */
fn init_step_array(insertions: &HashMap<&str, &str>, gabarit: &str) -> HashMap<String, i64> {
    //initialise le tableau de mappings avec des 0
    let mut step_array = insertions.iter().fold(HashMap::new(), |mut newhashmap, (key, _val)| {
        newhashmap.insert(String::from_str(*key).unwrap(), 0);
        newhashmap
    });
    gabarit.as_bytes().windows(2).for_each(|deux_chars| {
        let key = String::from_utf8_lossy(deux_chars);
        *step_array.entry(key.to_string()).or_insert(1) += 1;
    });
    step_array
}

/**
 * loop sur le tableau des insertions
 * insertions : NN => C => m1=NC & m2=CN
 * step_avant / step attendu
 * NN => 1      NN => 1
 * NC => 0      NC => 1
 * CN => 0      CN => 1
 */
fn loop_sur_insertions(
    insertions: &HashMap<&str, &str>,
    alphabet: &mut HashMap<char, i64>,
    step_avant: &HashMap<String, i64>,
) -> HashMap<String, i64> {
    let step_reset = step_avant.keys().map(|c| (c.to_owned(), 0_i64)).collect::<HashMap<String, i64>>();

    step_avant.iter().fold(step_reset, |mut newhashmap, (key, &val)| {
        let mapping = insertions.get(&key.as_ref()).unwrap();
        //print!("key = {} => ",key);
        let (m1, m2) = (format!("{}{}", &key[0..1], mapping), format!("{}{}", mapping, &key[1..2]));
        //print!("m1={}, m2={}",m1,m2);
        //println!(" => {} = {}",mapping, val);
        *alphabet.entry(mapping.as_bytes()[0] as char).or_insert(0) += val;

        *newhashmap.entry(m1).or_insert(0) += val;
        *newhashmap.entry(m2).or_insert(0) += val;

        newhashmap
    })
}

/**
 * display occurrences
 */
fn display_alphabet(alphabet: &HashMap<char, i64>) {
    print!("alphabet : ");
    for k in alphabet.keys().sorted() {
        print!("{}:{} ", k, alphabet.get(k).unwrap());
    }
    println!();
}

#[allow(dead_code)]
fn display_step(alphabet: &HashMap<String, i64>) {
    print!("step : ");
    for k in alphabet.keys().sorted() {
        print!("{}:{} ", k, alphabet.get(k).unwrap());
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    const ÉNONCÉ: &str = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

    #[test]
    fn test_part1() {
        // avec les données d'essai de la partie 1
        assert_eq!(part1(ÉNONCÉ).unwrap(), 1588);
    }

    #[test]
    fn test_part2() {
        // avec les données d'essai de la partie 2
        assert_eq!(part2(ÉNONCÉ).unwrap(), 2188189693529);
    }

    #[test]
    fn real_tests() {
        // avec les vraies données
        assert_eq!(part1(include_str!("../inputs/input14.txt")).unwrap(), 2899);
        assert_eq!(part2(include_str!("../inputs/input14.txt")).unwrap(), 3528317079545);
    }
}

