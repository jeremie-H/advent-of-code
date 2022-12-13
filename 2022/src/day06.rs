use std::{error::Error, thread, time::Duration};
use colored::*;

/**
 * Part 1
 */
pub fn part1(input: &str) -> Result<i64, Box<dyn Error>> {
    Ok(pos(input, 4))
}

/**
 * Part 2
 */
pub fn part2(input: &str) -> Result<i64, Box<dyn Error>> {
    Ok(pos(input, 14))

    // //solution timvisee très élégante pour garder une trace
    // let d = input.as_bytes();
    // let mut w = 0;
    // 'main: loop {
    //     let mut seen = 0u32;
    //     for i in (1..=14).rev() {
    //         //display_and_reset_screen(input, w, i);
    //         let mask = 1 << d[w + i] - b'a';
    //         if seen & mask == mask {
    //             w += i;
    //             continue 'main;
    //         }
    //         seen |= mask;
    //     }
    //     break;
    // }
    // Ok((w+15) as i64)
}

#[allow(dead_code,clippy::needless_range_loop)]
fn display_and_reset_screen(input: &str, w: usize, i:usize) {
    println!("\x1b[1J\x1b[A");//clear screeen
    print!("{}", input[0..w].to_owned());
    print!("{}", input[w..w+i].bright_purple());
    print!("{}", input[w+i..=w+14].on_red());
    print!("{}", input[w+1..].to_owned());
    println!();
    thread::sleep(Duration::from_millis(1000));
}


fn pos(input: &str, n: usize) -> i64 {
    (input.as_bytes().windows(n).position(|s| (1..s.len()).all(|i| !s[i..].contains(&s[i - 1]))).unwrap() + n) as i64 
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("mjqjpqmgbljsphdztnvjfqwrcgsmlb").unwrap(), 7);
        assert_eq!(part1("bvwbjplbgvbhsrlpgdmjqwftvncz").unwrap(), 5);
        assert_eq!(part1("nppdvjthqldpwncqszvftbrmjlhg").unwrap(), 6);
        assert_eq!(part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg").unwrap(), 10);
        assert_eq!(part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw").unwrap(), 11);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb").unwrap(), 19);
        assert_eq!(part2("bvwbjplbgvbhsrlpgdmjqwftvncz").unwrap(), 23);
        assert_eq!(part2("nppdvjthqldpwncqszvftbrmjlhg").unwrap(), 23);
        assert_eq!(part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg").unwrap(), 29);
        assert_eq!(part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw").unwrap(), 26);
    }

    #[test]
    fn real_tests() {
        // avec les vraies données
        assert_eq!(part1(include_str!("../inputs/input06.txt")).unwrap(), 1850);
        assert_eq!(part2(include_str!("../inputs/input06.txt")).unwrap(), 2823);
    }
}
