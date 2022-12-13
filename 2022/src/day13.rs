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
    //println!("tabl {:?}", ordered_signals);
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
    if left[0] == b'[' && right[0] == b'[' {//gestion 2 listes
        let mut i = 1;
        let mut j = 1;
        while left[i] != b']' && right[j] != b']' {
            let l = left[i];
            let r = right[j];
            //on incrémente dès le début, ça nous évite de dupliquer ce code à chaque fois qu'on appelle continue;
            //et si besoin d'accéder à la valeur de i, ou j, on fait i-1 et j-1
            i += 1;
            j += 1;

            //analyse des 2 premiers caractères
            match (l,r) {
                (b'0'..=b'A', b'0'..=b'A') => {
                    
                    if l == r {continue;}
                    else {return l.cmp(&r); }
                },

                (b',', b',') => continue,

                (b'[', b'[') => {
                    let subleft = sub_list(&left[i-1..]);
                    let subright = sub_list(&right[j-1..]);
                    let sousresultat = compare(subleft, subright);
                    if sousresultat == Ordering::Equal {
                        i += subleft.len()-1;
                        j += subright.len()-1;
                        continue;
                    }
                    else {return sousresultat;}
                },

                (b'[', b'0'..=b'A') => {
                    let subleft = sub_list(&left[i-1..]);
                    let subright = [b'[', r, b']'];
                    let sousresultat = compare(subleft, &subright);
                    if sousresultat == Ordering::Equal {
                        i += subleft.len()-1;
                        continue;
                    }
                    else {return sousresultat;}
                },

                (b'0'..=b'A', b'[') => {
                    let subleft = [b'[', l, b']'];
                    let subright = sub_list(&right[j-1..]);
                    let sousresultat = compare(&subleft, subright);
                    if sousresultat == Ordering::Equal {
                        j += subright.len()-1;
                        continue;
                    }
                    else {return sousresultat;}
                },
                _ => panic!("unknow chars")
            }
        }
        //left side ran out of items
        if left[i] == b']' && right[j] != b']' {
            return Ordering::Less;
        }
        //right side ran out of items
        if left[i] != b']' && right[j] == b']' {
            return Ordering::Greater;
        }
    }
    Ordering::Equal
}

/**
 * search for closing ']'
 * attention : dans les cas où l'on a [1,2,[3,[4]]]
 * on risquerai de s'arrêter ici ---------------^
 * il faut donc compter également le nombre de '[' ouvrant
 */
fn sub_list(substr: &[u8]) -> &[u8] {
    let mut cout_bracket=1;
    for i in 1..substr.len() {
        if substr[i] == b'[' { cout_bracket += 1;}
        if substr[i] == b']' {
            cout_bracket -= 1;
            if cout_bracket == 0 {
                return &substr[..i+1];
            }
        }
    }
    substr
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
