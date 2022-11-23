use std::{
    fmt::Debug,
    ops::{Add, Sub},
};

#[allow(clippy::upper_case_acronyms)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct XYZ(pub i32, pub i32, pub i32);

impl Add for XYZ {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output { Self(self.0 + other.0, self.1 + other.1, self.2 + other.2) }
}

impl Sub for XYZ {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output { Self(self.0 - other.0, self.1 - other.1, self.2 - other.2) }
}

impl Debug for XYZ {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let a = match self.0 {
            i32::MAX => "###".to_string(),
            _ => self.0.to_string(),
        };
        let b = match self.1 {
            i32::MAX => "###".to_string(),
            _ => self.1.to_string(),
        };
        let c = match self.2 {
            i32::MAX => "###".to_string(),
            _ => self.2.to_string(),
        };
        write!(f, "[{: >5},{: >5},{: >5}]", a, b, c)
    }
}

impl XYZ {
    pub fn rotation(&self, n: usize) -> XYZ {
        match n {
            //x en face
            0 => XYZ(self.0, self.1, self.2),
            1 => XYZ(self.0, -self.2, self.1),
            2 => XYZ(self.0, -self.1, -self.2),
            3 => XYZ(self.0, self.2, -self.1),
            //x opposé
            4 => XYZ(-self.0, -self.1, self.2),
            5 => XYZ(-self.0, self.2, self.1),
            6 => XYZ(-self.0, self.1, -self.2),
            7 => XYZ(-self.0, -self.2, -self.1),
            //y en face
            8 => XYZ(self.1, self.2, self.0),
            9 => XYZ(self.1, -self.0, self.2),
            10 => XYZ(self.1, -self.2, -self.0),
            11 => XYZ(self.1, self.0, -self.2),
            //y opposé
            12 => XYZ(-self.1, -self.2, self.0),
            13 => XYZ(-self.1, self.0, self.2),
            14 => XYZ(-self.1, self.2, -self.0),
            15 => XYZ(-self.1, -self.0, -self.2),
            //z en face
            16 => XYZ(self.2, self.0, self.1),
            17 => XYZ(self.2, -self.1, self.0),
            18 => XYZ(self.2, -self.0, -self.1),
            19 => XYZ(self.2, self.1, -self.0),
            //z opposé
            20 => XYZ(-self.2, -self.0, self.1),
            21 => XYZ(-self.2, self.1, self.0),
            22 => XYZ(-self.2, self.0, -self.1),
            23 => XYZ(-self.2, -self.1, -self.0),
            n => panic!("éh ho tu te calmes là {}", n),
        }
    }

    pub fn composition_rotation(r1: usize, r2: usize) -> usize {
        let exemple_simple = XYZ(1, 2, 3);
        match exemple_simple.rotation(r1).rotation(r2) {
            //x en face
            XYZ(1, 2, 3) => 0,
            XYZ(1, -3, 2) => 1,
            XYZ(1, -2, -3) => 2,
            XYZ(1, 3, -2) => 3,
            //x opposé
            XYZ(-1, -2, 3) => 4,
            XYZ(-1, 3, 2) => 5,
            XYZ(-1, 2, -3) => 6,
            XYZ(-1, -3, -2) => 7,
            //y en face
            XYZ(2, 3, 1) => 8,
            XYZ(2, -1, 3) => 9,
            XYZ(2, -3, -1) => 10,
            XYZ(2, 1, -3) => 11,
            //y opposé
            XYZ(-2, -3, 1) => 12,
            XYZ(-2, 1, 3) => 13,
            XYZ(-2, 3, -1) => 14,
            XYZ(-2, -1, -3) => 15,
            //z en face
            XYZ(3, 1, 2) => 16,
            XYZ(3, -2, 1) => 17,
            XYZ(3, -1, -2) => 18,
            XYZ(3, 2, -1) => 19,
            //z opposé
            XYZ(-3, -1, 2) => 20,
            XYZ(-3, 2, 1) => 21,
            XYZ(-3, 1, -2) => 22,
            XYZ(-3, -2, -1) => 23,
            _ => panic!("stop la colle"),
        }
    }

    pub fn manhattan(&self, other: &XYZ) -> i32 { (self.0 - other.0).abs() + (self.1 - other.1).abs() + (self.2 - other.2).abs() }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Scanner {
    pub n: usize,
    pub pos: Vec<XYZ>,
}

impl Scanner {
    pub fn new(n: usize, pos: Vec<XYZ>) -> Scanner { Scanner { n, pos } }
}
