use std::{error::Error, collections::VecDeque};
use colored::*;

const DIR: [(i32,i32);4] = [(0,1),(1,0),(0,-1),(-1,0)];

/**
 * Part 1
 */
pub fn part1(input: &str) -> Result<i64, Box<dyn Error>> {
    let (mut map, arrivee) = load_map(input);
    //let map_copied = map.clone();//utilisé seulement pour le display
    let result = bfs(arrivee, &mut map, |height| *height==b'S');//on inverse le chemin, pour gérer la partie 2
    //display_chemin(&map_copied, &result.1);
    Ok(result.0.unwrap() as i64)
}

/**
 * Part 2
 */
pub fn part2(input: &str) -> Result<i64, Box<dyn Error>> {
    let (mut map, arrivee) = load_map(input);
    //let map_copied = map.clone();//utilisé seulement pour le display
    let result = bfs(arrivee, &mut map, |height| *height==b'a');//on part de la fin pour trouver le premier 'a'
    //display_chemin(&map_copied, &result.1);
    Ok(result.0.unwrap() as i64)
}

fn load_map(input: &str) -> (Vec<Vec<u8>>, (usize,usize)) {
    let mut depart = (0,0);
    (input.lines().enumerate().map(|(y, line)| {
        line.bytes().enumerate().map(|(x, c)| {
            if c == b'E' { depart = (y,x); }
            c
        }).collect()
    }).collect(), depart)
}

// bfs doit être suffisant, car il n'y a pas de poids sur les trajets, hormis la règle de pas monter d'un char +1
fn bfs<ClosureSuccess>(depart: (usize, usize), map: &mut [Vec<u8>], mut is_finish: ClosureSuccess) -> (Option<i64>, Vec<(usize,usize)>)
where ClosureSuccess: FnMut(&u8) -> bool
{
    map[depart.0][depart.1] = 0;//le départ est marqué visité
    let mut chemins = VecDeque::new();
    let mut increment = 0;
    // on pourrait accélerer le traitement en virant simplement le vec attaché à chaque position
    // ce vec permet juste de garder la trace du plus court chemin
    // c'est quand même plus sympa pour dessiner le chemin
    chemins.push_back((b'z', (depart.0, depart.1), vec![(depart.0, depart.1)]));
    while !chemins.is_empty() {
        for _ in 0..chemins.len() {
            let (height, (y, x),mut trackpath);
            
            let pop = chemins.pop_front();
            if pop == None { continue ;}
            else {
                (height, (y, x), trackpath) = pop.unwrap();
            }
            if is_finish(&height) {
                trackpath.push((y,x));
                return (Some(increment), trackpath);
            }
            for dir in DIR {
                let newpoint = (y as i32 + dir.0, x as i32 + dir.1);
                //limites de la map
                if newpoint.0 < 0 || newpoint.1 < 0 { continue; }
                let (yy,xx) = (newpoint.0 as usize, newpoint.1 as usize);
                if yy >= map.len() || xx >= map[0].len() { continue; }

                let mut next_height = map[yy][xx];
                if next_height == b'S' {
                    next_height = b'a'
                }
                // on sort si la prochaine height est déjà visité ou si elle dépasse d'un cran
                if next_height == 0 || next_height < height-1 { continue; }
                //le trackpath nous permet de conserver un tracé vers le chemin le plus court
                trackpath.push((y,x));
                chemins.push_back((map[yy][xx], (yy, xx),trackpath.clone()));
                
                // marqué comme visité
                map[yy][xx] = 0;
            }
        }
        increment += 1;
    }

    (None, Vec::new())
}


#[allow(dead_code)]
fn display_chemin(tab: &[Vec<u8>], chemin: &[(usize,usize)]) {
    let mut aff = Vec::with_capacity(tab.len());
    for i in 0..tab.len() {
        aff.push(Vec::with_capacity(tab[0].len()));
        for j in 0..tab[0].len() {
            aff[i].push((tab[i][j], false));
        }
    }
    for ch in chemin {
        aff[ch.0][ch.1].1 = true;
    }
    for i in 0..aff.len() {
        for j in 0..aff[0].len() {
            if aff[i][j].1 {
                print!("{}", format!("{}", aff[i][j].0 as char).cyan());
            } else {
                print!("{}", aff[i][j].0 as char);
            }
        }
        println!();
    }
    println!("==========");
}



#[cfg(test)]
mod tests {
    use super::*;

    const ÉNONCÉ: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn test_part1() {
        assert_eq!(part1(ÉNONCÉ).unwrap(), 31);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(ÉNONCÉ).unwrap(), 29);
    }

    #[test]
    fn real_tests() {
        // avec les vraies données
        assert_eq!(part1(include_str!("../inputs/input12.txt")).unwrap(), 370);
        assert_eq!(part2(include_str!("../inputs/input12.txt")).unwrap(), 363);
    }
}
