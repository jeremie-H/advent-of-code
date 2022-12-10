use std::{error::Error, str};

const ROW_SIZE:i64 =40;
/**
 * Part 1
 */
pub fn part1(input: &str) -> Result<i64, Box<dyn Error>> {
    let mut xbefore=0;
    Ok(input.lines()
    .fold((1,0, 0), |(mut x,mut signal_strength, mut cycle), line|{
        let alreadypassed = search_signal_strength(cycle, xbefore, &mut signal_strength);
        match line {
            "noop" => {
                xbefore = x;
                cycle+=1;
            },
            addx => {
                xbefore = x;
                let add_value = addx.split_once(' ').unwrap().1.parse::<i64>().unwrap();
                cycle+=1;
                if !alreadypassed { search_signal_strength(cycle, xbefore, &mut signal_strength);}
                cycle+=1;
                x += add_value;
            }
        }
        (x, signal_strength, cycle)
    }).1)

}

/**
 * Part 2
 */
pub fn part2(input: &str) -> Result<i64, Box<dyn Error>> {
    let mut xbefore=1;
    let (_,crt,_) = input.lines()
    .fold((1i64,vec![' ';ROW_SIZE as usize * 6], 0i64), |(mut x,mut crt, mut cycle), line|{
        fill_my_screen(cycle, x, &mut crt);
        match line {
            "noop" => {
                xbefore = x;
                cycle+=1;
            },
            addx => {
                xbefore = x;
                let inc = addx.split_once(' ').unwrap().1.parse::<i64>().unwrap();
                x += inc;
                cycle+=1;
                fill_my_screen(cycle, xbefore, &mut crt);
                cycle+=1;
            }
        }
        (x, crt, cycle)
    });
    display_letter(&crt);
    Ok(1)
}


fn search_signal_strength(cycle: i64, x: i64, signal_strength: &mut i64) -> bool {
    if (cycle+20)%ROW_SIZE == 0 {
        let strength=(cycle) * x;
        *signal_strength += strength;
        true
    } else { false }
}

fn fill_my_screen(cycle: i64, x: i64, crt: &mut [char]) {
        if x >= (cycle%40)-1 && x <=(cycle%40)+1 {
            crt[cycle as usize] = '█';
        }
}

/**
 * Only to display result
 */
fn display_letter(input:&[char]){
    let size: usize = ROW_SIZE as usize;
    for i in 0..6 {
        println!("{}",&input[i*size..i*size+40].iter().collect::<String>());
    };
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    const ÉNONCÉ: &str = "addx 15\naddx -11\naddx 6\naddx -3\naddx 5\naddx -1\naddx -8\naddx 13\naddx 4\nnoop\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx -35\naddx 1\naddx 24\naddx -19\naddx 1\naddx 16\naddx -11\nnoop\nnoop\naddx 21\naddx -15\nnoop\nnoop\naddx -3\naddx 9\naddx 1\naddx -3\naddx 8\naddx 1\naddx 5\nnoop\nnoop\nnoop\nnoop\nnoop\naddx -36\nnoop\naddx 1\naddx 7\nnoop\nnoop\nnoop\naddx 2\naddx 6\nnoop\nnoop\nnoop\nnoop\nnoop\naddx 1\nnoop\nnoop\naddx 7\naddx 1\nnoop\naddx -13\naddx 13\naddx 7\nnoop\naddx 1\naddx -33\nnoop\nnoop\nnoop\naddx 2\nnoop\nnoop\nnoop\naddx 8\nnoop\naddx -1\naddx 2\naddx 1\nnoop\naddx 17\naddx -9\naddx 1\naddx 1\naddx -3\naddx 11\nnoop\nnoop\naddx 1\nnoop\naddx 1\nnoop\nnoop\naddx -13\naddx -19\naddx 1\naddx 3\naddx 26\naddx -30\naddx 12\naddx -1\naddx 3\naddx 1\nnoop\nnoop\nnoop\naddx -9\naddx 18\naddx 1\naddx 2\nnoop\nnoop\naddx 9\nnoop\nnoop\nnoop\naddx -1\naddx 2\naddx -37\naddx 1\naddx 3\nnoop\naddx 15\naddx -21\naddx 22\naddx -6\naddx 1\nnoop\naddx 2\naddx 1\nnoop\naddx -10\nnoop\nnoop\naddx 20\naddx 1\naddx 2\naddx 2\naddx -6\naddx -11\nnoop\nnoop\nnoop";

    #[test]
    fn test_part1() {
        assert_eq!(part1(ÉNONCÉ).unwrap(), 13140);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(ÉNONCÉ).unwrap(), 1);
    }

    #[test]
    fn real_tests() {
        // avec les vraies données
        assert_eq!(part1(include_str!("../inputs/input10.txt")).unwrap(), 14420);
        assert_eq!(part2(include_str!("../inputs/input10.txt")).unwrap(), 1);//check le display pour la réponse
    }
}
