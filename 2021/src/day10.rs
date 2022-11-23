use std::error::Error;

/**
 * Part 1
 */
pub fn part1(input: &str) -> Result<i64, Box<dyn Error>> {
    let result = input
        .split('\n')
        .map(|ligne| {
            let mut pile = Vec::new();
            for c in ligne.chars() {
                if c == '(' || c == '[' || c == '{' || c == '<' {
                    pile.push(c);
                } else {
                    if pile.is_empty() {
                        return 0;
                    } else {
                        let dessus = pile.pop().unwrap();
                        if (dessus == '(' && c != ')')
                            || (dessus == '[' && c != ']')
                            || (dessus == '{' && c != '}')
                            || (dessus == '<' && c != '>')
                        {
                            return match c {
                                ')' => 3,
                                ']' => 57,
                                '}' => 1197,
                                '>' => 25137,
                                _ => 0,
                            };
                        }
                    }
                }
            }
            return 0;
        })
        .sum::<i64>();

    Ok(result)
}

/**
 * Part 2
 */
pub fn part2(input: &str) -> Result<i64, Box<dyn Error>> {
    let mut parentheses: Vec<i64> = input
        .split('\n')
        .map(|ligne| {
            let mut pile = Vec::new();
            for c in ligne.chars() {
                if c == '(' || c == '[' || c == '{' || c == '<' {
                    pile.push(c);
                } else {
                    if pile.is_empty() {
                        return 0;
                    } else {
                        let dessus = pile.pop().unwrap();
                        if (dessus == '(' && c != ')')
                            || (dessus == '[' && c != ']')
                            || (dessus == '{' && c != '}')
                            || (dessus == '<' && c != '>')
                        {
                            return 0;
                        }
                    }
                }
            }
            return pile.iter().rev().fold(0, |acc: i64, c| {
                let calcul = match *c {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
                    _ => 0,
                };
                calcul + 5 * acc
            });
        })
        .filter(|elt| *elt > 0)
        .collect();
    parentheses.sort();
    Ok(parentheses[(parentheses.len() - 1) / 2])
}

#[cfg(test)]
mod tests {
    use super::*;

    const ÉNONCÉ: &str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn test_part1() {
        assert_eq!(part1(ÉNONCÉ).unwrap(), 26397);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(ÉNONCÉ).unwrap(), 288957);
    }

    #[test]
    fn real_tests() {
        // avec les vraies données
        assert_eq!(part1(include_str!("../inputs/input10.txt")).unwrap(), 339477);
        assert_eq!(part2(include_str!("../inputs/input10.txt")).unwrap(), 3049320156);
    }
}
