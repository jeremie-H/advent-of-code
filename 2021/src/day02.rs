use std::error::Error;

/**
 * Part 1
 */
pub fn part1(input: &str) -> Result<i64, Box<dyn Error>> {
    let (mut forward, mut depth) = (0, 0);
    for line in input.split('\n') {
        let length = line.len();
        //let mut parts = line.split_whitespace().map(|s| s.parse::<i32>());
        let firstchar = line.get(..1);
        match firstchar {
            Some("f") => forward += line.get(length - 1..length).unwrap().parse::<i32>().unwrap(),
            Some("u") => depth -= line.get(length - 1..length).unwrap().parse::<i32>().unwrap(),
            Some("d") => depth += line.get(length - 1..length).unwrap().parse::<i32>().unwrap(),
            None => println!("None"),
            _ => println!("unknown"),
        }
    }
    let result = (forward * depth) as i64;
    println!("forward x depth = {}", result);
    Ok(result)
}

/**
 * Part 2
 */
pub fn part2(input: &str) -> Result<i64, Box<dyn Error>> {
    let (mut forward, mut depth, mut aim) = (0, 0, 0);
    for line in input.split('\n') {
        let length = line.len();
        //let mut parts = line.split_whitespace().map(|s| s.parse::<i32>());
        let firstchar = line.get(..1);
        match firstchar {
            Some("f") => {
                let x = line.get(length - 1..length).unwrap().parse::<i32>().unwrap();
                forward += x;
                depth += aim * x;
            }
            Some("u") => {
                let x = line.get(length - 1..length).unwrap().parse::<i32>().unwrap();
                aim -= x;
            }
            Some("d") => {
                let x = line.get(length - 1..length).unwrap().parse::<i32>().unwrap();
                aim += x;
            }
            None => println!("None"),
            _ => println!("unknown"),
        }
    }
    let result = (forward * depth) as i64;
    println!("forward x depth = {}", result);
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2").unwrap(), 150);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2").unwrap(), 900);
    }

    #[test]
    fn real_tests() {
        // avec les vraies données
        assert_eq!(part1(include_str!("../inputs/input02.txt")).unwrap(), 1868935);
        assert_eq!(part2(include_str!("../inputs/input02.txt")).unwrap(), 1965970888);
    }
}
