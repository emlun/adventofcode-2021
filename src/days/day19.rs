use crate::common::Solution;
use std::collections::HashSet;
use std::fmt::Display;
use std::ops::Add;
use std::ops::Mul;
use std::ops::Sub;

#[derive(Clone, Debug)]
struct Scanner {
    beacons: Vec<Vec3<i64>>,
}

impl Scanner {
    fn new(beacons: Vec<Vec3<i64>>) -> Self {
        Self { beacons }
    }

    fn translate(self, dxyz: &Vec3<i64>) -> Self {
        Self {
            beacons: self.beacons.into_iter().map(|b| &b + dxyz).collect(),
        }
    }

    fn rotate(&self, rot: &Matrix3<i64>) -> Self {
        Self {
            beacons: self.beacons.iter().map(|v| rot * v).collect(),
        }
    }
}

impl Display for Scanner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        writeln!(f, "[Scanner]")?;
        for b in &self.beacons {
            writeln!(f, "{}", b)?
        }
        Ok(())
    }
}

enum Angle {
    Deg0,
    Deg90,
    Deg180,
    Deg270,
}
use Angle::Deg0;
use Angle::Deg180;
use Angle::Deg270;
use Angle::Deg90;

impl Angle {
    const fn sin(&self) -> i64 {
        match self {
            Self::Deg0 => 0,
            Self::Deg90 => 1,
            Self::Deg180 => 0,
            Self::Deg270 => -1,
        }
    }

