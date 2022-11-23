use std::error::Error;

/**
 * Part 1
 */
pub fn part1(input: &str) -> Result<i64, Box<dyn Error>> {
    let mut gamma = 0;
    let nbits = input.get(0..input.find('\n').unwrap()).unwrap().len();

    let tab: Vec<u16> = input
        .split('\n')
        //.inspect(|f| println!("{:?}",f))
        .map(|line| u16::from_str_radix(line, 2).unwrap())
        .collect();

    let mut binding = Vec::<i32>::with_capacity(nbits);
    (0..nbits).for_each(|_| binding.push(0));
    let compteur_bits = binding.as_mut_slice();

    for one_elt in tab {
        for i in (0..nbits).rev() {
            if (one_elt & 0x01 << i) == 0 {
                compteur_bits[nbits - 1 - i] -= 1;
            } else {
                compteur_bits[nbits - 1 - i] += 1;
            }
        }
    }
    println!("compteur_bits {:?}", compteur_bits);
    let mut i: i32 = (nbits - 1) as i32;
    for e in compteur_bits {
        let bit = if *e > 0 { 1 } else { 0 };
        gamma += bit << i;
        i -= 1;
    }
    let mask = if nbits == 5 { 0b11111 } else { 0b111111111111 };
    let epsilon = !gamma & mask;
    let result = gamma * epsilon;
    println!("gamma({})*epsilon({}) = {}", gamma, epsilon, result);
    Ok(result)
}

/**
 * Part 2
 */
pub fn part2(input: &str) -> Result<i64, Box<dyn Error>> {
    let (oxygen, co2);
    let nbits = input.get(0..input.find('\n').unwrap()).unwrap().len();

    let tab: Vec<u16> = input
        .split('\n')
        //.inspect(|f| println!("{:?}",f))
        .map(|line| u16::from_str_radix(line, 2).unwrap())
        .collect();

    let mut binding = Vec::<i32>::with_capacity(nbits);
    (0..nbits).for_each(|_| binding.push(0));
    let compteur_bits = binding.as_mut_slice();
    let mut copie_tab: Vec<u16> = tab.clone();
    //search oxygen
    let closure_oxygen = |entier: u16, indice: usize, valeur: i32| {
        if valeur >= 0 {
            entier & 0x01 << indice == 0x01 << indice
        } else {
            !entier & 0x01 << indice == 0x01 << indice
        }
    };
    oxygen = loop_inputs(&mut copie_tab, compteur_bits, closure_oxygen);

    //search co2
    copie_tab = tab;
    let mut binding = Vec::<i32>::with_capacity(nbits);
    (0..nbits).for_each(|_| binding.push(0));
    let compteur_bits = binding.as_mut_slice();
    //compteur_bits = [0; 12];
    let closure_co2 = |entier: u16, indice: usize, valeur: i32| {
        if valeur < 0 {
            entier & 0x01 << indice == 0x01 << indice
        } else {
            !entier & 0x01 << indice == 0x01 << indice
        }
    };
    co2 = loop_inputs(&mut copie_tab, compteur_bits, closure_co2);
    let result = co2 as i64 * oxygen as i64;
    println!("co2({})*oxygen({}) = {}", co2, oxygen, result);
    Ok(result)
}

fn loop_inputs(tableau: &mut Vec<u16>, compteur_bits: &mut [i32], filtre: impl Fn(u16, usize, i32) -> bool) -> u16 {
    let taille = compteur_bits.len();
    for i in (0..taille).rev() {
        for un_entier in &*tableau {
            if (un_entier & 0x01 << i) == 0 {
                compteur_bits[taille - 1 - i] -= 1;
            } else {
                compteur_bits[taille - 1 - i] += 1;
            }
        }
        //println!("compteur_bits {:?}", compteur_bits);
        *tableau = tableau
            .iter()
            .cloned()
            .filter(|e| filtre(*e, i, compteur_bits[taille - 1 - i]))
            .collect::<Vec<u16>>();

        if tableau.len() < 2 {
            break;
        }
    }
    tableau[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1("00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010").unwrap(),
            22 * 9
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2("00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010").unwrap(),
            23 * 10
        );
    }

    #[test]
    fn real_tests() {
        // avec les vraies données
        assert_eq!(part1(include_str!("../inputs/input03.txt")).unwrap(), 2003336);
        assert_eq!(part2(include_str!("../inputs/input03.txt")).unwrap(), 1877139);
    }
}
