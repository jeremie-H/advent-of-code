use std::error::Error;


#[derive(Debug,Clone, PartialEq, Eq)]
struct Monkey {
    numero: u8,
    items: Vec<u64>,
    operation: u8,
    operation_value: u64,
    divisible_by: u64,
    div_true: usize,
    div_false: usize
}

impl Monkey {
    
    fn from (input: &str) -> Self {
        let mut it = input.lines();
        let numero = it.next().unwrap().as_bytes()[7]-b'0';
        let items = it.next().unwrap()[18..].split(", ").map(|f|f.parse::<u64>().unwrap()).collect::<Vec<u64>>();
        let operation_line = it.next().unwrap();
        let mut operation = operation_line.as_bytes()[23];
        if operation_line[25..].eq("old") { operation = b'2';}
        //if square operation, operation_value will not be used
        let operation_value = operation_line[25..].parse::<u64>().unwrap_or_default();
        let divisible_by = it.next().unwrap()[21..].parse::<u64>().unwrap_or_default();
        let div_true = (it.next().unwrap().as_bytes()[29]-b'0') as usize;
        let div_false = (it.next().unwrap().as_bytes()[30]-b'0') as usize;
        Self {numero, items, operation, operation_value, divisible_by, div_true, div_false }
    }
}

/**
 * Part 1
 */
pub fn part1(input: &str) -> Result<i64, Box<dyn Error>> {
    let mut monkeys = input.split("\n\n")
    .map(|bloc| Monkey::from(bloc))
    .collect::<Vec<Monkey>>();
    let mut inspections = vec![0u64; monkeys.len()];
    
    //monkeys rounds
    monkeys_rounds(&mut monkeys, &mut inspections, 20, |x| x / 3);
    
    //display_inspections(&inspections);
    inspections.sort();
    
    Ok(inspections.iter().rev().take(2).product::<u64>() as i64)
}

/**
 * Part 2
 */
pub fn part2(input: &str) -> Result<i64, Box<dyn Error>> {
    let mut monkeys = input.split("\n\n")
    .map(|bloc| Monkey::from(bloc))
    .collect::<Vec<Monkey>>();
    let mut inspections = vec![0u64; monkeys.len()];
    let all_divisibles: u64 = monkeys.iter().map(|m|m.divisible_by).product();
    
    //monkeys rounds
    monkeys_rounds(&mut monkeys, &mut inspections, 10000, |x| x % all_divisibles);
    
    //display_inspections(&inspections);
    inspections.sort();
    
    Ok(inspections.iter().rev().take(2).product::<u64>() as i64)
}

fn monkeys_rounds(monkeys: &mut Vec<Monkey>, inspections: &mut Vec<u64>, rounds: usize, cloj :impl Fn(u64) -> u64) {
    // let mut monkey_clone = monkeys.clone();
    
    for _ in 0..rounds { // rounds
        for j in 0..monkeys.len() {
            // std::mem::swap(&mut monkey_clone[j], &mut monkeys[j]);
            let monkey_copy = monkeys[j].clone();
            monkey_copy.items.iter().for_each(|item|{
                inspections[j] +=1;
                let worry_level = cloj(match monkeys[j].operation {
                    b'+' => item + &monkeys[j].operation_value,
                    b'*' => item * &monkeys[j].operation_value,
                    b'2' => item * item,
                    e => panic!("unknown op {}",e)
                });
                if worry_level % monkeys[j].divisible_by == 0 {
                    monkeys[monkey_copy.div_true].items.push(worry_level);
                } else {
                    monkeys[monkey_copy.div_false].items.push(worry_level);
                }
                monkeys[j].items.clear();

            })
        }
    }
}

#[allow(dead_code)]
fn display_inspections(inspections: &Vec<u64>)  {
    for i in 0..inspections.len() {
        println!("Monkey {} inspected items {}", i, inspections[i]);
    }
}

#[allow(dead_code)]
fn display_monkeys_items(monkeys: &Vec<Monkey>)  {
    monkeys.iter().for_each(|m|{
        println!("Monkey {} : {:?}",m.numero, m.items);
    })
}


#[cfg(test)]
mod tests {
    use super::*;

    const ÉNONCÉ: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

  Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

  Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

  Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn test_part1() {
        assert_eq!(part1(ÉNONCÉ).unwrap(), 10605);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(ÉNONCÉ).unwrap(), 2713310158);
    }

    #[test]
    fn real_tests() {
        // avec les vraies données
        assert_eq!(part1(include_str!("../inputs/input11.txt")).unwrap(), 107822);
        assert_eq!(part2(include_str!("../inputs/input11.txt")).unwrap(), 27267163742);
    }
}
