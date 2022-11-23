use array2d::Array2D;
use std::error::Error;

/**
 * Part 1
 */
pub fn part1(input: &str) -> Result<i64, Box<dyn Error>> {
    let mut boards = load_board_bingo(&input);
    let score;
    for drawnumber in load_drawn_number(&input) {
        for board in boards.iter_mut() {
            if board.draw(drawnumber) {
                println!("au numéro {} nous avons un board gagnant  : {:?}", drawnumber, board);
                score = drawnumber * board.score_without_latest();
                return Ok(score as i64);
            }
        }
    }
    Ok(0)
}

/**
 * Part 2
 */
pub fn part2(input: &str) -> Result<i64, Box<dyn Error>> {
    let mut boards = load_board_bingo(&input);
    let score;
    for drawnumber in load_drawn_number(&input) {
        for board in boards.iter_mut() {
            if board.draw(drawnumber) && board.is_last {
                println!("au numéro {} nous avons un board gagnant  : {:?}", drawnumber, board);
                score = drawnumber * board.score_without_latest();
                return Ok(score as i64);
            }
        }

        let mut lesderniers: Vec<&mut Board> = boards.iter_mut().filter(|b| !b.is_winning()).collect::<Vec<&mut Board>>();

        if lesderniers.len() == 1 {
            lesderniers[0].set_last()
        }
    }
    Ok(0)
}

#[derive(Default, Copy, Clone, Eq, PartialEq, Debug)]
pub struct Cell {
    value: i32,
    drawn: bool,
}

impl Cell {
    pub fn new() -> Self { Self { value: 0, drawn: false } }
    pub fn from_val(val: i32) -> Self { Self { value: val, drawn: false } }
    fn drawn(&mut self) { self.drawn = true; }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Board {
    cells: Array2D<Cell>,
    pub is_last: bool,
}

impl Default for Board {
    fn default() -> Self { Self::new() }
}

impl Board {
    #[allow(unused)]
    pub fn new() -> Self {
        Self {
            cells: Array2D::filled_with(Cell::new(), 5, 5),
            is_last: false,
        }
    }

    pub fn set_last(&mut self) { self.is_last = true; }

    pub(crate) fn fill(&mut self, five_lines: Vec<i32>) {
        let cellules = five_lines.iter().map(|f| Cell::from_val(*f)).collect::<Vec<Cell>>();
        self.cells = Array2D::from_row_major(&cellules, 5, 5)
    }

    pub fn draw(&mut self, numero: i32) -> bool {
        for i in 0..5 {
            for j in 0..5 {
                if let Some(cellule) = self.cells.get_mut(i, j) {
                    if cellule.value == numero {
                        cellule.drawn()
                    }
                }
            }
        }
        self.is_winning()
    }

    pub fn is_winning(&self) -> bool {
        let check_by_rows = self.cells.rows_iter().any(|mut row| row.all(|cellule| cellule.drawn));
        let check_by_columns = self.cells.columns_iter().any(|mut column| column.all(|cellule| cellule.drawn));

        check_by_rows | check_by_columns
    }

    pub fn score_without_latest(&self) -> i32 {
        let score = self
            .cells
            .elements_row_major_iter()
            .filter(|cell| !cell.drawn)
            .map(|cell| cell.value)
            .sum();
        score
    }
}

/**
 * load drawn number for bingo
 */
pub fn load_drawn_number(input: &str) -> Vec<i32> {
    let drawn_numbers = input
        .split_once("\n\n")
        .unwrap()
        .0
        .split(',')
        //.inspect(|f| println!("{:?}",f))
        .map(|line| line.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    println!("drawns numbers : {:?}", drawn_numbers);
    drawn_numbers
}

/**
 *  load board for bingo
 */
pub fn load_board_bingo(input: &str) -> Vec<Board> {
    let (result, _, _) = input
        .split_once("\n\n")
        .unwrap()
        .1
        .split('\n')
        .filter(|ligne| !ligne.is_empty())
        .fold((Vec::new(), String::new(), 0), |(mut result, mut chaine, mut compteur), s_param| {
            if compteur > 4 {
                let mut current_board = Board::new();
                current_board.fill(chaine.split_whitespace().map(|i| i.parse().unwrap()).collect());
                result.push(current_board);
                chaine = String::new();
                compteur = 0;
            }

            (result, chaine + " " + s_param, compteur + 1)
        });

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const ÉNONCÉ: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
 ";

    #[test]
    fn test_part1() {
        assert_eq!(part1(ÉNONCÉ).unwrap(), 4512);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(ÉNONCÉ).unwrap(), 1924);
    }

    #[test]
    fn real_tests() {
        // avec les vraies données
        assert_eq!(part1(include_str!("../inputs/input04.txt")).unwrap(), 33348);
        assert_eq!(part2(include_str!("../inputs/input04.txt")).unwrap(), 8112);
        //on désactive ces tests car beaucoup trop longs
    }
}
