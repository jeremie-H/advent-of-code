use itertools::Itertools;
use std::{
    error::Error,
    collections::{HashMap, HashSet}
};

use crate::day19coord::{Scanner, XYZ};

/**
 * THOUGHS
 * un scanner possède une vue sur 25 balises, donc si on teste toutes les combinaisons de distance entre les balises d'un scanner,
 * exemple balise 0 : 0/1, 0/2, 0/3... 0/24
 * puis la balise 1 : 1/2, 1/3, ...1/24
 * on doit avoir 25+24+23+22....+3+2+1 = 25(25-1)/2 = 300 distances à calculer entre chaque balises.
 * 
 * si au moins 12 balises sont communes entre elles, on devrait avoir à minima 12+11+10...+3+2+1 = 12(12-1)/2 = 66 distances égales entre 2 scanners
 * ******************
 * 
 * ensuite, une fois 2 scanners ayant des balises communes, 
 * on doit pouvoir fixer le premier scanner comme repère 0,0,0 et calculer la position du scanner 2 en fonction
 * 
 * ***************
 * pour calculer les rotations / translations 
 * (0,1) =========> (6, [   68,-1246,  -43])
 * (1,4) =========> (10, [   88,  113,-1104])
 * 
 * ça devrait donner à minima : (XXX, [-20,-1133,1061])
 */
/**
 * Part 1
 */
pub fn part1(input: &str) -> Result<i64, Box<dyn Error>> {
    let scanners = load_scanners(input);

    let distances = calcul_distances_toutes_balises(&scanners);

    let matches = find_chevauchement_scanners(&distances);

    let rotation_et_positions_scanners = calcul_rotation_et_positions_scanners(&matches, &scanners, &distances);

    let rotation_et_translation = recalcul_positions_par_rapport_au_zero(scanners.len(), &matches, &rotation_et_positions_scanners);
    println!("position et orientation de tous les scanners {:?}", rotation_et_translation);

    let hashset_resultat = scanners.iter().enumerate().fold(HashSet::new(), |acc, (indice, scan)| {
        let hs = map_balises_dans_repere_zero(scan, rotation_et_translation[indice].0, rotation_et_translation[indice].1);
        acc.union(&hs).copied().collect()
    });

    Ok(hashset_resultat.len() as i64)
}

/**
 * Part 2
 */
pub fn part2(input: &str) -> Result<i64, Box<dyn Error>> {
    
    let scanners = load_scanners(input);

    let distances = calcul_distances_toutes_balises(&scanners);

    let matches = find_chevauchement_scanners(&distances);

    let rotation_et_positions_scanners = calcul_rotation_et_positions_scanners(&matches, &scanners, &distances);

    let juste_les_translations = recalcul_positions_par_rapport_au_zero(scanners.len(), &matches, &rotation_et_positions_scanners)
        .iter()
        .map(|(_rot, pos)| *pos)
        .collect::<Vec<XYZ>>();

    let resultat = juste_les_translations
        .iter()
        .tuple_combinations()
        .map(|(b1, b2)| b1.manhattan(b2))
        .max()
        .unwrap() as i64;

    Ok(resultat)
}


/**
 * méthode de lecture du fichier d'input et retourne un tableau de structure Scanner
 * un scanner contient la position des balises dans son repère
 * et le numéro du scanner 0,1, ... 25
 */
fn load_scanners(input: &str) -> Vec<Scanner> {
    let scanners = input
        .split("\n\n")
        .enumerate()
        .map(|(i, scanner)| {
            Scanner::new(
                i,
                scanner.lines().skip(1).fold(Vec::new(), |mut acc, ligne| {
                    let mut coord = ligne.split(',').map(|coordonée| coordonée.parse::<i32>().unwrap());
                    acc.push(XYZ(coord.next().unwrap(), coord.next().unwrap(), coord.next().unwrap()));
                    acc
                }),
            )
        })
        .collect::<Vec<Scanner>>();
    scanners
}

/**
 * prends la liste des scanners en paramètres
 * et retourne un tableau contenant les HashSet des distances entre toutes les balises d'un même scanner
 * exemple :
 * resultat[0] => HashSet(1234, 2001, 987, ...)
 * resultat[1] => HashSet(1544, 587, 1234, ...)
 * on peut déjà voir par exemple que l'on a une distance(1234) égale entre le scanner [0] et [1]
 */
