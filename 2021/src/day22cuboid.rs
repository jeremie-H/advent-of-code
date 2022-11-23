#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Cuboid {
    pub state: i8, // 1 si on, 0 si off
    pub range_x: (i32, i32),
    pub range_y: (i32, i32),
    pub range_z: (i32, i32),
    pub volume: i64,
}

impl Cuboid {
    pub fn new(state: i8, x_start: i32, x_end: i32, y_start: i32, y_end: i32, z_start: i32, z_end: i32) -> Cuboid {
        Cuboid {
            state,
            range_x: (x_start, x_end),
            range_y: (y_start, y_end),
            range_z: (z_start, z_end),
            volume: ((x_end - x_start).abs() + 1) as i64 * ((y_end - y_start).abs() + 1) as i64 * ((z_end - z_start).abs() + 1) as i64,
        }
    }

    pub fn intersection(&self, other: &Cuboid) -> Option<Cuboid> {
        let state = -other.state;
        let (startx, endx) = (self.range_x.0.max(other.range_x.0), self.range_x.1.min(other.range_x.1));
        if startx > endx {
            return None;
        }
        let (starty, endy) = (self.range_y.0.max(other.range_y.0), self.range_y.1.min(other.range_y.1));
        if starty > endy {
            return None;
        }
        let (startz, endz) = (self.range_z.0.max(other.range_z.0), self.range_z.1.min(other.range_z.1));
        if startz > endz {
            return None;
        }

        Some(Cuboid::new(state, startx, endx, starty, endy, startz, endz))
    }
}

impl std::fmt::Debug for Cuboid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "([{}:{}] x={}..{}, y={}..{}, z={}..{})",
            self.volume, self.state, self.range_x.0, self.range_x.1, self.range_y.0, self.range_y.1, self.range_z.0, self.range_z.1,
        )
    }
}
