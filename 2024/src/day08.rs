use std::{collections::HashSet, error::Error};


fn extract_antennes(carte: Vec<&str>) -> Vec<(char, i32, i32)> {
    // Stockage des antennes
    let antennes: Vec<(char, i32, i32)> = carte
        .iter()
        .enumerate() // Parcourt les lignes avec leurs index
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate() // Parcourt les caractères de chaque ligne
                .filter_map(move |(col, ch)| {
                    if ch != '.' {
                        Some((ch, row as i32, col as i32)) // Conserve les antennes
                    } else {
                        None
                    }
                })
        })
        .collect();
    antennes
}

/**
 * Part 1
 */
pub fn part1(input: &str) -> Result<i64, Box<dyn Error>> {
    let carte: Vec<&str> = input.lines().collect();
    let rows = carte.len();
    let cols = carte[0].len();
    let antennes = extract_antennes(carte);
    let antinodes: HashSet<(i32, i32)> = antennes
        .iter()
        .enumerate()
        .flat_map(|(i, &(freq1, x1, y1))| {
            antennes
                .iter()
                .skip(i + 1) // on ignore les antennes présentes avant
                .filter(move |&&(freq2, _, _)| freq1 == freq2) // Même fréquence
                .flat_map(move |&(_, x2, y2)| {
                    let dx = x2 - x1;
                    let dy = y2 - y1;
                    vec![
                        (x1 - dx, y1 - dy), // Antinode 1
                        (x2 + dx, y2 + dy), // Antinode 2
                    ]
                })
        })
        .filter(|&(x, y)| x >= 0 && y >= 0 && (x as usize) < rows && (y as usize) < cols)
        .collect();
    Ok(antinodes.len() as i64)
}



/**
 * Part 2
 */
pub fn part2(input: &str) -> Result<i64, Box<dyn Error>> {
    let carte: Vec<&str> = input.lines().collect();
    let rows = carte.len();
    let cols = carte[0].len();
    let mut antinodes = HashSet::new();
    let antennes = extract_antennes(carte);
    antennes.iter()
    .enumerate()
    .for_each(|(i, &(f1, x1, y1))| {
        antennes.iter()
        .skip(i + 1)
        .filter(|&&(f2, _, _)| f1 == f2)
        .for_each(|&(_, x2, y2)| {
            let dx = x2 - x1;
            let dy = y2 - y1;

            // Générer les antinodes dans les deux directions
            generate_antinodes(x1, y1, dx, dy, rows as i32, cols as i32, &mut antinodes);
            generate_antinodes(x2, y2, -dx, -dy, rows as i32, cols as i32, &mut antinodes);
        });
    });

    Ok(antinodes.len() as i64)
}

fn generate_antinodes(mut x: i32, mut y: i32, dx: i32, dy: i32, rows: i32, cols: i32, antinodes: &mut HashSet<(i32, i32)>) {
    while x >= 0 && y >= 0 && x < rows && y < cols {
        antinodes.insert((x, y));
        x += dx;
        y += dy;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ÉNONCÉ: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn test_part1() {
        assert_eq!(part1(ÉNONCÉ).unwrap(), 14);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(ÉNONCÉ).unwrap(), 34);
    }

    #[test]
    fn real_tests() {
        // avec les vraies données
        assert_eq!(part1(include_str!("../inputs/input08.txt")).unwrap(), 390);
        assert_eq!(part2(include_str!("../inputs/input08.txt")).unwrap(), 1246);
    }
}
