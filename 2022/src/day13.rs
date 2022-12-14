use std::cmp::Ordering;
use std::error::Error;
use std::str;
/**
 * Part 1
 */
pub fn part1(input: &str) -> Result<i64, Box<dyn Error>> {
    //le cas du 10 va nous empêcher de comparer byte à byte, alors on remplace par un caractère supérieur à 9 dans la table utf-8
    let input = input.replace("10", "A");
    let result = input.split("\n\n")
    .enumerate()
    .filter(|(_,pair)|isrightorder(pair))
    .map(|(i,_)| i as i64 + 1 )
    .sum();
    Ok(result)
}

/**
 * Part 2
 * un cas chiant à gérer c'est le compare : "[[1],[2,3,4]]", "[1,1,3,1,1]"
 */
pub fn part2(input: &str) -> Result<i64, Box<dyn Error>> {
    //le cas du 10 va nous empêcher de comparer byte à byte, alors on remplace par un caractère supérieur à 9 dans la table utf-8
    let input = input.replace("10", "A");
    let mut input = input.replace("\n\n", "\n");//trim all blank line
    input.push_str("\n[[2]]");
    input.push_str("\n[[6]]");
    let mut ordered_signals = input.lines().collect::<Vec<&str>>();
    ordered_signals.sort_by(|left,right| compare(left.as_bytes(), right.as_bytes()));
    let position2 = ordered_signals.iter().position(|l| l.eq(&"[[2]]")).unwrap() as i64 + 1;
    let position6 = ordered_signals.iter().position(|l| l.eq(&"[[6]]")).unwrap() as i64 + 1;
    let result = position2*position6;
    Ok(result)
}


fn isrightorder(pair: &str) -> bool {
    let (left, right) = pair.split_once('\n').unwrap();
    compare(left.as_bytes(), right.as_bytes()) == Ordering::Less
}

// retourne Ordering::Less si c'est dans le bon ordre
// retourne Ordering::Equals si on doit continuer l'analyse
// retourne Ordering::Greater si c'est pas dans le bon ordre
fn compare(left: &[u8], right: &[u8]) -> Ordering {
    //analyse des 2 premiers caractères
    match (left[0],right[0]) {
        (a,b) if a==b => compare(&left[1..], &right[1..]),
        (_, b']') => Ordering::Greater,
        (b']', _) => Ordering::Less,
        (b'[', _) => {
            let subright = [&[right[0], b']'],&right[1..]].concat();
            compare(&left[1..], &subright)
        },
        (_, b'[') => {
            let subleft = [&[left[0], b']'],&left[1..]].concat();
            compare(&subleft, &right[1..])
        },
        (_,_) => left[0].cmp(&right[0])
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    const ÉNONCÉ: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn test_part1() {
        assert_eq!(part1(ÉNONCÉ).unwrap(), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(ÉNONCÉ).unwrap(), 140);
    }

    #[test]
    fn real_tests() {
        // avec les vraies données
        assert_eq!(part1(include_str!("../inputs/input13.txt")).unwrap(), 5760);
        assert_eq!(part2(include_str!("../inputs/input13.txt")).unwrap(), 26670);
    }
}
