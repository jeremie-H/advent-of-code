use std::{error::Error, mem};

/**
 * Part 1
 */
pub fn part1(input: &str) -> Result<i64, Box<dyn Error>> {
    let mut result = 0;
    let mut tableau: Vec<Vec<u8>> = input.lines().map(|l| l.bytes().map(|c| c).collect()).collect();
    //display(&tableau);

    let hauteur = tableau.len();
    let largeur = tableau[0].len();

    let mut working_array: Vec<Vec<u8>>;

    for runs in 1..2000 {
        working_array = vec![vec![0; tableau[0].len()]; tableau.len()];
        let mut deplacement: bool = false;
        for i in 0..hauteur {
            for j in (0..largeur).rev() {
                match tableau[i][j] {
                    b'>' => {
                        if tableau[i][(j + 1) % largeur] == b'.' {
                            working_array[i][(j + 1) % largeur] = b'>';
                            working_array[i][j] = b'.';
                            deplacement = true;
                        } else {
                            working_array[i][j] = b'>';
                        }
                    }
                    c => {
                        if working_array[i][j] == 0 {
                            working_array[i][j] = c
                        }
                    }
                }
            }
        }
        mem::swap(&mut tableau, &mut working_array);
        working_array = vec![vec![0; tableau[0].len()]; tableau.len()];

        for i in (0..hauteur).rev() {
            for j in 0..largeur {
                match tableau[i][j] {
                    b'v' => {
                        if tableau[(i + 1) % hauteur][j] == b'.' {
                            working_array[(i + 1) % hauteur][j] = b'v';
                            working_array[i][j] = b'.';
                            deplacement = true;
                        } else {
                            working_array[i][j] = b'v';
                        }
                    }
                    c => {
                        if working_array[i][j] == 0 {
                            working_array[i][j] = c
                        }
                    }
                }
            }
        }
        mem::swap(&mut tableau, &mut working_array);
        if !deplacement {
            result = runs;
            break;
        }
        //display(&tableau);
    }
    Ok(result)
}

/**
 * Part 2
 */
pub fn part2(_input: &str) -> Result<i64, Box<dyn Error>> {
    Ok(0)
}


#[allow(dead_code)]
fn display(concombres_de_mer: &Vec<Vec<u8>>) {
    for i in 0..concombres_de_mer.len() {
        for j in 0..concombres_de_mer[0].len() {
            print!("{}", ((concombres_de_mer[i][j]) as u8) as char);
        }
        println!();
    }
    println!("==========");
}

#[cfg(test)]
mod tests {
    use super::*;

    const ÉNONCÉ: &str = "\
v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>";

    #[test]
    fn test_part1() {
        assert_eq!(part1(ÉNONCÉ).unwrap(), 58);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(ÉNONCÉ).unwrap(), 0);
    }

    #[test]
    fn real_tests() {
        // avec les vraies données
        assert_eq!(part1(include_str!("../inputs/input25.txt")).unwrap(), 513);
        assert_eq!(part2(include_str!("../inputs/input25.txt")).unwrap(), 0);
    }
}
