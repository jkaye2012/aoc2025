use std::str::FromStr;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct Point3D {
    x: i64,
    y: i64,
    z: i64,
}

impl FromStr for Point3D {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(',');
        let x: i64 = split
            .next()
            .ok_or("missing x")?
            .parse()
            .map_err(|_| "failed to parse x")?;
        let y: i64 = split
            .next()
            .ok_or("missing y")?
            .parse()
            .map_err(|_| "failed to parse y")?;
        let z: i64 = split
            .next()
            .ok_or("missing z")?
            .parse()
            .map_err(|_| "failed to parse z")?;
        Ok(Self { x, y, z })
    }
}

impl Point3D {
    pub fn euclidean_distance(&self, other: &Self) -> i64 {
        ((self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)).isqrt()
    }

    pub fn x(&self) -> i64 {
        self.x
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct PointDistance {
    distance: i64,
    a: Point3D,
    b: Point3D,
}

impl PointDistance {
    pub fn new(a: Point3D, b: Point3D) -> Self {
        Self {
            a,
            b,
            distance: a.euclidean_distance(&b),
        }
    }

    pub fn a(&self) -> &Point3D {
        &self.a
    }

    pub fn b(&self) -> &Point3D {
        &self.b
    }
}

impl PartialOrd for PointDistance {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PointDistance {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.distance.cmp(&other.distance)
    }
}
