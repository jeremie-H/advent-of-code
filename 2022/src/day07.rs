use std::error::Error;
use std::iter::{self, Iterator};

/**
 * Part 1
 */
pub fn part1(input: &str) -> Result<i64, Box<dyn Error>> {
    let mut it = input.lines();
    it.next().unwrap();
    let result = walk_tree(&mut it);
    let sum = size_dir(&result)
        .filter(|&size| size <= 100000)
        .sum();
    Ok(sum)
}



/**
 * Part 2
 */
pub fn part2(input: &str) -> Result<i64, Box<dyn Error>> {
    let mut it = input.lines();
    it.next().unwrap();
    let result = walk_tree(&mut it);
    let r = size_dir(&result)
        .filter(|&size| size > result.size - (70000000 - 30000000))
        .min()
        .unwrap();
    Ok(r)
}


struct Dir {
    size: i64,
    dirs: Vec<Dir>,
}

fn walk_tree<'a>(cmds: &mut impl Iterator<Item = &'a str>) -> Dir {
    let mut dirs: Vec<Dir> = vec![];
    let mut size = 0;

    while let Some(line) = cmds.next() {
        match line {
            s if s.starts_with("$ ls") => {continue;},
            s if s.starts_with("dir") => {continue;},
            s if s.eq("$ cd ..") => {break;},
            s if s.starts_with("$ cd ") => {
                let sub_dir = walk_tree(cmds);
                size += sub_dir.size;
                dirs.push(sub_dir);
            },
            _ => {
                size += line
                .split_ascii_whitespace()
                .next()
                .unwrap()
                .parse::<i64>()
                .unwrap();
            }
        };
    }
    Dir { size, dirs }
}

fn size_dir<'a>(directory: &'a Dir) -> Box<dyn Iterator<Item = i64> + 'a > {
    Box::new(iter::once(directory.size).chain(
        directory.dirs.iter().flat_map(size_dir)
    ))
}


#[cfg(test)]
mod tests {
    use super::*;

    const ÉNONCÉ: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn test_part1() {
        assert_eq!(part1(ÉNONCÉ).unwrap(), 95437);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(ÉNONCÉ).unwrap(), 24933642);
    }

    #[test]
    fn real_tests() {
        // avec les vraies données
        assert_eq!(part1(include_str!("../inputs/input07.txt")).unwrap(), 1555642);
        assert_eq!(part2(include_str!("../inputs/input07.txt")).unwrap(), 5974547);
    }
}