fn calcul_distances_toutes_balises(scanners: &[Scanner]) -> Vec<HashSet<i32>> {
    scanners.iter().map(distances).collect::<Vec<HashSet<i32>>>()
}

/**
 * parcours de l'ensemble des distances par scanner, et cherche s'il y a au moins 66 correspondances entre 2 hashSet de distance
 * pourquoi 66 ? voir le fichier jereflexionne.txt
 *
 */
fn find_chevauchement_scanners(distances: &[HashSet<i32>]) -> Vec<(usize, usize)> {
    let matches = distances
        .iter()
        .enumerate()
        .tuple_combinations()
        //ici on test toutes les combinaisons de scanners entre eux, 0/1, 0/2, 0/3, 0/4, 1/2, 1/3, 1/4, 2/3, 2/4, 3/4 : soit 10 possibilités sur l'exemple
        //.inspect(|(f, g)| {println!("f: {:?} / g: {:?}", f.0, g.0);})
        .filter(|((_i, set_distances_a), (_j, set_distances_b))| set_distances_a.intersection(set_distances_b).count() >= 66)
        .flat_map(|((i, _), (j, _))| [(i, j), (j, i)])
        .collect::<Vec<(usize, usize)>>();
    println!("matches : {:?}", matches);
    matches
}

/**
 * calcul les rotations et positions des scanners en prenant comme repère toujours le premier match seulement
 * 2 sous fonctions possible, l'un trop lente est abandonnée (voir commentaire)
 */
fn calcul_rotation_et_positions_scanners(
    matches: &[(usize, usize)],
    scanners: &[Scanner],
    distances: &[HashSet<i32>],
) -> Vec<(usize, XYZ)> {
    // *** méthode mettant 700ms, puis 234ms après optim !! abandon 🙅‍♂️
    // let rotation_et_positions_scanners = matches.iter()
    // .map(|(index_a, index_b)| {
    //     find_rotation_and_translation(&scanners[*index_a], &scanners[*index_b])
    // })
    // .collect::<Vec<_>>();
    // println!("positions_scanners (test 1): {:?}", rotation_et_positions_scanners);

    // *** méthode plus sale, mais prenant ~ 3 ms !! 🎉
    let rotation_et_positions_scanners = matches
        .iter()
        .map(|(index_a, index_b)| {
            quick_find_rotation_and_translation(&scanners[*index_a], &scanners[*index_b], &distances[*index_a], &distances[*index_b])
        })
        .collect::<Vec<_>>();
    //println!("positions_scanners (test 2): {:?}", rotation_et_positions_scanners);
    rotation_et_positions_scanners
}

/**
 * recalcul les positions de toutes les balises d'un scanner en fonction de la rotation et translation appliquée
 */
fn map_balises_dans_repere_zero(scanner: &Scanner, rotation: usize, translation: XYZ) -> HashSet<XYZ> {
    scanner
        .pos
        .iter()
        .map(|b| b.rotation(rotation) + translation)
        .collect::<HashSet<XYZ>>()
}

/**
 * recalcul les positions des scanners et leur rotation relative pour en sortir un tableau des rotations et des positions
 * relative au scanner origine dont la rotation est 0 et la position est 0,0,0
 */
fn recalcul_positions_par_rapport_au_zero(
    nb_scanners: usize,
    matches: &[(usize, usize)],
    positions_scanners: &[(usize, XYZ)],
) -> Vec<(usize, XYZ)> {
    //hashmap : exemple, on a en matches [indice_0:(0,1) indice_1:(1,3), indice_2:(3,5), indice_3:(2,5)] on veut connaitre la transformation (0,5)
    // on mets en hashmap
    // 1 => (0,indice_1)
    // 3 => (1,indice_1)
    // 5 => (3,indice_2) (2, indice_3)
    let mut hashmap_matches = HashMap::new();
    matches.iter().enumerate().for_each(|(i, m)| {
        let entry = hashmap_matches.entry(m.1).or_insert(Vec::new());
        entry.push((m.0, i));
    });
    //println!("hashmap {:?}", hashmap_matches);

    let mut result = vec![(usize::MAX, XYZ(i32::MAX, i32::MAX, i32::MAX)); nb_scanners];
    result[0] = (0, XYZ(0, 0, 0));
    let mut liste_transfo = vec![0usize];
    while !liste_transfo.is_empty() {
        // println!("liste_transfo : {:?}",liste_transfo);
        let current = liste_transfo.pop().unwrap();

        let copie_filtrée = matches
            .iter()
            .enumerate()
            .filter(|(_j, (m1, m2))| *m1 == current && result[*m2].0 == usize::MAX)
            .map(|(j, (m1, m2))| (j, (*m1, *m2)))
            .collect::<Vec<(usize, (usize, usize))>>();

        copie_filtrée.iter().for_each(|(j, (m1, m2))| {
            // println!("applique ({}, {})", m1,m2);
            // println!("applique rot={}, trans={:?}",positions_scanners[*j].0, positions_scanners[*j].1);
            liste_transfo.push(*m2);
            //c'est tricky ici, dans le tableau de result, on avance petit à petit, sur chaque nouveau match, on va calculer quel est
            //la composition de rotation à appliquer, puis quelle est la composition de translation
            let composition_rotation = XYZ::composition_rotation(positions_scanners[*j].0, result[*m1].0);
            let composition_translation = result[*m1].1 + positions_scanners[*j].1.rotation(result[*m1].0);

            result[*m2] = (composition_rotation, composition_translation);
        });
        // println!("après result : {:?}",result);
        // println!("******************************");
    }

    result.to_vec()
}

