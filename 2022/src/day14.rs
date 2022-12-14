use std::{error::Error, ops::Range};
use std::io::{stdout, Stdout, Write};
/**
 * Part 1
 */
pub fn part1(input: &str) -> Result<i64, Box<dyn Error>> {
    let mut stdout = stdout();
    let (mut map, bottom) = load_map(&input);
    let range_vizu = 460..540;
    let result = falling_sand(&mut map, bottom, false, &range_vizu, &mut stdout);
    //display_cave(&map, bottom, &range_vizu, &mut stdout);
    Ok(result)
}


/**
 * Part 2 - définir le range_vizu ci-dessous, c'est juste pour l'affichage
 * 485..515 pour la démo exemple
 * 330..670 pour valeur réelle
 */
pub fn part2(input: &str) -> Result<i64, Box<dyn Error>> {
    let mut stdout = stdout();
    let (mut map, bottom) = load_map(&input);
    let bottom = insert_floor(&mut map, bottom);
    let range_vizu = 330..670;
    let result = falling_sand(&mut map, bottom, true, &range_vizu, &mut stdout);
    //display_cave(&map, bottom, &range_vizu, &mut stdout);
    Ok(result)
}


fn insert_floor(map: &mut [Vec<u8>], bottom: usize) -> usize {
    for j in 0..map[0].len(){
        map[bottom+1][j] = b'#';
    }
    bottom+1
}

fn falling_sand(map: &mut [Vec<u8>], bottom: usize, with_floor: bool, _range: &Range<usize>, _stdout: &mut Stdout) -> i64 {
    let mut units = 0;
    let mut leave_me_out = false;
    while !leave_me_out {
        let mut y = 0;
        let mut x = 500;
        //display_cave(map, bottom, _range, _stdout);
        loop {
            if y > bottom && !with_floor {leave_me_out=true;break;}
            if with_floor && map[0][500]!= b'+' {leave_me_out=true;break;}
            if map[y+1][x] == b'.' {y += 1; continue;}
            else if map[y+1][x-1] == b'.' {y += 1; x-=1; continue;}
            else if map[y+1][x+1] == b'.' {y += 1; x+=1; continue;}
            else {
                map[y][x] = b'o';
                break;
            }
        }
        units += 1;
    }
    units-1
}


fn load_map(input: &str) -> (Vec<Vec<u8>>, usize) {
    let mut map = vec![vec![b'.';1000];170];
    let max = input.lines()
    .map(|l|draw_rocks(&l.split(" -> ").collect::<Vec<&str>>(), &mut map))
    .max().unwrap();
    (map, max)
    
}

/**
 * return the coordinate of max y for this draw
 */
fn draw_rocks(split: &[&str], map: &mut [Vec<u8>]) -> usize {
    let mut last_point = (0,0);
    let mut maxy=0;
    map[0][500]=b'+';
    for s in split {
        let (x,y) = s.split_once(',').unwrap();
        let (x,y) = (x.parse::<usize>().unwrap(),y.parse::<usize>().unwrap());
        maxy = y.max(maxy);
        if last_point == (0,0) {
            last_point = (x,y);
            continue;
        }
        let rangex = if last_point.0 < x { last_point.0..=x }
        else { x..=last_point.0 };
        for i in rangex {
            map[y][i] = b'#';
        }

        let rangey = if last_point.1 < y { last_point.1..=y }
        else { y..=last_point.1 };
        for j in rangey {
            map[j][x] = b'#';
        }
        
        last_point = (x,y);
    }
    maxy+1
}

#[allow(dead_code)]
fn display_cave(map: &[Vec<u8>], bottom: usize, range: &Range<usize>, stdout: &mut Stdout) {
    let mut buffer: Vec<u8> = Vec::new();
    buffer.append(&mut "\x1b[1J\x1b[A".as_bytes().to_vec());
    for i in 0..bottom+1 {
        for j in range.clone() {
            let c = if map[i][j] == b'#' {"🧱"} else if map[i][j] == b'.' {"  "} else if map[i][j] == b'o' {"🟡"} else {""};
            buffer.append(&mut c.as_bytes().to_vec());
        }
        buffer.push(b'\n');
    };
    if let Ok(_r) = stdout.write_all(&buffer){
        stdout.flush().unwrap();
    }
    //thread::sleep(Duration::from_millis(50));
}


#[cfg(test)]
mod tests {
    use super::*;

    const ÉNONCÉ: &str = "498,4 -> 498,6 -> 496,6\n503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn test_part1() {
        assert_eq!(part1(ÉNONCÉ).unwrap(), 24);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(ÉNONCÉ).unwrap(), 93);
    }

    #[test]
    fn real_tests() {
        // avec les vraies données
        assert_eq!(part1(include_str!("../inputs/input14.txt")).unwrap(), 1003);
        assert_eq!(part2(include_str!("../inputs/input14.txt")).unwrap(), 25771);
    }
}
