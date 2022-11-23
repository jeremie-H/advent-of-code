use std::{
    collections::{BinaryHeap, HashSet},
    error::Error,
    hash::Hash,
};
use colored::*;
/**
 * Part 1
 */
pub fn part1(input: &str) -> Result<i64, Box<dyn Error>> {
    let map = load_map(input);
    //display(&map);
    let goal: Pos = Pos(map[0].len() - 1, map.len() - 1);

    let result = dijkstra(map.len(), map[0].len(), &Pos(0, 0), |p| p.prochaines_cases(&map), |p| *p == goal).unwrap();
    // let result = dijkstra(&Pos(0, 0), |p| p.successors(&map),|p| *p == goal).unwrap();
    display_chemin(&map, &result.0);

    Ok(result.1 as i64)
}

/**
 * Part 2
 */
pub fn part2(input: &str) -> Result<i64, Box<dyn Error>> {
    let map = load_map(input);
    let goal: Pos = Pos((map[0].len() * 5) - 1, (map.len() * 5) - 1);

    let result = dijkstra(
        map.len() * 5,
        map[0].len() * 5,
        &Pos(0, 0),
        |p| p.prochaines_cases_cinq(&map),
        |p| *p == goal,
    )
    .unwrap();
    //display_chemin_five(&map, &result.0);

    Ok(result.1 as i64)
}


fn dijkstra<ClosureNext, ClosureIter, ClosureSuccess>(
    hauteur: usize,
    largeur: usize,
    start: &Pos,
    mut successors: ClosureNext,
    mut is_ended: ClosureSuccess,
) -> Option<(Vec<Pos>, i32)>
where
    ClosureNext: FnMut(&Pos) -> ClosureIter,
    ClosureIter: IntoIterator<Item = (Pos, i32)>,
    ClosureSuccess: FnMut(&Pos) -> bool, {
    let mut cout_noeud = vec![vec![i32::MAX; largeur]; hauteur];
    let mut visited = HashSet::new();
    cout_noeud[0][0] = 0;
    let mut open_list = BinaryHeap::new();

    open_list.push(State {
        cost: i32::MAX,
        position: *start,
    });
    visited.insert(*start);

    while let Some(State { cost, position }) = open_list.pop() {
        let cost = i32::MAX - cost;

        if is_ended(&position) {
            // uniquement pour ressortir la liste d'affichage
            let mut last: i32 = i32::MAX;
            let (mut i, mut j): (i32, i32) = (largeur as i32 - 1, largeur as i32 - 1);
            while i > 0 && j > 0 {
                let direc: [(i32, i32); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];
                for dir in direc.iter() {
                    if i + dir.0 >= largeur as i32 || j + dir.1 >= largeur as i32 {
                        continue;
                    }
                    if last > cout_noeud[(i + dir.0) as usize][(j + dir.1) as usize] {
                        last = cout_noeud[(i + dir.0) as usize][(j + dir.1) as usize];
                        i += dir.0;
                        j += dir.1;
                        visited.insert(Pos(j as usize, i as usize));
                    }
                }
            }
            return Some((visited.iter().copied().collect::<Vec<Pos>>(), cost));
        }

        for (v_position, cost_v) in successors(&position) {
            if cout_noeud[v_position.1][v_position.0] > cost + cost_v {
                let cost = cost + cost_v;
                cout_noeud[v_position.1][v_position.0] = cost;
                let next = State {
                    cost: i32::MAX - cost,
                    position: v_position,
                };
                open_list.push(next);
            }
        }
    }
    None
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct State {
    cost: i32,
    position: Pos,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Copy)]
struct Pos(usize, usize);

impl Pos {
    fn prochaines_cases(&self, map: &[Vec<i32>]) -> Vec<(Pos, i32)> {
        let &Pos(x, y) = self;
        let mut result = Vec::with_capacity(4);

        if x < map[0].len() - 1 {
            result.push((Pos(x + 1, y), map[y][x + 1]));
        }
        if y < map.len() - 1 {
            result.push((Pos(x, y + 1), map[y + 1][x]));
        }
        if x > 0 {
            result.push((Pos(x - 1, y), map[y][x - 1]));
        }
        if y > 0 {
            result.push((Pos(x, y - 1), map[y - 1][x]));
        }
        result
    }

