use std::{collections::HashSet, error::Error};

/**
 * Part 1
 */
pub fn part1(input: &str) -> Result<i64, Box<dyn Error>> {
    let mut trace: HashSet<(i64, i64)> = HashSet::with_capacity(input.len() + 1);
    let mut last_coord = (0, 0);
    trace.insert(last_coord);
    input.chars().for_each(|d| {
        let mov = mouvement(d);
        last_coord.0 += mov.0;
        last_coord.1 += mov.1;
        trace.insert(last_coord);
    });
    Ok(trace.len() as i64)
}

fn mouvement(d: char) -> (i64, i64) {
    match d {
        '^' => (1, 0),
        '<' => (0, -1),
        'v' => (-1, 0),
        '>' => (0, 1),
        _ => (0, 0),
    }
}

/**
 * Part 2
 */
pub fn part2(input: &str) -> Result<i64, Box<dyn Error>> {
    let mut trace_santa: HashSet<(i64, i64)> = HashSet::with_capacity(input.len() / 2 + 1);
    let mut trace_robot: HashSet<(i64, i64)> = HashSet::with_capacity(input.len() / 2 + 1);
    let mut last_coord_santa = (0, 0);
    let mut last_coord_robot = (0, 0);

    trace_santa.insert(last_coord_santa);
    trace_robot.insert(last_coord_robot);
    input.chars().step_by(2).for_each(|d| {
        let mov = mouvement(d);
        last_coord_santa.0 += mov.0;
        last_coord_santa.1 += mov.1;
        trace_santa.insert(last_coord_santa);
    });
    input.chars().skip(1).step_by(2).for_each(|d| {
        let mov = mouvement(d);
        last_coord_robot.0 += mov.0;
        last_coord_robot.1 += mov.1;
        trace_robot.insert(last_coord_robot);
    });
    let union = trace_robot.union(&trace_santa).collect::<HashSet<&(i64, i64)>>();
    Ok(union.len() as i64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(">").unwrap(), 2);
        assert_eq!(part1("^>v<").unwrap(), 4);
        assert_eq!(part1("^v^v^v^v^v").unwrap(), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("^v").unwrap(), 3);
        assert_eq!(part2("^>v<").unwrap(), 3);
        assert_eq!(part2("^v^v^v^v^v").unwrap(), 11);
    }

    #[test]
    fn real_tests() {
        // avec les vraies données
        assert_eq!(part1(include_str!("../inputs/input03.txt")).unwrap(), 2592);
        assert_eq!(part2(include_str!("../inputs/input03.txt")).unwrap(), 2360);
    }
}