/**
 * 🤮 méthode beaucoup trop lente ! abandon
 * après optimisation, elle prend toujours ~ 230ms
 */
#[allow(dead_code)]
fn find_rotation_and_translation(scanner_a: &Scanner, scanner_b: &Scanner) -> (usize, XYZ) {
    //piste d'optim : utiliser l'intersection
    //let intersec = dist_a.intersection(dist_b).copied().collect::<HashSet<i32>>();

    let debug = scanner_a.n == 0 && scanner_b.n == 1;

    let hash_set_a = HashSet::from_iter(scanner_a.pos.clone());

    let (mut rotation, mut translation): (usize, XYZ) = (0, XYZ(0, 0, 0));
    for n in 0..24 {
        let r = scanner_b
            .pos
            .iter()
            .enumerate()
            .take_while(|(i, _point)| i <= &14) // (on a 25 balises, il faut découvrir à minima 14 balises, au cas où les 12 balises ok sont les dernières)
            .find(|(_g, h)| {
                let point_b_rotationné = h.rotation(n);
                if debug {
                    println!("n = {}, rotation {:?} ==> {:?}", n, h, point_b_rotationné);
                }

                let deltas = scanner_a
                    .pos
                    .iter()
                    .enumerate()
                    .take_while(|(i, _j)| i <= &14) // on cherche sur les 14 premiers de A
                    .map(|(_h, point_dans_a)| *point_dans_a - point_b_rotationné) // calcul le delta
                    .collect::<Vec<XYZ>>();
                if debug {
                    println!("n = {}, les deltas {:?}", n, deltas);
                }

                //let tous_les_b_rot_translaté =
                let mut trouvé = false;
                for delta in deltas.iter() {
                    let touslesbrotatéettranslaté = scanner_b.pos.iter().map(|b| *delta + b.rotation(n)).collect::<HashSet<XYZ>>();
                    if hash_set_a.intersection(&touslesbrotatéettranslaté).count() >= 12 {
                        if debug {
                            touslesbrotatéettranslaté.iter().for_each(|b| println!("b {:?}", b));
                        }
                        //println!("[{},{}] rotation = {}, delta trouvé {:?}",scanner_a.n, scanner_b.n, n, delta);
                        translation = *delta;
                        rotation = n;
                        trouvé = true;
                        break;
                    }
                }
                trouvé
            });
        if r != None {
            break;
        }
    }
    (rotation, translation)
}

/**
 * 😎 méthode 100x plus efficace
 * mais il est nécessaire à plusieurs moment d'exclure les faux positifs remontés par les calculs de distances,
 * on cherche à identifier au moins 3 points (donc 2 distances) équivalent dans les scanners A et B
 * dès que l'on ne trouve pas un troisième point dans B par exemple, c'est qu'on a du tomber sur un faux positif de calcul de distance
 * on exclue de la recherche et on recommence donc
 * il arrive que les 3 points, 2 distances soient bien trouvés, mais après analyse, il s'avère que ce sont des mauvais points, et donc encore
 * des faux positifs !
 * un ultime test est de vérifier la présence des 12 balises identiques en utilisant la rotation et translation trouvée
 * sinon on exclue les points utilisés et on recommence tout encore
 */
