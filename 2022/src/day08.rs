use std::error::Error;

/**
 * Part 1
 */
pub fn part1(input: &str) -> Result<i64, Box<dyn Error>> {
    let foret = input.lines()
        .map(|l| l.bytes().map(|c| c-b'0').collect::<Vec<u8>>())
        .collect::<Vec<Vec<u8>>>();
    // display_foret(&foret);
    let size = foret.len();
    let nb_edge_trees = (size*2+(size-2)*2) as i64;
    let mut visibles = vec![vec![0;size];size];
    
    let mut plushaut;
    for i in 1..size-1 {
        plushaut = foret[i][0];
        for j in 1..size-1 {
            if foret[i][j] > plushaut {
                visibles[i][j] = 1;
                plushaut = foret[i][j];
            }
        }
    };
    for i in 1..size-1 {
        plushaut = foret[0][i];
        for j in 1..size-1 {
            if foret[j][i] > plushaut {
                visibles[j][i] = 1;
                plushaut = foret[j][i];
            }
        }
    };
    for i in (1..size-1).rev() {
        plushaut = foret[i][size-1];
        for j in (1..size-1).rev() {
            if foret[i][j] > plushaut {
                visibles[i][j] = 1;
                plushaut = foret[i][j];
            }
        }
    };
    for i in (1..size-1).rev() {
        plushaut = foret[size-1][i];
        for j in (1..size-1).rev() {
            if foret[j][i] > plushaut {
                visibles[j][i] = 1;
                plushaut = foret[j][i];
            }
        }
    };
    let mut interieurs = 0;
    for i in 0..size {
        for j in 0..size {
            if visibles[i][j] > 0 {
                interieurs+=1;
            }
        }
    }
    // display_foret(&visibles);
    Ok(nb_edge_trees+interieurs)
}

/**
 * Part 2
 */
pub fn part2(input: &str) -> Result<i64, Box<dyn Error>> {
    let foret = input.lines()
    .map(|l| l.bytes().map(|c| c-b'0').collect::<Vec<u8>>())
    .collect::<Vec<Vec<u8>>>();
    let size = foret.len();
    
    let mut highest=0;
    for i in 1..size-1 {
        for j in 1..size-1 {
            let current = calcul_score(&foret, i, j, size);
            if current > highest {
                highest = current;
            }
            
        }
    };
    Ok(highest as i64)
}

fn calcul_score(foret: &Vec<Vec<u8>>, i: usize, j: usize, taille: usize) -> i64 {
    let mut score:i64 = 1;
    let mut trace:i64 = 0;
    for k in (0..i).rev() {
        trace+=1;
        if foret[k][j] >= foret[i][j]{ break; }
    }
    score *= trace;

    trace = 0;
    for k in i+1..taille {
        trace+=1;
        if foret[k][j] >= foret[i][j] { break; }
    }
    score *= trace;

    trace = 0;
    for m in (0..j).rev() {
        trace+=1;
        if foret[i][m] >= foret[i][j] { break; }
    }
    score *= trace;

    trace = 0;
    for m in j+1..taille {
        trace+=1;
        if foret[i][m] >= foret[i][j] { break; }
    }
    score *= trace;
    score
}

#[allow(dead_code)]
fn display_foret(visibles: &Vec<Vec<u8>>) {
    for i in 0..visibles.len() {
        for j in 0..visibles[0].len() {
            print!("{}",(visibles[i][j]+b'0') as char);
        }
        println!();
    };
}


#[cfg(test)]
mod tests {
    use super::*;

    const ÉNONCÉ: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn test_part1() {
        assert_eq!(part1(ÉNONCÉ).unwrap(), 21);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(ÉNONCÉ).unwrap(), 8);
    }

    #[test]
    fn real_tests() {
        // avec les vraies données
        assert_eq!(part1(include_str!("../inputs/input08.txt")).unwrap(), 1798);
        assert_eq!(part2(include_str!("../inputs/input08.txt")).unwrap(), 259308);
    }
}
