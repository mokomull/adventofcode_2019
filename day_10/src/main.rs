use std::convert::{TryFrom, TryInto};

fn main() {
    println!("Hello, world!");
}

struct Map {
    asteroids: Vec<Vec<bool>>,
}

impl TryFrom<&str> for Map {
    type Error = ();

    fn try_from(input: &str) -> Result<Map, Self::Error> {
        let mut asteroids = Vec::new();

        for line in input.lines() {
            let mut row = Vec::new();
            for c in line.chars() {
                let asteroid = match c {
                    '#' => true,
                    '.' => false,
                    _ => return Err(()),
                };
                row.push(asteroid);
            }
            asteroids.push(row);
        }

        Ok(Map { asteroids })
    }
}

impl Map {
    pub fn contains_asteroid(&self, x: usize, y: usize) -> bool {
        self.asteroids[y][x]
    }

    pub fn asteroids(&self) -> impl IntoIterator<Item = (usize, usize)> {
        let asteroid_vec: Vec<_> = self
            .asteroids
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(move |(x, &col)| if col { Some((x, y)) } else { None })
            })
            .collect();
        asteroid_vec
    }

    fn can_see(&self, x1: usize, y1: usize, x2: usize, y2: usize) -> bool {
        let dx = x2 as isize - x1 as isize;
        let dy = y2 as isize - y1 as isize;

        // step by the minimal rise/run that will reach (2) from (1)
        let factor = gcd(dx.abs() as usize, dy.abs() as usize);
        let dx = dx / factor as isize;
        let dy = dy / factor as isize;

        let (mut x, mut y) = (x1 as isize, y1 as isize);

        while (x, y) != (x2 as isize, y2 as isize) {
            x += dx;
            y += dy;

            if self.contains_asteroid(x as usize, y as usize) {
                return false;
            }
        }

        true
    }
}

fn gcd(a: usize, b: usize) -> usize {
    if a > b {
        return gcd(b, a);
    }

    if a == 0 {
        return b;
    }

    gcd(b % a, a)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parser() {
        let map: Map = ".#..#
.....
#####
....#
...##
"
        .try_into()
        .expect("could not parse map");

        assert!(!map.can_see(3, 4, 1, 0));
        assert!(!map.can_see(1, 0, 3, 4));
        assert!(map.can_see(3, 4, 4, 0));
        assert!(map.can_see(4, 0, 3, 4));
    }
}
