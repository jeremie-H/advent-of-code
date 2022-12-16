use std::{error::Error, ops::Range, fmt::{Debug,Display, Formatter, self}};
use itertools::*;

/**
 * Part 1
 * la corde en géométrie est la droite qui coupe un cercle et wikipedia permet de trouver une formule ici https://fr.wikipedia.org/wiki/Segment_circulaire
 * on peut y trouver notamment que la corde = 2 rac(R²-d²)
 * ça ne fonctionne pas ici, car on est en distance de manhattan, mais ça donne un indice sur la façon de faire le calcul
 * si on prend distance de manhattan et qu'on retranche la distance du centre du cercle à la row souhaitée (10 dans l'exemple)
 * on trouve le nombre de cellule à remplir à gauche ou à droite
 * ne reste plus qu'à gérer les superpositions, prendre donc le min à gauche, et max à droite
 * ex: Sensor at x=8, y=7: closest beacon is at x=2, y=10, la distance de manhattan vaut 9
 * S est à 3 pixel au dessus de la ligne 10, on a donc 9-3 = 6 à gauche et 6 à droite
 */
pub fn part1(input: &str) -> Result<i64, Box<dyn Error>> {
    let row = if input.lines().count()<15 { 10 } else { 2_000_000 };//si donnée essai, c'est 10, sinon 2_000_000
    let rangetotal = input.lines()
    .map(Sensor::from)
    .map(|c|c.corde(row))
    .fold((0,0), |acc, range| {
        (acc.0.min(range.start),
         acc.1.max(range.end))
    });
    Ok(rangetotal.1-rangetotal.0)
}


/**
 * Part 2
 * Okeyyy là c'est chaud ! bon pour tomber sur la balise toute seule, il faut remplir des conditions, à savoir, être entouré de 4 "sensors"
 * qui laissent une zone(ligne) d'un "pixel" entre 2 sensors qui se font face, et 2 autres sensors dont la ligne vide mitoyenne est orthogonale à la première
 * l'intersection des 2 lignes vides doit permettre de trouver la balise toute seule
 * exemple ci dessous A et B se font face séparé par une ligne de vide
 * C et D (non représenté par souci de clarté) se font face, l'intersection des 2 lignes de vide laisse un unique point vide, celui recherché
 * lignes de O à O et de © à ©
..........#.................
.........###................
........#####...............
.......#######..............
......#########.............
.....###########............
....#############...........
...###############..........
..#################.........
.#########A#########O.......
..#################.#.......
...##########©####.###......
.....########█###.#####.....
.....#######███#.#######....
......#####█████####B####....
.......###███C███©######....
........###█████..#####....
.........###███....###......
..........#O.█......#.......
............................

 */
pub fn part2(input: &str) -> Result<i64, Box<dyn Error>> {
    let sensors = input.lines().map(Sensor::from)
    .collect::<Vec<Sensor>>();
    let result = sensors.iter().combinations(4)
    //on cherche 4 candidats dans toutes les combinaisons qui se font face par pairs et distant de 2 (un vide de 1 pixel existe entre eux)
    .filter(|four_sensor| (four_sensor[0].distant_de_2(&four_sensor[1]) && four_sensor[2].distant_de_2(&four_sensor[3])) || four_sensor[0].distant_de_2(&four_sensor[2]) && four_sensor[1].distant_de_2(&four_sensor[3]))
    //on range par pair, 0,1 sont face à face, et 2,3 également
    .map(|mut four_sensors| { if four_sensors[0].distant_de_2(&four_sensors[2]) {four_sensors.swap(1,2)} four_sensors})
    //vasy on débug, c'est le bordel
    //.inspect(|mut v|println!("4 Sensors matches : {:?}, intersection : {:?}", v, intersection(&mut v)))
    //on cherche l'intersection des lignes de vide et on serre les miches très fort pour que ça fonctionne
    .map(|mut v|intersection(&mut v))
    //sur le jeu d'essai, des cas de non croisements apparaissent
    .find(|coord| *coord != (-1,-1)).unwrap();
    let result = result.0*4_000_000+result.1;
    println!("result : {:?}",result);
    Ok(result)
}

//find here : https://stackoverflow.com/a/9997374
fn ccw(a: (i64,i64), b: (i64,i64), c: (i64,i64)) -> bool {
    return (c.1-a.1) * (b.0-a.0) > (b.1-a.1) * (c.0-a.0)
}

// Return true if line segments ab and cd intersect
fn is_segment_intersect(a: (i64,i64), b: (i64,i64), c: (i64,i64), d: (i64,i64)) -> bool { 
    return ccw(a,c,d) != ccw(b,c,d) && ccw(a,b,c) != ccw(a,b,d)
}


/**
 * supposed that v is ordonned by pair facing
 * juste calculate the intersection point between the two "void lines"
 */
