use itertools::Itertools;
use std::{error::Error, usize};

/**
 * Part 1
 */
pub fn part1(input: &str) -> Result<i64, Box<dyn Error>> {
    let tableau = input
        .chars()
        .tuples::<(_, _)>()
        .map(|(c1, c2)| ((c1.to_digit(16).unwrap() as u8) << 4) | c2.to_digit(16).unwrap() as u8)
        .collect::<Vec<u8>>();

    let mut bf = Bitfield::load_from_vec(&tableau);
    //bf.display();
    let (version, _calcul) = read_unknown_packet(&mut bf);
    Ok(version as i64)
}

/**
 * Part 2
 */
pub fn part2(input: &str) -> Result<i64, Box<dyn Error>> {
    let tableau = input
        .chars()
        .tuples::<(_, _)>()
        .map(|(c1, c2)| ((c1.to_digit(16).unwrap() as u8) << 4) | c2.to_digit(16).unwrap() as u8)
        .collect::<Vec<u8>>();

    let mut bf = Bitfield::load_from_vec(&tableau);
    //bf.display();
    let (_, calcul) = read_unknown_packet(&mut bf);
    Ok(calcul as i64)
}


/**
 * retourne la somme des versions pour part1 et le calcul pour part2
 */
fn read_unknown_packet(bf: &mut Bitfield) -> (u64, u64) {
    let (version, type_de_packet) = read_header(bf);
    //println!("    [read_unknown_packet] {:?} packet version : {}", type_de_packet, version);
    match type_de_packet {
        PacketType::Literal => (version, read_literal(bf)),
        PacketType::Sum => {
            let (v, calcul) = read_sum(bf);
            (version + v, calcul)
        }
        PacketType::Product => {
            let (v, calcul) = read_product(bf);
            (version + v, calcul)
        }
        PacketType::Minimum => {
            let (v, calcul) = read_minimum(bf);
            (version + v, calcul)
        }
        PacketType::Maximum => {
            let (v, calcul) = read_maximum(bf);
            (version + v, calcul)
        }
        PacketType::Greater => {
            let (v, calcul) = read_greater(bf);
            (version + v, calcul)
        }
        PacketType::Less => {
            let (v, calcul) = read_less(bf);
            (version + v, calcul)
        }
        PacketType::Equals => {
            let (v, calcul) = read_equals(bf);
            (version + v, calcul)
        }
    }
}

fn read_equals(bf: &mut Bitfield) -> (u64, u64) {
    let type_id = bf.get_bits(1);
    let (versions, agg) = aggregate_numbers(type_id, bf);
    let equals = if agg[0] == agg[1] { 1 } else { 0 };
    (versions, equals)
}

fn read_less(bf: &mut Bitfield) -> (u64, u64) {
    let type_id = bf.get_bits(1);
    let (versions, agg) = aggregate_numbers(type_id, bf);
    let less = if agg[0] < agg[1] { 1 } else { 0 };
    (versions, less)
}

fn read_greater(bf: &mut Bitfield) -> (u64, u64) {
    let type_id = bf.get_bits(1);
    let (versions, agg) = aggregate_numbers(type_id, bf);
    let greater = if agg[0] > agg[1] { 1 } else { 0 };
    (versions, greater)
}

fn read_maximum(bf: &mut Bitfield) -> (u64, u64) {
    let type_id = bf.get_bits(1);
    let (versions, agg) = aggregate_numbers(type_id, bf);
    (versions, *agg.iter().max().unwrap())
}

fn read_minimum(bf: &mut Bitfield) -> (u64, u64) {
    let type_id = bf.get_bits(1);
    let (versions, agg) = aggregate_numbers(type_id, bf);
    (versions, *agg.iter().min().unwrap())
}

fn read_literal(bf: &mut Bitfield) -> u64 {
    let mut start = bf.get_bits(1);
    let mut result = 0;
    while start == 1 {
        result = result << 4 | bf.get_bits(4);
        start = bf.get_bits(1);
    }
    //last packet
    result = result << 4 | bf.get_bits(4);
    //println!("  LITERAL {}", result);
    result
}

fn read_sum(bf: &mut Bitfield) -> (u64, u64) {
    let type_id = bf.get_bits(1);
    let (versions, agg) = aggregate_numbers(type_id, bf);
    (versions, agg.iter().sum())
}

fn read_product(bf: &mut Bitfield) -> (u64, u64) {
    let type_id = bf.get_bits(1);
    let (versions, agg) = aggregate_numbers(type_id, bf);
    (versions, agg.iter().product())
}

