use std::{
    collections::{BTreeMap, HashSet},
    error::Error,
};
/**
 * Part 1
 */
pub fn part1(input: &str) -> Result<i64, Box<dyn Error>> {
    let liste_chemins = extract_paths(input);

    let mut chemins = HashSet::new();
    let mut chemin = Vec::with_capacity(15);
    chemin.push(0); //start
    follow_rabbit(&liste_chemins, 0, &mut chemin, &mut chemins, u8::MAX);
    //println!("chemins:{:?}",chemins);
    Ok(chemins.len() as i64)
}

/**
 * Part 2
 */
pub fn part2(input: &str) -> Result<i64, Box<dyn Error>> {
    let liste_chemins = extract_paths(input);
    println!("extract paths");
    let mut chemins = HashSet::new();
    let mut chemin = Vec::with_capacity(15);
    chemin.push(0); //start
    println!("extract paths 2");
    liste_chemins.keys().filter(|ch| **ch > 1).for_each(|key| {
        follow_rabbit(&liste_chemins, 0, &mut chemin, &mut chemins, *key);
    });

    // for ele in chemins {
    //     println!("{:?}",ele);
    // }
    Ok(chemins.len() as i64)
}

fn extract_paths(input: &str) -> BTreeMap<u8, Vec<u8>> {
    let liste_chemins = input.lines().fold(BTreeMap::new(), |mut tabl: BTreeMap<u8, Vec<u8>>, ligne| {
        let (source, dest) = ligne.split_once('-').map(|(f, g)| (convert_to_u8(f), convert_to_u8(g))).unwrap();

        match tabl.get_mut(&source) {
            Some(val) => {
                val.push(dest);
            }
            None => {
                tabl.insert(source, vec![dest]);
            }
        };
        match tabl.get_mut(&dest) {
            Some(val) => {
                val.push(source);
            }
            None => {
                tabl.insert(dest, vec![source]);
            }
        };
        tabl
    });
    liste_chemins
}

/**
 * fonction moche qui convertit les chaines de caractères 'iu' en u8
 * si majuscule, alors on set le premier bit des u8
 * coup de bol, il n'y a pas de collisions dans mes inputs, mais ça pourrait
 * la flemme de faire plus propre -_-"
 */
fn convert_to_u8(chaine: &str) -> u8 {
    match chaine {
        "start" => 0,
        "end" => 1,
        w => {
            let majuscule = if w.chars().next().unwrap().is_lowercase() {
                0b0
            } else {
                0b1000_0000
            };
            let value: u8 = w.as_bytes().iter().sum::<u8>() % 0b0111_1111 as u8;
            //println!("mapping {} => {}", w, value | majuscule);
            value | majuscule
            //*mapping.entry(w).or_insert(value & majuscule)
        }
    }
}

fn follow_rabbit(map_paths: &BTreeMap<u8, Vec<u8>>, node: u8, chemin: &mut Vec<u8>, chemins: &mut HashSet<Vec<u8>>, node_twice: u8) {
    match map_paths.get(&node) {
        Some(tab) /*if tab.len() > 1 */ => {
            for i in 0..tab.len() {
                if tab[i] == 1 {chemins.insert(chemin.to_vec()); continue;}

                let lowercase = tab[i] & 0b1000_0000 == 0;
                if  (lowercase && !chemin.contains(&tab[i]) && tab[i] != node_twice) ||
                    !lowercase {
                    //let mut newchemin = chemin.clone();
                    chemin.push(tab[i]);
                    follow_rabbit(map_paths, tab[i], chemin, chemins, node_twice);
                    chemin.remove(chemin.len()-1);
                }
                if lowercase && tab[i] == node_twice &&
                    chemin.iter().filter(|f| &tab[i] == *f).count() < 2 {

                    chemin.push(tab[i]);
                    follow_rabbit(map_paths, tab[i], chemin, chemins, node_twice);
                    chemin.remove(chemin.len()-1);
                }
            }
        },
        None => panic!("non prévu")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const  ÉNONCÉ: &str = "fs-end\nhe-DX\nfs-he\nstart-DX\npj-DX\nend-zg\nzg-sl\nzg-pj\npj-he\nRW-he\nfs-DX\npj-RW\nzg-RW\nstart-pj\nhe-WI\nzg-he\npj-fs\nstart-RW";

    #[test]
    fn test_part1() {
        assert_eq!(part1("start-A\nstart-b\nA-c\nA-b\nb-d\nA-end\nb-end").unwrap(), 10);
        assert_eq!(
            part1("dc-end\nHN-start\nstart-kj\ndc-start\ndc-HN\nLN-dc\nHN-end\nkj-sa\nkj-HN\nkj-dc").unwrap(),
            19
        );
        assert_eq!(part1(ÉNONCÉ).unwrap(), 226);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(ÉNONCÉ).unwrap(), 3509);
    }

    #[test]
    fn real_tests() {
        // avec les vraies données
        assert_eq!(part1(include_str!("../inputs/input12.txt")).unwrap(), 3738);
        assert_eq!(part2(include_str!("../inputs/input12.txt")).unwrap(), 120506);
    }
}
