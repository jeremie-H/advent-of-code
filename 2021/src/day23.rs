use std::error::Error;

/**
 * Part 1
 */
pub fn part1(input: &str) -> Result<i64, Box<dyn Error>> {
    load_input(input);
    let result = 0;
    Ok(result)
}

/**
 * Part 2
 */
pub fn part2(input: &str) -> Result<i64, Box<dyn Error>> {
    load_input(input);
    let result = 0;
    Ok(result)
}


/**
 * méthode de lecture du fichier d'input et retourne un amphipod
 */
fn load_input(input: &str) -> Vec<Vec<u8>> {
    let mut result = Vec::new();
    let (mut room_a, mut room_b, mut room_c, mut room_d) = (Vec::new(), Vec::new(), Vec::new(), Vec::new());
    input
        .lines()
        .skip(2)
        //.inspect(|f| println!("{}",f))
        .for_each(| l|{
            println!("{}",l);
            let ligne  = l.chars()
                .filter(char::is_ascii_uppercase)
                .map(|c| c as u8 - b'A'+1)
                .collect::<Vec<u8>>();
            if ligne.len() == 4 {
                room_a.push(ligne[0]);
                room_b.push(ligne[1]);
                room_c.push(ligne[2]);
                room_d.push(ligne[3]);
            }
        });
        result.push(room_a);
        result.push(room_b);
        result.push(room_c);
        result.push(room_d);
        result
}

#[cfg(test)]
mod tests {
    use super::*;

    const ÉNONCÉ: &str = "\
#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########";

    #[test]
    fn test_part1() {
        assert_eq!(part1(ÉNONCÉ).unwrap(), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(ÉNONCÉ).unwrap(), 0);
    }

    #[test]
    fn real_tests() {
        // avec les vraies données
        assert_eq!(part1(include_str!("../inputs/input23.txt")).unwrap(), 0);
        assert_eq!(part2(include_str!("../inputs/input23.txt")).unwrap(), 0);
    }
}
