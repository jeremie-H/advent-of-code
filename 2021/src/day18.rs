use std::{
    error::Error,
    cell::RefCell,
    fmt::{Debug},
};
use itertools::Itertools;
use std::{borrow::BorrowMut, ops::DerefMut};


use crate::day18noeud::Noeud;


/**
 * Part 1
 */
pub fn part1(input: &str) -> Result<i64, Box<dyn Error>> {
    let resultat = input
        .lines()
        .map(|l| read_input(l).0)
        //.inspect(|noeud| println!("lecture noeud : {}",noeud))
        .fold(
            Noeud::Valeur {
                valeur: RefCell::new(Box::new(0)),
            },
            |n, f| {
                if n.value() == 0 {
                    f
                } else {
                    let mut r = n + f;
                    //println!("n = {:?}",r);
                    reduce(&mut r);
                    r
                }
            },
        );
    let magnitude = resultat.magnitude();
    Ok(magnitude)
}

/**
 * Part 2
 */
pub fn part2(input: &str) -> Result<i64, Box<dyn Error>> {
    let resultat = input
        .lines()
        .map(|l| read_input(l).0)
        .permutations(2)
        .map(|v| {
            let mut r = v[0].clone() + v[1].clone();
            reduce(&mut r);
            r.magnitude()
        })
        .max()
        .unwrap();
    Ok(resultat)
}


fn reduce(r: &mut Noeud) {
    loop {
        let rr = explode_fourth_pairs(r, 1, &mut true);
        *r = rr.node;
        if rr.exploded {
            continue;
        }

        let rr = split(r);
        *r = rr.1;
        if rr.0 {
            continue;
        }
        break;
    }
}

#[derive(Debug)]
struct ResultReduce {
    finale: bool,
    exploded: bool,
    node: Noeud,
    gauche: i64,
    droite: i64,
}
#[derive(Debug)]
enum Direction {
    Gauche,
    Droite,
}

fn split(n: &mut Noeud) -> (bool, Noeud) {
    match n {
        Noeud::Valeur { valeur } => {
            let v = **valeur.get_mut();
            if v > 9 {
                let g = v / 2;
                let d = v / 2 + v % 2;
                (
                    true,
                    Noeud::Paire {
                        gauche: RefCell::new(Box::new(Noeud::Valeur {
                            valeur: RefCell::new(Box::new(g)),
                        })),
                        droite: RefCell::new(Box::new(Noeud::Valeur {
                            valeur: RefCell::new(Box::new(d)),
                        })),
                    },
                )
            } else {
                (false, n.clone())
            }
        }
        Noeud::Paire { gauche, droite } => {
            let (split_a_gauche, noeud_gauche) = split(gauche.get_mut());
            let (split_a_droite, noeud_droit) = if !split_a_gauche {
                split(droite.get_mut())
            } else {
                let c = droite.get_mut().to_owned();
                (false, *c)
            };
            (
                (split_a_gauche | split_a_droite),
                Noeud::Paire {
                    gauche: RefCell::new(Box::new(noeud_gauche)),
                    droite: RefCell::new(Box::new(noeud_droit)),
                },
            )
        }
    }
}

fn explode_fourth_pairs(n: &mut Noeud, imbrication: usize, should_explode: &mut bool) -> ResultReduce {
    // println!("[explode_fourth_pairs] n={}",n);
    match n {
        Noeud::Paire { gauche, droite } => {
            let mut reduce_g = explode_fourth_pairs(gauche.get_mut(), imbrication + 1, should_explode);
            *should_explode &= !reduce_g.exploded;
            let mut reduce_d = explode_fourth_pairs(droite.get_mut(), imbrication + 1, should_explode);

            if matches!(&reduce_g.node, Noeud::Valeur { valeur: _v_g }) && matches!(&reduce_d.node, Noeud::Valeur { valeur: _v_d }) {
                // if imbrication>4 {
                //     println!("trouvé : {}, imbrication : {}", n, imbrication);
                // }
                return ResultReduce {
                    finale: imbrication > 4,
                    exploded: false,
                    node: n.clone(),
                    gauche: 0,
                    droite: 0,
                };
            }

            if let Some(value) = process_sub_node(&reduce_g, *should_explode, droite, imbrication, Direction::Gauche) {
                return value;
            }

            if let Some(value) = process_sub_node(&reduce_d, *should_explode, gauche, imbrication, Direction::Droite) {
                return value;
            }

            if reduce_g.droite > 0 && report_sum(&mut reduce_d.node, Direction::Gauche, reduce_g.droite) {
                reduce_g.droite = 0;
            }
            if reduce_d.gauche > 0 && report_sum(&mut reduce_g.node, Direction::Droite, reduce_d.gauche) {
                reduce_d.gauche = 0;
            }

            ResultReduce {
                finale: false,
                exploded: reduce_g.exploded | reduce_d.exploded,
                node: Noeud::Paire {
                    gauche: RefCell::new(Box::new(reduce_g.node)),
                    droite: RefCell::new(Box::new(reduce_d.node)),
                },
                gauche: reduce_g.gauche,
                droite: reduce_d.droite,
            }
        }
        Noeud::Valeur { valeur: _v } => ResultReduce {
            finale: false,
            exploded: false,
            node: n.clone(),
            gauche: 0,
            droite: 0,
        },
    }
}