fn aggregate_numbers(type_id: u64, bf: &mut Bitfield) -> (u64, Vec<u64>) {
    let mut versions = 0;
    let mut numbers = Vec::new();
    if type_id == 1 {
        let sub_packet_length = bf.get_bits(11);
        //println!("sub_packet_length {}",sub_packet_length);
        for _i in 0..sub_packet_length {
            let (v, c) = read_unknown_packet(bf);
            versions += v;
            numbers.push(c);
        }
    } else if type_id == 0 {
        let total_length = bf.get_bits(15) as usize;
        //println!("total_length {}",total_length);
        let position_initiale = bf.get_position();
        let mut position = bf.get_position();
        while position < position_initiale + total_length {
            let (v, c) = read_unknown_packet(bf);
            versions += v;
            numbers.push(c);
            position = bf.get_position();
        }
    } else {
        unreachable!("type_id ne devrait pas être différent de 0 ou 1");
    }
    (versions, numbers)
}

fn read_header(bf: &mut Bitfield) -> (u64, PacketType) {
    let version = bf.get_bits(3);
    let operator = bf.get_bits(3);
    //println!("[HEADER] version {} / operator {}", version, operator);
    (
        version,
        match operator {
            4 => PacketType::Literal,
            0 => PacketType::Sum,
            1 => PacketType::Product,
            2 => PacketType::Minimum,
            3 => PacketType::Maximum,
            5 => PacketType::Greater,
            6 => PacketType::Less,
            7 => PacketType::Equals,
            _ => PacketType::Literal,
        },
    )
}

#[derive(Debug)]
enum PacketType {
    Literal,
    Sum,
    Product,
    Minimum,
    Maximum,
    Greater,
    Less,
    Equals,
}

struct Bitfield<'a> {
    tableau: &'a [u8],
    position: usize,
}

impl<'a> Bitfield<'a> {
    fn load_from_vec(tab: &'a [u8]) -> Self {
        Self {
            tableau: tab,
            position: 0,
        }
    }
    #[allow(unused)]
    fn display(&self) {
        for i in self.tableau {
            //affiche sous format binaire, 4 digits, et complète de 0 à gauche, si c'est 0
            //   0 => 0000
            // 101 => 0101
            println!("{:0>8b}", i);
        }
    }

    fn get_bits(&mut self, nb: usize) -> u64 {
        let index = self.position / 8;
        let bit_start = self.position - ((self.position / 8) * 8);
        let bits_pris_dans_case = 8 - bit_start;
        //println!("pos: {} ++ get bits({}) : index {} / bit_start {} / bits_dispo_1er_case {}",self.position, nb, index, bit_start, bits_pris_dans_case);

        let result = self.tableau[index] << bit_start;

        let shift_inverse_calcul = if nb < bits_pris_dans_case {
            bits_pris_dans_case - nb + bit_start
        } else {
            bit_start
        };

        //si ça tient dans le u8 tronqué, on peut ramener tel quel
        let result: u64 = if nb <= bits_pris_dans_case {
            (result >> shift_inverse_calcul) as u64
        }
        //sinon c'est la merde, faut aller taper sur le(s) u8 suivant(s)
        else {
            let mut bits_pris_suite;
            let mut reste = usize::MAX;
            let mut construction: u64 = (result >> shift_inverse_calcul) as u64;
            let mut compteur = 0;
            while reste > 0 {
                bits_pris_suite = if nb - bits_pris_dans_case - ((compteur) * 8) > 8 {
                    8
                } else {
                    nb - (compteur * 8) - bits_pris_dans_case
                };
                reste = nb - bits_pris_dans_case - bits_pris_suite - ((compteur) * 8);

                construction = (construction << (bits_pris_suite)) | (self.tableau[index + compteur + 1] >> (8 - bits_pris_suite)) as u64;
                compteur += 1;
            }
            construction
        };
        //println!("binary : {:0>nb$b} / dec : {}",result, result);
        self.position += nb;
        result
    }
    fn get_position(&self) -> usize { self.position }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("8A004A801A8002F478").unwrap(), 16);
        assert_eq!(part1("620080001611562C8802118E34").unwrap(), 12);
        assert_eq!(part1("C0015000016115A2E0802F182340").unwrap(), 23);
        assert_eq!(part1("A0016C880162017C3686B18A3D4780").unwrap(), 31);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("C200B40A82").unwrap(), 3);
        assert_eq!(part2("04005AC33890").unwrap(), 54);
        assert_eq!(part2("880086C3E88112").unwrap(), 7);
        assert_eq!(part2("CE00C43D881120").unwrap(), 9);
        assert_eq!(part2("D8005AC2A8F0").unwrap(), 1);
        assert_eq!(part2("F600BC2D8F").unwrap(), 0);
        assert_eq!(part2("9C005AC2F8F0").unwrap(), 0);
        assert_eq!(part2("9C0141080250320F1802104A08").unwrap(), 1);
    }

    #[test]
    fn real_tests() {
        // avec les vraies données
        assert_eq!(part1(include_str!("../inputs/input16.txt")).unwrap(), 901);
        assert_eq!(part2(include_str!("../inputs/input16.txt")).unwrap(), 110434737925);
    }
}
