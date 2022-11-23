use std::error::Error;

/**
 * Part 1
 */
pub fn part1(input: &str) -> Result<i64, Box<dyn Error>> {
    let (enhancement, mut image) = load_input(input);
    let mut infinite_value = 0usize;
    //display(&image, infinite_value);
    for _ in 0..2 {
        let (new_infinite_value, new_image) = enhance_image_once(&enhancement, &image, infinite_value);
        infinite_value = new_infinite_value;
        image = new_image;
    }
    //display(&image, infinite_value.try_into().unwrap());
    let sum = image
        .iter()
        .map(|row| row.iter().map(|value| *value as i64).sum::<i64>())
        .sum::<i64>();

    Ok(sum)
}

/**
 * Part 2
 */
pub fn part2(input: &str) -> Result<i64, Box<dyn Error>> {
    let (enhancement, mut image) = load_input(input);
    let mut infinite_value = 0usize;
    //display(&image, infinite_value);
    for _ in 0..50 {
        let (new_infinite_value, new_image) = enhance_image_once(&enhancement, &image, infinite_value);
        infinite_value = new_infinite_value;
        image = new_image;
    }
    //display(&image, infinite_value.try_into().unwrap());
    let sum = image
        .iter()
        .map(|row| row.iter().map(|value| *value as i64).sum::<i64>())
        .sum::<i64>();

    Ok(sum)
}


/**
 * on ajoute 1 rangée de pixel autour de l'image origine
 * cette rangée contient la valeur de infinite_value
 * puis on calcule la nouvelle valeur de infinite_value à renvoyer, car sur l'input de non-test,
 * on peut voir que l'infinie alterne entre '.' et '#'
 */
#[allow(clippy::needless_range_loop)]
fn enhance_image_once(enhancement: &[u8], image: &Vec<Vec<u8>>, infinite_value: usize) -> (usize, Vec<Vec<u8>>) {
    let mut image_output = vec![vec![0u8; image.len() + 2]; image.len() + 2]; //ajout de 2 valeurs d'infinie des deux côtés

    for i_out in 0..image.len() + 2 {
        for j_out in 0..image.len() + 2 {
            image_output[i_out][j_out] = calcul_pixel_sorti(i_out as i32, j_out as i32, enhancement, image, infinite_value);
        }
    }
    let new_infinite_value = match infinite_value {
        0 => enhancement[0] as usize,
        1 => enhancement[511] as usize,
        _ => usize::MAX,
    };
    (new_infinite_value, image_output)
}

/**
 * récupère les 9 pixels adjacents et fait le calcul pour récupérer la nouvelle valeur dans le tableau enhancement
 */
fn calcul_pixel_sorti(i_out: i32, j_out: i32, enhancement: &[u8], image: &Vec<Vec<u8>>, infinite_value: usize) -> u8 {
    let i_in = i_out - 1;
    let j_in = j_out - 1;

    let top_rowbit: usize = get_pixel(i_in - 1, j_in - 1, image, infinite_value) << 8
        | get_pixel(i_in - 1, j_in, image, infinite_value) << 7
        | get_pixel(i_in - 1, j_in + 1, image, infinite_value) << 6;

    let mid_rowbit: usize = get_pixel(i_in, j_in - 1, image, infinite_value) << 5
        | get_pixel(i_in, j_in, image, infinite_value) << 4
        | get_pixel(i_in, j_in + 1, image, infinite_value) << 3;

    let bottom_rowbit: usize = get_pixel(i_in + 1, j_in - 1, image, infinite_value) << 2
        | get_pixel(i_in + 1, j_in, image, infinite_value) << 1
        | get_pixel(i_in + 1, j_in + 1, image, infinite_value);

    let threerowbit = top_rowbit | mid_rowbit | bottom_rowbit;
    enhancement[threerowbit]
}

/**
 * retourne le pixel dans l'image de base, aux coordonnées (i,j)
 * les valeurs out of bound sont possibles, et renvoient dans ce cas : infinite_value
 */
fn get_pixel(i: i32, j: i32, image: &Vec<Vec<u8>>, infinite_value: usize) -> usize {
    if i < 0 || i >= (image.len() as i32) || j < 0 || j >= (image.len() as i32) {
        infinite_value
    } else {
        image[i as usize][j as usize] as usize
    }
}

/**
 * affiche simplement le tableau, avec 3 pixels de contours en valeurs infinies
 */
#[allow(dead_code)]
fn display(image: &Vec<Vec<u8>>, infinite: u8) {
    let inf = match infinite {
        0 => '.',
        1 => '#',
        _ => '?',
    };
    for _ in 0..3 {
        for _ in 0..(image.len() + 6) {
            print!("{}", inf);
        }
        println!();
    }

    for i in 0..image.len() {
        print!("{}", inf.to_string().repeat(3));
        for j in 0..image[0].len() {
            let c = match image[i][j] {
                0 => '.',
                1 => '#',
                _ => '?',
            };
            print!("{}", c);
        }
        println!("{}", inf.to_string().repeat(3));
    }

    for _ in 0..3 {
        for _ in 0..(image.len() + 6) {
            print!("{}", inf);
        }
        println!();
    }
    println!("==========");
}

/**
 * méthode de lecture du fichier d'input et retourne un tableau de structure Scanner
 * un scanner contient la position des balises dans son repère
 * et le numéro du scanner 0,1, ... 25
 */
fn load_input(input: &str) -> (Vec<u8>, Vec<Vec<u8>>) {
    let (enhancement, image) = input.split_once("\n\n").unwrap();

    let enhancement = enhancement
        .chars()
        .map(|c| match c {
            '#' => 1,
            '.' => 0,
            _ => u8::MAX,
        })
        .collect::<Vec<u8>>();
    let taille = image.split_once('\n').unwrap().0.len();
    let image = image.lines().fold(Vec::with_capacity(taille), |mut tableau, ligne| {
        tableau.push(
            ligne
                .chars()
                .map(|c| match c {
                    '#' => 1,
                    '.' => 0,
                    _ => u8::MAX,
                })
                .collect::<Vec<u8>>(),
        );
        tableau
    });
    (enhancement, image)
}

#[cfg(test)]
mod tests {
    use super::*;

    const ÉNONCÉ: &str = 
"..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###";

    #[test]
    fn test_part1() {
        assert_eq!(part1(ÉNONCÉ).unwrap(), 35);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(ÉNONCÉ).unwrap(), 3351);
    }

    #[test]
    fn real_tests() {
        // avec les vraies données
        assert_eq!(part1(include_str!("../inputs/input20.txt")).unwrap(), 5680);
        assert_eq!(part2(include_str!("../inputs/input20.txt")).unwrap(), 19766);
    }
}
