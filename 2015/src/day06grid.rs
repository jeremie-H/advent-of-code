#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Grid {
    pub state: i8, // 1 si on, 0 si off
    pub corner_a: (usize, usize),
    pub corner_b: (usize, usize),
    //pub surface: i64,
}

impl Grid {
    pub fn new(state: i8, a_x: usize, a_y: usize, b_x: usize, b_y: usize) -> Grid {
        Grid {
            state,
            corner_a: (a_x, a_y),
            corner_b: (b_x, b_y),
            //surface: ((b_x - a_x).abs() + 1) as i64 * ((b_y - a_y).abs() + 1) as i64,
        }
    }
    pub fn from_str(state: &str, a: &str, b: &str) -> Grid {
        if let Some((a1, a2)) = a.split_once(',') {
            if let Some((b1, b2)) = b.split_once(',') {
                Grid {
                    state: match state {
                        "turn on" => 1,
                        "turn off" => 0,
                        "toggle" => 2,
                        _ => -1,
                    },
                    corner_a: (a1.parse::<usize>().unwrap(), a2.parse::<usize>().unwrap()),
                    corner_b: (b1.parse::<usize>().unwrap(), b2.parse::<usize>().unwrap()),
                    //surface: ((b_x - a_x).abs() + 1) as i64 * ((b_y - a_y).abs() + 1) as i64,
                }
            } else {
                panic!("error in from_str")
            }
        } else {
            panic!("error in from_str")
        }
    }

    // pub fn intersection(&self, other: &Grid) -> Option<Grid> {
    //     let state = -other.state;
    //     let (maxx, minx) = (self.corner_a.0.max(other.corner_a.0), self.corner_b.0.min(other.corner_b.0));
    //     if maxx > minx {
    //         return None;
    //     }
    //     let (maxy, miny) = (self.corner_a.1.max(other.corner_a.1), self.corner_b.1.min(other.corner_b.1));
    //     if maxy > miny {
    //         return None;
    //     }

    //     Some(Grid::new(state, maxx, maxy, minx, miny))
    // }
}

impl std::fmt::Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // write!(
        //     f,
        //     "([{}:{}] a={},{}, b={},{})",
        //     self.surface, self.state, self.corner_a.0, self.corner_a.1, self.corner_b.0, self.corner_b.1,
        // )
        write!(f, "([{}] a={},{}, b={},{})", self.state, self.corner_a.0, self.corner_a.1, self.corner_b.0, self.corner_b.1,)
    }
}