fn process_sub_node(
    result_reduce: &ResultReduce,
    should_explode: bool,
    parent_node: &mut RefCell<Box<Noeud>>,
    imbrication: usize,
    direction: Direction,
) -> Option<ResultReduce> {
    // println!("[process_sub_node] res_red: {:?}, should_explode: {}, parent_node: {}, direction: {:?}",result_reduce, should_explode, parent_node.get_mut(), direction);
    match &result_reduce.node {
        Noeud::Valeur { valeur: _ } => None,
        Noeud::Paire { gauche: v_g, droite: v_d } => {
            if result_reduce.finale && should_explode {
                match parent_node.get_mut().as_ref() {
                    Noeud::Valeur { valeur: _ } => {
                        let (vg, vd) = match direction {
                            Direction::Gauche => (v_d.borrow().value(), parent_node.get_mut().value()),
                            Direction::Droite => (parent_node.get_mut().value(), v_g.borrow().value()),
                        };
                        // vg = match direction {
                        //     Direction::Gauche => v_d.borrow().value(),
                        //     Direction::Droite => v_g.borrow().value(),
                        // };
                        // let vg = parent_node.clone().borrow().value();
                        //println!("[Paire] vg={}, vd={} ", vg, vd);

                        let (g, d) = match direction {
                            Direction::Gauche => (0, vd + vg),
                            Direction::Droite => (vd + vg, 0),
                        };

                        let node = Noeud::Paire {
                            gauche: RefCell::new(Box::new(Noeud::Valeur {
                                valeur: RefCell::new(Box::new(g)),
                            })),
                            droite: RefCell::new(Box::new(Noeud::Valeur {
                                valeur: RefCell::new(Box::new(d)),
                            })),
                        };
                        // match direction {
                        //     Direction::Gauche => println!("[{},{}] ======> {}", result_reduce.node, parent_node.get_mut(), node),
                        //     Direction::Droite => println!("[{},{}] ======> {}", parent_node.get_mut(), result_reduce.node, node),
                        // }
                        match direction {
                            Direction::Gauche => Some(ResultReduce {
                                finale: imbrication > 4,
                                exploded: true,
                                node,
                                gauche: v_g.borrow().value(),
                                droite: 0,
                            }),
                            Direction::Droite => Some(ResultReduce {
                                finale: imbrication > 4,
                                exploded: true,
                                node,
                                gauche: 0,
                                droite: v_d.borrow().value(),
                            }),
                        }
                    }
                    Noeud::Paire {
                        gauche: _parent_g,
                        droite: _parent_d,
                    } => {
                        //let copy_parent = parent_node.clone();
                        // println!("[Paire] g={}, d={}", v_g.get_mut(), v_d.get_mut());
                        // println!("parent_g {}, parent_d {}", parent_g.borrow(), parent_d.borrow());
                        // println!("[Paire] parent {}", parent_node.get_mut());
                        let valeur_gauche = v_g.borrow().value();
                        let (g, d) = match direction {
                            Direction::Gauche => (0, valeur_gauche),
                            Direction::Droite => (valeur_gauche, 0),
                        };
                        // println!("(g {}, d {})", g, d);

                        // println!("(parent_node(1) {})", &parent_node.get_mut());
                        report_sum(parent_node.get_mut(), Direction::Gauche, v_d.borrow().value());
                        let node = match direction {
                            Direction::Gauche => Noeud::Paire {
                                gauche: RefCell::new(Box::new(Noeud::Valeur {
                                    valeur: RefCell::new(Box::new(g)),
                                })),
                                droite: parent_node.clone(),
                            },
                            Direction::Droite => Noeud::Paire {
                                gauche: parent_node.clone(),
                                droite: RefCell::new(Box::new(Noeud::Valeur {
                                    valeur: RefCell::new(Box::new(d)),
                                })),
                            },
                        };
                        // match direction {
                        //     Direction::Gauche => println!("[{},{}] ======> {}", result_reduce.node, copy_parent.borrow(), node),
                        //     Direction::Droite => println!("[{},{}] ======> {}", copy_parent.borrow(), result_reduce.node, node),
                        // }
                        Some(ResultReduce {
                            finale: imbrication > 4,
                            exploded: true,
                            node,
                            gauche: d,
                            droite: g,
                        })
                    }
                }
            } else {
                None
            }
        }
    }
}

