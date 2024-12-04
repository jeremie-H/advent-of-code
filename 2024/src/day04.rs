use std::error::Error;

const WORD: &str = "XMAS";

const DIRECTIONS: [(isize, isize); 8] = [
        (1, 1), // diagonale 1
        (-1, -1), // diagonale 1
        (1, -1), // diagonale 2
        (-1, 1), // diagonale 2
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
    ];


/**
 * Part 1
 */
pub fn part1(input: &str) -> Result<i64, Box<dyn Error>> {
    let tableau = input.lines()
        .map(|l| l.bytes().collect::<Vec<u8>>())
        .collect::<Vec<Vec<u8>>>();
    let mut count = 0;
    for row in 0..tableau.len() {
        for col in 0..tableau[row].len() {
            for &(dx, dy) in &DIRECTIONS {
                if matches_xmas(&tableau, WORD, row as isize, col as isize, dx, dy) {
                    count += 1;
                }
            }
        }
    }
    Ok(count)
}


fn matches_xmas(tableau: &Vec<Vec<u8>>, word: &str, start_row: isize, start_col: isize, dx: isize, dy: isize,) -> bool {
    word.bytes()
    .enumerate()
    .all(|(i, c)| {
        let x = start_row + i as isize * dx;
        let y = start_col + i as isize * dy;
        x >= 0 && y >= 0
        && x < tableau.len() as isize && y < tableau[0].len() as isize
        && tableau[x as usize][y as usize] == c
    })
}

/**
 * Part 2
 */
pub fn part2(input: &str) -> Result<i64, Box<dyn Error>> {
    let tableau = input.lines()
        .map(|l| l.bytes().collect::<Vec<u8>>())
        .collect::<Vec<Vec<u8>>>();
    let mut count = 0;
    for row in 0..tableau.len() {
        for col in 0..tableau[row].len() {
            if tableau[row][col] == b'A' {
                if matches_x_mas(&tableau, row as isize, col as isize) {
                    count += 1;
                }
            }
        }
    }
    Ok(count)
}


fn matches_x_mas(
    tableau: &Vec<Vec<u8>>,
    start_row: isize,
    start_col: isize
) -> bool {
    
    [0,2].iter().all(|i|{
        let xa = start_row + DIRECTIONS[0+i].0;
        let ya = start_col + DIRECTIONS[0+i].1;
        let xb = start_row + DIRECTIONS[1+i].0;
        let yb = start_col + DIRECTIONS[1+i].1;
        
        xa >= 0 && ya >= 0 && xb >= 0 && yb >= 0
        && xa < tableau.len() as isize && ya < tableau[0].len() as isize && xb < tableau.len() as isize && yb < tableau[0].len() as isize
        && match tableau[xa as usize][ya as usize] {
            b'M' => { tableau[xb as usize][yb as usize] == b'S' }
            b'S' => { tableau[xb as usize][yb as usize] == b'M' }
            _ => false
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const ÉNONCÉ: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn test_part1() {
        assert_eq!(part1(ÉNONCÉ).unwrap(), 18);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(ÉNONCÉ).unwrap(), 9);
    }

    #[test]
    fn real_tests() {
        // avec les vraies données
        assert_eq!(part1(include_str!("../inputs/input04.txt")).unwrap(), 2464);
        assert_eq!(part2(include_str!("../inputs/input04.txt")).unwrap(), 1982);
    }
}
