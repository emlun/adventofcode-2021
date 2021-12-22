use crate::common::Solution;

#[derive(Clone, Debug)]
struct Cuboid<T> {
    xbgn: T,
    xend: T,
    ybgn: T,
    yend: T,
    zbgn: T,
    zend: T,
}

impl Cuboid<i64> {
    fn len(&self) -> i64 {
        std::cmp::max(0, self.xend - self.xbgn)
            * std::cmp::max(0, self.yend - self.ybgn)
            * std::cmp::max(0, self.zend - self.zbgn)
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn intersect(&self, other: &Self) -> Option<Self> {
        Some(Self {
            xbgn: std::cmp::max(self.xbgn, other.xbgn),
            xend: std::cmp::min(self.xend, other.xend),
            ybgn: std::cmp::max(self.ybgn, other.ybgn),
            yend: std::cmp::min(self.yend, other.yend),
            zbgn: std::cmp::max(self.zbgn, other.zbgn),
            zend: std::cmp::min(self.zend, other.zend),
        })
        .filter(|c| !c.is_empty())
    }

    fn subtract(self, other: &Self) -> Vec<Self> {
        if let Some(intr) = self.intersect(other) {
            [
                Self {
                    xbgn: self.xbgn,
                    xend: intr.xbgn,
                    ybgn: self.ybgn,
                    yend: self.yend,
                    zbgn: self.zbgn,
                    zend: self.zend,
                },
                Self {
                    xbgn: intr.xbgn,
                    xend: intr.xend,
                    ybgn: self.ybgn,
                    yend: intr.ybgn,
                    zbgn: self.zbgn,
                    zend: self.zend,
                },
                Self {
                    xbgn: intr.xbgn,
                    xend: intr.xend,
                    ybgn: intr.ybgn,
                    yend: intr.yend,
                    zbgn: self.zbgn,
                    zend: intr.zbgn,
                },
                Self {
                    xbgn: intr.xbgn,
                    xend: intr.xend,
                    ybgn: intr.ybgn,
                    yend: intr.yend,
                    zbgn: intr.zend,
                    zend: self.zend,
                },
                Self {
                    xbgn: intr.xbgn,
                    xend: intr.xend,
                    ybgn: intr.yend,
                    yend: self.yend,
                    zbgn: self.zbgn,
                    zend: self.zend,
                },
                Self {
                    xbgn: intr.xend,
                    xend: self.xend,
                    ybgn: self.ybgn,
                    yend: self.yend,
                    zbgn: self.zbgn,
                    zend: self.zend,
                },
            ]
            .into_iter()
            .filter(|c| !c.is_empty())
            .collect()
        } else {
            vec![self]
        }
    }
}

fn run<I: Iterator<Item = (bool, Cuboid<i64>)>>(steps: I) -> i64 {
    steps
        .fold(Vec::new(), |mut on_regions, (on, c)| {
            if on {
                let reduced = on_regions.iter().fold(vec![c], |red, already_on| {
                    red.into_iter()
                        .flat_map(|r| r.subtract(already_on))
                        .collect()
                });
                on_regions.extend(reduced);
                on_regions
            } else {
                on_regions
                    .into_iter()
                    .flat_map(|reg| reg.subtract(&c))
                    .collect()
            }
        })
        .into_iter()
        .map(|reg| reg.len())
        .sum()
}

pub fn solve(lines: &[String]) -> Solution {
    let steps: Vec<(bool, Cuboid<i64>)> = lines
        .iter()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let mut splits = l.split(' ');
            let on = splits.next().unwrap() == "on";
            let mut coords = splits.next().unwrap().split(',').flat_map(|coord| {
                coord
                    .split('=')
                    .nth(1)
                    .unwrap()
                    .split("..")
                    .map(|c| c.parse().unwrap())
            });
            (
                on,
                Cuboid {
                    xbgn: coords.next().unwrap(),
                    xend: coords.next().unwrap() + 1,
                    ybgn: coords.next().unwrap(),
                    yend: coords.next().unwrap() + 1,
                    zbgn: coords.next().unwrap(),
                    zend: coords.next().unwrap() + 1,
                },
            )
        })
        .collect();

    let sol_a = run(steps.iter().map(|(on, c)| {
        (
            *on,
            Cuboid {
                xbgn: std::cmp::max(-50, c.xbgn),
                xend: std::cmp::min(51, c.xend),
                ybgn: std::cmp::max(-50, c.ybgn),
                yend: std::cmp::min(51, c.yend),
                zbgn: std::cmp::max(-50, c.zbgn),
                zend: std::cmp::min(51, c.zend),
            },
        )
    }));
    let sol_b = run(steps.into_iter());

    (sol_a.to_string(), sol_b.to_string())
}