    const fn cos(&self) -> i64 {
        match self {
            Self::Deg0 => 1,
            Self::Deg90 => 0,
            Self::Deg180 => -1,
            Self::Deg270 => 0,
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Vec3<T> {
    x: T,
    y: T,
    z: T,
}

impl Vec3<i64> {
    fn abs(&self) -> i64 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

impl<T> Display for Vec3<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "[{}, {}, {}]", self.x, self.y, self.z)
    }
}

impl<T> Add<&Vec3<T>> for &Vec3<T>
where
    T: Copy,
    T: Add<T, Output = T>,
{
    type Output = Vec3<T>;
    fn add(self, other: &Vec3<T>) -> <Self as Add<&Vec3<T>>>::Output {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T> Sub<Self> for &Vec3<T>
where
    T: Copy,
    T: Sub<T, Output = T>,
{
    type Output = Vec3<T>;
    fn sub(self, other: Self) -> <Self as Sub<Self>>::Output {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

#[derive(Clone)]
struct Matrix3<T> {
    col1: Vec3<T>,
    col2: Vec3<T>,
    col3: Vec3<T>,
}

impl Matrix3<i64> {
    const ID: Self = Self::rotx(Deg0);

    const fn rotx(angle: Angle) -> Self {
        Self {
            col1: Vec3 { x: 1, y: 0, z: 0 },
            col2: Vec3 {
                x: 0,
                y: angle.cos(),
                z: angle.sin(),
            },
            col3: Vec3 {
                x: 0,
                y: -angle.sin(),
                z: angle.cos(),
            },
        }
    }

    const fn roty(angle: Angle) -> Self {
        Self {
            col1: Vec3 {
                x: angle.cos(),
                y: 0,
                z: -angle.sin(),
            },
            col2: Vec3 { x: 0, y: 1, z: 0 },
            col3: Vec3 {
                x: angle.sin(),
                y: 0,
                z: angle.cos(),
            },
        }
    }

    const fn rotz(angle: Angle) -> Self {
        Self {
            col1: Vec3 {
                x: angle.cos(),
                y: angle.sin(),
                z: 0,
            },
            col2: Vec3 {
                x: -angle.sin(),
                y: angle.cos(),
                z: 0,
            },
            col3: Vec3 { x: 0, y: 0, z: 1 },
        }
    }

    const ALL_ROTATIONS: [Self; 24] = [
        Self::ID,
        Self::rotx(Deg90),
        Self::rotx(Deg180),
        Self::rotx(Deg270),
        Self::rotz(Deg90).matrix_mul(Self::ID),
        Self::rotz(Deg90).matrix_mul(Self::rotx(Deg90)),
        Self::rotz(Deg90).matrix_mul(Self::rotx(Deg180)),
        Self::rotz(Deg90).matrix_mul(Self::rotx(Deg270)),
        Self::rotz(Deg180).matrix_mul(Self::ID),
        Self::rotz(Deg180).matrix_mul(Self::rotx(Deg90)),
        Self::rotz(Deg180).matrix_mul(Self::rotx(Deg180)),
        Self::rotz(Deg180).matrix_mul(Self::rotx(Deg270)),
        Self::rotz(Deg270).matrix_mul(Self::ID),
        Self::rotz(Deg270).matrix_mul(Self::rotx(Deg90)),
        Self::rotz(Deg270).matrix_mul(Self::rotx(Deg180)),
        Self::rotz(Deg270).matrix_mul(Self::rotx(Deg270)),
        Self::roty(Deg90),
        Self::rotz(Deg90).matrix_mul(Self::roty(Deg90)),
        Self::rotz(Deg180).matrix_mul(Self::roty(Deg90)),
        Self::rotz(Deg270).matrix_mul(Self::roty(Deg90)),
        Self::roty(Deg270),
        Self::rotz(Deg90).matrix_mul(Self::roty(Deg270)),
        Self::rotz(Deg180).matrix_mul(Self::roty(Deg270)),
        Self::rotz(Deg270).matrix_mul(Self::roty(Deg270)),
    ];

    const fn matrix_mul(self, other: Self) -> Self {
        Matrix3 {
            col1: self.vector_mul(other.col1),
            col2: self.vector_mul(other.col2),
            col3: self.vector_mul(other.col3),
        }
    }

    const fn vector_mul(&self, v: Vec3<i64>) -> Vec3<i64> {
        Vec3 {
            x: self.col1.x * v.x + self.col2.x * v.y + self.col3.x * v.z,
            y: self.col1.y * v.x + self.col2.y * v.y + self.col3.y * v.z,
            z: self.col1.z * v.x + self.col2.z * v.y + self.col3.z * v.z,
        }
    }
}

impl<'m, 'v, T> Mul<&'v Vec3<T>> for &'m Matrix3<T>
where
    T: Copy,
    T: Add<T, Output = T>,
    T: Mul<T, Output = T>,
{
    type Output = Vec3<T>;
    fn mul(self, v: &'v Vec3<T>) -> <Self as Mul<&'v Vec3<T>>>::Output {
        Vec3 {
            x: self.col1.x * v.x + self.col2.x * v.y + self.col3.x * v.z,
            y: self.col1.y * v.x + self.col2.y * v.y + self.col3.y * v.z,
            z: self.col1.z * v.x + self.col2.z * v.y + self.col3.z * v.z,
        }
    }
}

fn find_overlap(scana: &Scanner, scanb: &Scanner) -> Option<(Vec3<i64>, Scanner)> {
    Matrix3::ALL_ROTATIONS.into_iter().find_map(|rot| {
        let brot = scanb.rotate(&rot);
        for origin_a in &scana.beacons {
            for origin_b in &brot.beacons {
                let pos = origin_a - origin_b;
                if brot
                    .beacons
                    .iter()
                    .filter(|b| scana.beacons.contains(&(*b + &pos)))
                    .take(3)
                    .count()
                    == 3
                {
                    let btrans = brot.translate(&pos);
                    return Some((pos, btrans));
                }
            }
        }
        None
    })
}

pub fn solve(lines: &[String]) -> Solution {
    let scanners: Vec<Scanner> = lines
        .iter()
        .filter(|l| !l.is_empty())
        .fold(Vec::new(), |mut scanners, l| {
            if l.starts_with("---") {
                scanners.push(Vec::new());
            } else {
                let mut coords = l.split(',').map(|num| num.parse().unwrap());
                scanners.last_mut().unwrap().push(Vec3 {
                    x: coords.next().unwrap(),
                    y: coords.next().unwrap(),
                    z: coords.next().unwrap(),
                });
            }
            scanners
        })
        .into_iter()
        .map(Scanner::new)
        .collect();

    let mut known: Vec<Option<(Vec3<i64>, Scanner)>> = vec![None; scanners.len()];
    known[0] = Some((Vec3 { x: 0, y: 0, z: 0 }, scanners[0].clone()));
    let mut futile: HashSet<(usize, usize)> = HashSet::new();

    while known.iter().any(|k| k.is_none()) {
        for i in 0..scanners.len() {
            if known[i].is_some() {
                for j in 0..scanners.len() {
                    if known[j].is_none() && !futile.contains(&(i, j)) {
                        if let Some((posb, absolute_b)) =
                            find_overlap(&known[i].as_ref().unwrap().1, &scanners[j])
                        {
                            known[j] = Some((posb, absolute_b));
                        } else {
                            futile.insert((i, j));
                        }
                    }
                }
            }
        }
    }

    let sol_b = (0..scanners.len())
        .flat_map(|i| ((i + 1)..scanners.len()).map(move |j| (i, j)))
        .map(|(i, j)| (&known[i].as_ref().unwrap().0 - &known[j].as_ref().unwrap().0).abs())
        .max()
        .unwrap();

    let sol_a = known
        .into_iter()
        .flat_map(|k| {
            if let Some((_, scan)) = k {
                scan.beacons.into_iter()
            } else {
                unreachable!()
            }
        })
        .collect::<HashSet<Vec3<i64>>>()
        .len();

    (sol_a.to_string(), sol_b.to_string())
}