    fn prochaines_cases_cinq(&self, map: &[Vec<i32>]) -> Vec<(Pos, i32)> {
        let &Pos(x, y) = self;
        let mut result = Vec::with_capacity(4);

        let largeur = map.len();
        let index_x = (x / largeur) as i32;
        let index_y = (y / largeur) as i32;
        
        let index_x_plus_un = ((x + 1) / largeur) as i32;
        let index_y_plus_un = ((y + 1) / largeur) as i32;

        if x < (largeur * 5) - 1 {
            result.push((
                Pos(x + 1, y),
                (map[y % largeur][(x + 1) % largeur] + index_x_plus_un + index_y - 1) % 9 + 1,
            ));
        }
        if y < (largeur * 5) - 1 {
            result.push((
                Pos(x, y + 1),
                (map[(y + 1) % largeur][x % largeur] + index_y_plus_un + index_x - 1) % 9 + 1,
            ));
        }
        if x > 0 {
            let index_x_moins_un = ((x - 1) / largeur) as i32;
            result.push((
                Pos(x - 1, y),
                (map[y % largeur][(x - 1) % largeur] + index_x_moins_un + index_y - 1) % 9 + 1,
            ));
        }
        if y > 0 {
            let index_y_moins_un = ((y - 1) / largeur) as i32;
            result.push((
                Pos(x, y - 1),
                (map[(y - 1) % largeur][x % largeur] + index_y_moins_un + index_x - 1) % 9 + 1,
            ));
        }
        result
    }
}

#[allow(dead_code)]
fn display(tab: &[Vec<i32>]) {
    for i in 0..tab.len() {
        for j in 0..tab[0].len() {
            print!("{}", tab[i][j]);
        }
        println!();
    }
    println!("==========");
}

fn load_map(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|l| l.bytes().map(|c| (c - b'0') as i32).collect())
        .collect()
}

#[allow(dead_code)]
fn display_chemin(tab: &[Vec<i32>], chemin: &[Pos]) {
    let mut aff = Vec::with_capacity(tab.len());
    for i in 0..tab.len() {
        aff.push(Vec::with_capacity(tab[0].len()));
        for j in 0..tab[0].len() {
            aff[i].push((tab[i][j], false));
        }
    }
    for ch in chemin {
        aff[ch.1][ch.0].1 = true;
    }
    for i in 0..aff.len() {
        for j in 0..aff[0].len() {
            if aff[i][j].1 {
                print!("{}", format!("{}", aff[i][j].0).cyan());
            } else {
                print!("{}", aff[i][j].0);
            }
        }
        println!();
    }
    println!("==========");
}

#[allow(dead_code)]
fn display_chemin_five(tab: &[Vec<i32>], chemin: &[Pos]) {
    let mut aff: Vec<Vec<(i32, bool)>> = Vec::with_capacity(tab.len() * 5);
    for i in 0..(tab.len() * 5) {
        aff.push(Vec::with_capacity(tab[0].len() * 5));
        for j in 0..(tab[0].len() * 5) {
            aff[i].push((((tab[i % 10][j % 10] + (i / 10) as i32 + (j / 10) as i32 - 1) % 9) + 1, false));
        }
    }
    for ch in chemin {
        aff[ch.1][ch.0].1 = true;
    }
    for i in 0..aff.len() {
        for j in 0..aff[0].len() {
            if aff[i][j].1 {
                print!("{}", format!("{}", aff[i][j].0).cyan());
            } else {
                print!("{}", aff[i][j].0);
            }
        }
        println!();
    }
    println!("==========");
}

#[cfg(test)]
mod tests {
    use super::*;

    const ÉNONCÉ: &str = "\
1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

    #[test]
    fn test_part1() {
        assert_eq!(part1(ÉNONCÉ).unwrap(), 40);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(ÉNONCÉ).unwrap(), 315);
    }

    #[test]
    fn real_tests() {
        // avec les vraies données
        assert_eq!(part1(include_str!("../inputs/input15.txt")).unwrap(), 388);
        assert_eq!(part2(include_str!("../inputs/input15.txt")).unwrap(), 2819);
    }
}
