use std::error::Error;

/**
 * Part 1
 */
pub fn part1(input: &str) -> Result<i64, Box<dyn Error>> {
    Ok(input.lines().fold(0, |acc, l| {
        let (gamen, subsets) = l.split_once(": ").unwrap();
        let n = gamen.chars().filter(|c| c.is_numeric()).collect::<String>().parse::<i64>().unwrap();
        let eligible = subsets.split("; ").fold(true, |acc, subset| test_subset(subset, 12, 13, 14) && acc);
        if eligible { acc + n }
        else { acc }
    }))
}

fn test_subset(subset: &str, rcubes: i64, gcubes: i64, bcubes: i64) -> bool {
    subset.split(", ").fold(true, |acc, cube| {
        let (n, color) = cube.split_once(" ").unwrap();
        let n = n.parse::<i64>().unwrap();
        (match color {
            "red" => n <= rcubes,
            "green" => n <= gcubes,
            "blue" => n <= bcubes,
            _ => false,
        } && acc)
    })
}


/**
 * Part 2
 */
pub fn part2(input: &str) -> Result<i64, Box<dyn Error>> {
    Ok(input.lines().fold(0, |acc, l| {
        let (_, subsets) = l.split_once(": ").unwrap();
        let power = power_cubes(subsets);
        acc + power
    }))
}

fn power_cubes(subsets: &str) -> i64 {
    subsets.split("; ").fold([1, 1, 1], |acc, subset| {
        subset.split(", ").fold(acc, |mut acc, cube| {
            let (n, color) = cube.split_once(" ").unwrap();
            let n = n.parse::<i64>().unwrap();
            match color {
                "red" => acc[0] = n.max(acc[0]),
                "green" => acc[1] = n.max(acc[1]),
                "blue" => acc[2] = n.max(acc[2]),
                _ => (),
            };
            acc
        })
    }).iter().product()
}

#[cfg(test)]
mod tests {
    use super::*;
    const ÉNONCÉ: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    #[test]
    fn test_part1() {
        assert_eq!(part1(&ÉNONCÉ).unwrap(),8);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&ÉNONCÉ).unwrap(), 2286);
    }

    #[test]
    fn real_tests() {
        // avec les vraies données
        assert_eq!(part1(include_str!("../inputs/input02.txt")).unwrap(), 2593);
        assert_eq!(part2(include_str!("../inputs/input02.txt")).unwrap(), 54699);
    }
}