fn intersection(v: &mut Vec<&Sensor>) -> (i64,i64) {
    let audessus = if v[0].scanner.1 > v[1].scanner.1 {v[1]} else {v[0]};
    let audessous = if v[0].scanner.1 < v[1].scanner.1 {v[1]} else {v[0]};

    //line 1 : between v[0] && v[1]
    let p1x = if audessus.scanner.0 < audessous.scanner.0 { audessus.scanner.0+audessus.rayon+1 }
    else { audessus.scanner.0-audessus.rayon-1 };
    let p1y = audessus.scanner.1;
    let p2x = audessus.scanner.0;
    let p2y = audessus.scanner.1+audessus.rayon+1;

    let audessus = if v[2].scanner.1 > v[3].scanner.1 {v[3]} else {v[2]};
    let audessous = if v[2].scanner.1 < v[3].scanner.1 {v[3]} else {v[2]};
    //line 2 : between v[2] && v[3]
    let p3x = if audessus.scanner.0 < audessous.scanner.0 { audessus.scanner.0+audessus.rayon+1 }
            else { audessus.scanner.0-audessus.rayon-1 };
    let p3y = audessus.scanner.1;
    let p4x = audessus.scanner.0;
    let p4y = audessus.scanner.1+audessus.rayon+1;
    if !is_segment_intersect((p1x, p1y), (p2x, p2y), (p3x, p3y), (p4x, p4y)) {return (-1,-1)};//filtre les segments qui ne se coupent pas
    compute_intersection(p1x, p1y, p2x, p2y, p3x, p3y, p4x, p4y)
   
}

fn compute_intersection(p1x: i64, p1y: i64, p2x: i64, p2y: i64, p3x: i64, p3y: i64, p4x: i64, p4y: i64) -> (i64,i64) {
    let c2x = p3x - p4x;
    let c3x = p1x - p2x;
    let c2y = p3y - p4y;
    let c3y = p1y - p2y;
    let d  = c3x * c2y - c3y * c2x;

    //this test check only for line intersection, not segments, that's why we've got the intersect method
    if d == 0 {
      return (-1,-1);
    }

    // upper part of intersection point formula
    let u1 = p1x * p2y - p1y * p2x;
    let u4 = p3x * p4y - p3y * p4x;

    // intersection point formula
    let px = (u1 * c2x - c3x * u4) / d;
    let py = (u1 * c2y - c3y * u4) / d;
    
    (px,py)
}


#[derive(PartialEq,Eq, Clone, Copy)]
struct Sensor {
    scanner: (i64,i64),//scanner is center of the circle/square
    beacon: (i64,i64),
    rayon: i64
}

impl Sensor {
    fn from(line: &str) -> Self {
        let parsing = line
        .split_ascii_whitespace()
        .map(|s| s.replace("x=", "").replace(',', "").replace("y=", "").replace(':', ""))
        .filter_map(|s| s.parse::<i64>().ok())
        .collect::<Vec<i64>>();
        let mut s = Self { scanner: (parsing[0],parsing[1]), beacon: (parsing[2],parsing[3]), rayon: 0};
        s.rayon = s.manhattan();
        s
    }

    fn manhattan(&self) -> i64 {
        (self.scanner.0-self.beacon.0).abs() + (self.scanner.1-self.beacon.1).abs()
    }

    fn corde(&self, row: i64) -> Range<i64>  {
        let manhattan = self.rayon;
        let reste = manhattan - (self.scanner.1 - row).abs();
        if reste > 0 { self.scanner.0-reste..self.scanner.0+reste }
        else {0..0}
    }

    //si on calcule la distance manhattan entre les 2 centres de sensors, et qu'on retire chaque rayon, le reste doit être la zone entre les 2
    fn distant_de_2(&self, second: &Sensor) -> bool {
        (self.scanner.0-second.scanner.0).abs() + (self.scanner.1-second.scanner.1).abs() - (self.rayon + second.rayon) == 2
    }
}

//only for debugging
impl Display for Sensor {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})({},{}):{}", self.scanner.0, self.scanner.1, self.beacon.0, self.beacon.1,self.rayon)
    }
}
//only for debugging
impl Debug for Sensor {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})({},{}):{}", self.scanner.0, self.scanner.1, self.beacon.0, self.beacon.1,self.rayon)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ÉNONCÉ: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn test_part1() {
        assert_eq!(part1(ÉNONCÉ).unwrap(), 26);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(ÉNONCÉ).unwrap(), 56000011);
    }

    #[test]
    fn real_tests() {
        // avec les vraies données
        assert_eq!(part1(include_str!("../inputs/input15.txt")).unwrap(), 5878678);
        assert_eq!(part2(include_str!("../inputs/input15.txt")).unwrap(), 11796491041245);
    }
}
