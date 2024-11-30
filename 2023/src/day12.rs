use std::{error::Error, collections::HashMap};


/**
 * Part 1
 */
pub fn part1(input: &str) -> Result<i64, Box<dyn Error>> {
    let mut map : HashMap<(usize,usize),i64> = HashMap::new();
    Ok(input.lines().map(|s| arrangement(s, &mut map)).sum())
}

fn arrangement(line: &str, map: &mut HashMap<(usize,usize),i64>) -> i64 {
    let (springs,groups) = line.split_once(' ').unwrap();
    let springs = springs.as_bytes();
    let groups = groups.split(',').map(|c| c.parse::<usize>().unwrap()).collect::<Vec<usize>>();
    let arrangements = find_recursif(springs, &groups, map);
    map.clear();
    arrangements
}

fn find_recursif(springs: &[u8], groups: &[usize], map: &mut HashMap<(usize, usize), i64>) -> i64 {
    let springs_size = springs.len();
    let groups_size = groups.len();
    if let Some(count) = map.get(&(springs_size, groups_size)) {
        *count
    } else {
        let mut count  = 0;
        let espace_mini  = groups.iter().sum::<usize>();
        for i in 0..=springs_size - espace_mini {
            if i > 0 && springs[i - 1] == b'#' { break; }
            if groups_size == 0 { continue; }
            let offset = i + groups[0];
            if springs[i..offset].iter().all(|&b| b != b'.') {
                if groups_size == 1 && springs[offset..].iter().all(|b| *b != b'#') {
                    count += 1;
                } else if offset == springs_size || springs[offset] != b'#' && springs_size > i + espace_mini {
                    count += find_recursif(&springs[offset + 1..], &groups[1..], map);
                }
            }
        }
        map.insert((springs_size, groups_size), count);

        count
    }
}




/**
 * Part 2
 */
pub fn part2(input: &str) -> Result<i64, Box<dyn Error>> {
    let mut map : HashMap<(usize,usize),i64> = HashMap::new();
    Ok(input.lines().map(|s| arrangement(unfold(s).as_str(), &mut map)).sum())
}

fn unfold(line: &str) -> String {
    let (springs,groups) = line.split_once(' ').unwrap();
    let mut result = String::with_capacity(springs.len()*5+4+groups.len()*5+4);
    for _ in 0..4 {
        result.push_str(springs);
        result.push('?');
    }
    result.push_str(springs);
    result.push(' ');
    for _ in 0..4 {
        result.push_str(groups);
        result.push(',');
    }
    result.push_str(groups);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const ÉNONCÉ: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
    #[test]
    fn test_part1() {
        assert_eq!(part1(&ÉNONCÉ).unwrap(), 21);
    }

    #[test]
    fn test_part2() {
        // assert_eq!(part2(&ÉNONCÉ).unwrap(), 525152);
    }

    #[test]
    fn real_tests() {
        // avec les vraies données
        // assert_eq!(part1(include_str!("../inputs/input12.txt")).unwrap(), 8022);
        // assert_eq!(part2(include_str!("../inputs/input12.txt")).unwrap(), 4968620679637);
    }
}
