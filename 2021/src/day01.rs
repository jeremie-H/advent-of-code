use std::error::Error;

/**
 * Part 1
 */
pub fn part1(input: &str) -> Result<i64, Box<dyn Error>> {
    let mut monmax = 0;
    let mut compteur = 0;
    for line in input.split('\n') {
        let monentier = line.parse::<i32>().unwrap();
        if monentier > monmax {
            compteur += 1;
        }
        monmax = monentier;
    }
    Ok(compteur - 1)
}

/**
 * Part 2
 */
pub fn part2(input: &str) -> Result<i64, Box<dyn Error>> {
    let (mut a, mut b, mut c) = (0, 0, 0);
    let (mut totaln, mut totalnmoins1);
    let mut compteur = 0;
    for line in input.split('\n') {
        let monentier = line.parse::<i32>().unwrap();
        if a == 0 {
            a = monentier;
            continue;
        }
        if b == 0 {
            b = monentier;
            continue;
        }
        if c == 0 {
            c = monentier;
            continue;
        }

        totalnmoins1 = a + b + c;

        a = b;
        b = c;
        c = monentier;
        totaln = a + b + c;

        if totaln > totalnmoins1 {
            compteur += 1;
        }
    }
    println!("{}", compteur);
    Ok(compteur)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("199\n200\n208\n210\n200\n207\n240\n269\n260\n263").unwrap(), 7);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("607\n618\n618\n617\n647\n716\n769\n792").unwrap(), 5);
    }

    #[test]
    fn real_tests() {
        // avec les vraies données
        assert_eq!(part1(include_str!("../inputs/input01.txt")).unwrap(), 1475);
        assert_eq!(part2(include_str!("../inputs/input01.txt")).unwrap(), 1516);
    }
}
