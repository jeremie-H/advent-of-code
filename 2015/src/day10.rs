use std::collections::HashMap;
use std::error::Error;

/**
 * Part 1
 */
pub fn part1(input: &str) -> Result<i64, Box<dyn Error>> {
    let mut input = String::from(input);
    spell_digits(&mut input, 40);
    Ok(input.len() as i64)
}

fn spell_digits(input: &mut String, loopnb: isize) {
    let mut table: HashMap<String, String> = HashMap::new();

    // Read initial input from user
    table.insert(input.trim().to_string(), input.trim().to_string());

    for _i in 0..loopnb {
        // Generate the new string based on the input digits
        let mut new_string = String::new();
        let mut digit_count = 0;
        let mut last_digit = ' ';
        for c in input.trim().chars() {
            if c == last_digit {
                digit_count += 1;
            } else {
                if digit_count > 0 {
                    new_string.push_str(&digit_count.to_string());
                    new_string.push(last_digit);
                }
                last_digit = c;
                digit_count = 1;
            }
        }
        if digit_count > 0 {
            new_string.push_str(&digit_count.to_string());
            new_string.push(last_digit);
        }

        // Print the new string and use it as the input for the next iteration
        //println!("{}", new_string);
        *input = new_string;
    }
}

/**
 * Part 2
 */
pub fn part2(input: &str) -> Result<i64, Box<dyn Error>> {
    let mut input = String::from(input);
    spell_digits(&mut input, 50);
    Ok(input.len() as i64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("1").unwrap(), 82350);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("1").unwrap(), 1166642);
    }

    #[test]
    fn real_tests() {
        // avec les vraies données
        assert_eq!(part1(include_str!("../inputs/input10.txt")).unwrap(), 329356);
        assert_eq!(part2(include_str!("../inputs/input10.txt")).unwrap(), 4666278);
    }
}