fn quick_find_rotation_and_translation(
    scanner_a: &Scanner,
    scanner_b: &Scanner,
    dist_a: &HashSet<i32>,
    dist_b: &HashSet<i32>,
) -> (usize, XYZ) {
    // println!("[quick_find_rotation_and_translation] s_a {:?}", scanner_a);
    // println!("[quick_find_rotation_and_translation] s_b {:?}", scanner_b);
    let intersec = dist_a.intersection(dist_b).copied().collect::<HashSet<i32>>();
    let hashseta = HashSet::from_iter(scanner_a.pos.clone());
    let debug = false;
    let mut distance_balise_1_3_scanner_a: i32;
    let mut balise_1_scanner_a: XYZ;
    let mut balise_2_scanner_a: XYZ;
    let mut balise_3_scanner_a: XYZ;
    let mut balise_1_scanner_b: XYZ;
    let mut balise_2_scanner_b: XYZ;
    let mut balise_3_scanner_b: XYZ;
    let mut rotation = usize::MAX;
    let mut exclude_faux_positifs: HashSet<XYZ> = HashSet::new();
    let mut cherche_troisieme_balise_c = None;
    let mut position_scanner = XYZ(0, 0, 0);
    while cherche_troisieme_balise_c == None {
        //on peut trouver à partir des distances du scanner_b, 2 balises dans le scanner A qui matche une distance B
        let ra = scanner_a
            .pos
            .iter()
            .filter(|balise| !exclude_faux_positifs.contains(balise))
            .tuple_combinations()
            .find(|(i, j)| intersec.contains(&i.manhattan(j)))
            .unwrap();
        balise_1_scanner_a = *ra.0;
        balise_2_scanner_a = *ra.1;
        let distance_a = balise_1_scanner_a.manhattan(&balise_2_scanner_a);

        // une fois obtenue une distance entre 2 balises dans A
        // on retrouve ces 2 mêmes balises dans B
        let rb = scanner_b
            .pos
            .iter()
            .tuple_combinations()
            .find(|(i, j)| distance_a == i.manhattan(j))
            .unwrap();
        balise_1_scanner_b = *rb.0;
        balise_2_scanner_b = *rb.1;
        let distance_b = balise_1_scanner_b.manhattan(&balise_2_scanner_b);

        if debug {
            println!(
                "distance balise 1/2 du scanner a : {:?}, {:?} = {}",
                balise_1_scanner_a, balise_2_scanner_a, distance_a
            );
            println!(
                "distance balise 1/2 du scanner b : {:?}, {:?} = {}",
                balise_1_scanner_b, balise_2_scanner_b, distance_b
            );
        }

        //exemple de résultat :
        // distances 2 balises a : XYZ(404, -588, -901), XYZ(528, -643, 409) = 1489
        // distances 2 balises b : XYZ(-336, 658, 858), XYZ(-460, 603, -452) = 1489
        // ******************************************************************************
        // super on arrive ici avec 2 balises A et une distance X = 1489
        // et 2 balises B avec la même distance X = 1489
        // par contre, impossible de savoir quelle balise correspond à l'autre, (x_a=404 correspond à x_b=-336 ou x_b=-460 ??)
        // pour ça, il va falloir trouver une troisième balise pivot

        cherche_troisieme_balise_c = scanner_a
            .pos
            .iter()
            .filter(|b| **b != balise_1_scanner_a && **b != balise_2_scanner_a)
            .find(|i| intersec.contains(&balise_1_scanner_a.manhattan(i)))
            .map(|b| (*b, balise_1_scanner_a.manhattan(b)));

        match cherche_troisieme_balise_c {
            None => {
                exclude_faux_positifs.insert(balise_1_scanner_a);
                continue;
            }
            Some(b) => {
                balise_3_scanner_a = b.0;
                distance_balise_1_3_scanner_a = b.1
            }
        }

        if debug {
            println!(
                "distance balise 1/3 du scanner a : {:?}, {:?} = {}",
                balise_1_scanner_a, balise_3_scanner_a, distance_balise_1_3_scanner_a
            );
        }

        cherche_troisieme_balise_c = scanner_b
            .pos
            .iter()
            .filter(|b| **b != balise_1_scanner_b && **b != balise_2_scanner_b)
            .find(|i| {
                distance_balise_1_3_scanner_a == balise_1_scanner_b.manhattan(i)
                    || distance_balise_1_3_scanner_a == balise_2_scanner_b.manhattan(i)
            })
            .map(|b| (*b, i32::MAX)); //on se fout de la distance, car c'est tout simplement la même que distance_balise_1_3_scanner_a qui a été calculé

        match cherche_troisieme_balise_c {
            None => {
                exclude_faux_positifs.insert(balise_1_scanner_a);
                continue;
            }
            Some(b) => {
                balise_3_scanner_b = b.0;
                //distance_balise_1_3_scanner_a = b.1
            }
        }
        if debug {
            println!("balise 3 du scanner b : {:?}", balise_3_scanner_b);
        }

        //si balise_1_scanner_b et balise_2_scanner_b sont inversé, on les renomme correctement
        if balise_1_scanner_a.manhattan(&balise_3_scanner_a) == balise_2_scanner_b.manhattan(&balise_3_scanner_b) {
            if debug {
                println!("inversion !");
            }
            std::mem::swap(&mut balise_2_scanner_b, &mut balise_1_scanner_b)
        }
        if debug {
            println!(
                "balises A : {:?}  {:?}  {:?}",
                balise_1_scanner_a, balise_2_scanner_a, balise_3_scanner_a
            );
            println!(
                "balises B : {:?}  {:?}  {:?}",
                balise_1_scanner_b, balise_2_scanner_b, balise_3_scanner_b
            );
        }
        let potentielle_rotation = (0..24)
            //.inspect(|r | println!("tr : {:?} / point {:?} => {:?} ", balise_2_scanner_a - balise_2_scanner_b.rotation(*r), balise_2_scanner_b, balise_2_scanner_b.rotation(*r) ))
            .find(|r| {
                balise_2_scanner_a - balise_2_scanner_b.rotation(*r) == balise_1_scanner_a - balise_1_scanner_b.rotation(*r)
                    || balise_1_scanner_a - balise_1_scanner_b.rotation(*r) == balise_3_scanner_a - balise_3_scanner_b.rotation(*r)
            });

        //ok problème, on a toujours pas trouvé une rotation ?
        //c'est vraiment pas de bol, mais on a du tomber sur deux faux positifs!
        match potentielle_rotation {
            None => {
                exclude_faux_positifs.insert(balise_1_scanner_a);
                exclude_faux_positifs.insert(balise_2_scanner_a);
                cherche_troisieme_balise_c = None;
                continue;
            }
            Some(rot) => {
                rotation = rot;
            }
        }

        //ultime test, parce qu'on peut encore tomber sur des faux positifs (vraiment pas de chance, mais ça arrive visiblement !)
        //on vérifie donc qu'avec la translation + rotation, on a bien 12 matchs
        position_scanner = balise_3_scanner_a - balise_3_scanner_b.rotation(rotation);

        let touslesbrotatéettranslaté = scanner_b
            .pos
            .iter()
            .map(|b| position_scanner + b.rotation(rotation))
            .collect::<HashSet<XYZ>>();
        if hashseta.intersection(&touslesbrotatéettranslaté).count() < 12 {
            exclude_faux_positifs.insert(balise_1_scanner_a);
            exclude_faux_positifs.insert(balise_2_scanner_a);
            cherche_troisieme_balise_c = None;
        } else if debug {
            touslesbrotatéettranslaté.iter().for_each(|b| println!("b {:?}", b));
            println!(
                "[{},{}] rotation = {}, delta trouvé {:?}",
                scanner_a.n, scanner_b.n, rotation, position_scanner
            );
        }
    }
    if debug {
        println!("orientation : {:?}", position_scanner);
    }
    (rotation, position_scanner)
}

fn distances(scanner: &Scanner) -> HashSet<i32> {
    scanner
        .pos
        .iter()
        .tuple_combinations()
        //.inspect(|(f, g)| {println!("f: {:?} / g: {:?}", f.0, g.0);})
        .map(|(a, b)| a.manhattan(b))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const ÉNONCÉ: &str = 
"--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14";

    #[test]
    fn test_part1() {
        assert_eq!(part1(ÉNONCÉ).unwrap(), 79);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(ÉNONCÉ).unwrap(), 3621);
    }

    #[test]
    fn real_tests() {
        // avec les vraies données
        assert_eq!(part1(include_str!("../inputs/input19.txt")).unwrap(), 318);
        assert_eq!(part2(include_str!("../inputs/input19.txt")).unwrap(), 12166);
    }
}