fn report_sum(node: &mut Noeud, direction: Direction, assign: i64) -> bool {
    match node {
        Noeud::Valeur { valeur } => {
            valeur.borrow_mut().replace_with(|old| Box::new(*old.deref_mut() + assign));
            return true;
        }
        Noeud::Paire { gauche, droite } => match direction {
            Direction::Gauche => {
                report_sum(gauche.get_mut(), Direction::Gauche, assign);
            }
            Direction::Droite => {
                report_sum(droite.get_mut(), Direction::Droite, assign);
            }
        },
    };
    false
}

fn read_input(l: &str) -> (Noeud, usize) {
    let mut indice = 1;

    let start_noeud = match l.as_bytes()[indice] {
        b'[' => {
            let (noeud, p) = read_input(&l[indice..]);
            indice += p;
            noeud
        }
        v if (b'0'..=b'9').contains(&v) => {
            indice += 1;
            Noeud::Valeur {
                valeur: RefCell::new(Box::new((v - b'0') as i64)),
            }
        }
        _ => panic!("wow, c'est quoi cette chaine chelou ?"),
    };
    //on passe la virgule
    indice += 1;

    let end_noeud: Noeud = match l.as_bytes()[indice] {
        b'[' => {
            let (noeud, p) = read_input(&l[indice..]);
            indice += p;
            noeud
        }
        v if (b'0'..=b'9').contains(&v) => {
            indice += 1;
            Noeud::Valeur {
                valeur: RefCell::new(Box::new((v - b'0') as i64)),
            }
        }
        _ => panic!("wow, c'est quoi cette chaine chelou ?"),
    };
    //println!("[read_input] gauche = {}, droite = {}",start_noeud, end_noeud);

    (   Noeud::Paire {
            gauche: RefCell::new(Box::new(start_noeud)),
            droite: RefCell::new(Box::new(end_noeud)),
        },
        indice + 1,
    )
}


#[cfg(test)]
mod tests {
    use super::*;

    const ÉNONCÉ: &str = 
"[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";
    #[test]
    fn test_part1() {
        assert_eq!(part1("[[1,2],[[3,4],5]]").unwrap(), 143);
        assert_eq!(part1("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]").unwrap(), 1384);
        assert_eq!(part1("[[[[1,1],[2,2]],[3,3]],[4,4]]").unwrap(), 445);
        assert_eq!(part1("[[[[3,0],[5,3]],[4,4]],[5,5]]").unwrap(), 791);
        assert_eq!(part1("[[[[5,0],[7,4]],[5,5]],[6,6]]").unwrap(), 1137);
        assert_eq!(part1("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]").unwrap(), 3488);
        assert_eq!(part1(ÉNONCÉ).unwrap(), 4140);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(ÉNONCÉ).unwrap(), 3993);
    }

    #[test]
    fn real_tests() {
        // avec les vraies données
        assert_eq!(part1(include_str!("../inputs/input18.txt")).unwrap(), 4457);
        assert_eq!(part2(include_str!("../inputs/input18.txt")).unwrap(), 4784);
    }
}
