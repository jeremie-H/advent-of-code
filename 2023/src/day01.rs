use std::error::Error;

/**
 * Part 1
 */
pub fn part1(input: &str) -> Result<i64, Box<dyn Error>> { Ok(somme_des_nombres(input)) }

/**
 * Part 2
 */
pub fn part2(input: &str) -> Result<i64, Box<dyn Error>> {
    let transform = input.lines().fold(String::new(), |mut acc, l| {
        let l = l
            .replace("one", "o1e")
            .replace("two", "t2o")
            .replace("three", "t3e")
            .replace("four", "f4r")
            .replace("five", "f5e")
            .replace("six", "s6x")
            .replace("seven", "s7n")
            .replace("eight", "e8t")
            .replace("nine", "n9e");
        acc.push_str(l.as_str());
        acc.push('\n');
        acc
    });
    Ok(somme_des_nombres(&transform))
}

fn somme_des_nombres(lines: &str) -> i64 {
    lines.lines().fold(0, |acc, l| {
        let nombre = l.chars().filter(|c| c.is_numeric()).map(|c| (c as u8) - b'0').fold(Vec::new(), |mut acc, entier| {
            acc.push(entier);
            acc
        });
        let nombre = 10 * nombre[0] + nombre[nombre.len() - 1];
        acc + nombre as i64
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet").unwrap(), 142);
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2("two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen").unwrap(),
            281
        );
    }

    #[test]
    fn real_tests() {
        // avec les vraies données
        assert_eq!(part1(include_str!("../inputs/input01.txt")).unwrap(), 54630);
        assert_eq!(part2(include_str!("../inputs/input01.txt")).unwrap(), 54770);
    }
}
