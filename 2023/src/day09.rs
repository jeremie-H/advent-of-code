use std::error::Error;

use itertools::Itertools;

/**
 * Part 1
 */
pub fn part1(input: &str) -> Result<i64, Box<dyn Error>> {
    Ok(input.lines().map(|f| prediction(f.split(' ').map(|s| s.parse::<i64>().unwrap()).collect_vec())).sum())
}

fn prediction(split: Vec<i64>) -> i64 {
    //println!("prediction {:?}", split);
    let mut develop: Vec<Vec<i64>> = Vec::<Vec<i64>>::new();
    
    let mut intermediaire;
    let mut audessus = split;
    let mut end = false;
    while !end {
        end = true;
        intermediaire = vec![0; audessus.len()-1];
        for i in 0..audessus.len()-1 {
            intermediaire[i] = audessus[i+1] - audessus[i];
            end &= intermediaire[i] == 0;
        }
        // println!("intermediaire {:?}", intermediaire);
        develop.push(audessus.clone());
        audessus = intermediaire;
        
        
    }
    develop.push(audessus.clone());
    // println!("develop {:?}", develop);
    let mut result = 0;
    for d in develop.iter().rev() {
        result += d[d.len()-1];
        // println!("d {:?}, r = {}", d, result);
    }
    // println!("prediction {:?} = {}", develop[0], result);
    result
}


/**
 * Part 2
 */
pub fn part2(input: &str) -> Result<i64, Box<dyn Error>> {
    Ok(input.lines().map(|f| prediction_backward(f.split(' ').map(|s| s.parse::<i64>().unwrap()).collect_vec())).sum())
}

fn prediction_backward(split: Vec<i64>) -> i64 {
    //println!("prediction {:?}", split);
    let mut develop: Vec<Vec<i64>> = Vec::<Vec<i64>>::new();
    
    let mut intermediaire;
    let mut audessus = split;
    let mut end = false;
    while !end {
        end = true;
        intermediaire = vec![0; audessus.len()-1];
        for i in 0..audessus.len()-1 {
            intermediaire[i] = audessus[i+1] - audessus[i];
            end &= intermediaire[i] == 0;
        }
        develop.push(audessus.clone());
        audessus = intermediaire;
        
        
    }
    develop.push(audessus.clone());
    // println!("develop {:?}", develop);
    let mut result = 0;
    for d in develop.iter().rev() {
        result = d[0] - result;
        // println!("d {:?}, r = {}", d, result);
    }
    // println!("prediction {:?} = {}", develop[0], result);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const ÉNONCÉ: &str = "0 3 6 9 12 15\n1 3 6 10 15 21\n10 13 16 21 30 45";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&ÉNONCÉ).unwrap(), 114);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&ÉNONCÉ).unwrap(), 2);
    }

    #[test]
    fn real_tests() {
        // avec les vraies données
        assert_eq!(part1(include_str!("../inputs/input09.txt")).unwrap(), 1992273652);
        assert_eq!(part2(include_str!("../inputs/input09.txt")).unwrap(), 1012);
    }
}
