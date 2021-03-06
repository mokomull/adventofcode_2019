use std::collections::{HashMap, HashSet};

fn main() {
    do_main("inputs/day_03.txt");
}

fn do_main(path: &str) {
    let input = std::fs::read_to_string(path).expect("could not open input file");
    let lines: Vec<&str> = input.trim().lines().collect();
    assert_eq!(lines.len(), 2);

    let a = parse_sequence(lines[0]);
    let b = parse_sequence(lines[1]);

    let distance = closest_intersection(&a, &b);

    println!(
        "Intersection closest to the origin is {} distance",
        distance
    );
    assert_eq!(distance, 260);

    let min_steps = soonest_intersection(&a, &b);
    println!("Fewest steps: {}", min_steps);
    assert_eq!(min_steps, 15612);
}

#[derive(Debug, Eq, PartialEq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}
use Direction::*;

#[derive(Debug, Eq, PartialEq)]
struct Instruction {
    dir: Direction,
    count: isize,
}

fn parse_sequence(input: &str) -> Vec<Instruction> {
    let mut result = Vec::new();

    for i in input.split(',') {
        let dir = match i.chars().next() {
            Some('U') => Up,
            Some('R') => Right,
            Some('D') => Down,
            Some('L') => Left,
            _ => panic!("Unknown instruction: {:?}", i),
        };
        let count = i[1..].parse().expect("non-integer count");
        result.push(Instruction { dir, count });
    }

    result
}

fn intersect(a: &[Instruction], b: &[Instruction]) -> Vec<(isize, isize)> {
    let seen_a: HashSet<_> = follow(a).keys().cloned().collect();
    let seen_b: HashSet<_> = follow(b).keys().cloned().collect();
    seen_a.intersection(&seen_b).cloned().collect()
}

fn follow(path: &[Instruction]) -> HashMap<(isize, isize), usize> {
    let (mut x, mut y) = (0, 0);
    let mut result = HashMap::new();
    let mut steps = 0;

    for i in path {
        for _ in 0..i.count {
            steps += 1;
            match i.dir {
                Right => x += 1,
                Left => x -= 1,
                Up => y += 1,
                Down => y -= 1,
            }
            result.entry((x, y)).or_insert(steps);
        }
    }

    result
}

fn closest_intersection(a: &[Instruction], b: &[Instruction]) -> isize {
    intersect(a, b)
        .iter()
        .map(|&(x, y)| x.abs() + y.abs())
        .min()
        .unwrap()
}

fn soonest_intersection(a: &[Instruction], b: &[Instruction]) -> usize {
    let visited_a = follow(a);
    let visited_b = follow(b);

    let set_a: HashSet<_> = visited_a.keys().collect();
    let set_b: HashSet<_> = visited_b.keys().collect();

    set_a
        .intersection(&set_b)
        .map(|&&(x, y)| visited_a[&(x, y)] + visited_b[&(x, y)])
        .min()
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parser() {
        fn instruction(dir: Direction, count: isize) -> Instruction {
            Instruction { dir, count }
        }
        assert_eq!(
            parse_sequence("R8,U5,L5,D3"),
            vec![
                instruction(Right, 8),
                instruction(Up, 5),
                instruction(Left, 5),
                instruction(Down, 3),
            ]
        )
    }

    #[test]
    fn intersect() {
        let mut x = super::intersect(
            &parse_sequence("R8,U5,L5,D3"),
            &parse_sequence("U7,R6,D4,L4"),
        );
        x.sort();
        assert_eq!(x, vec![(3, 3), (6, 5)])
    }

    #[test]
    fn closest_intersection() {
        assert_eq!(
            super::closest_intersection(
                &parse_sequence("R8,U5,L5,D3"),
                &parse_sequence("U7,R6,D4,L4"),
            ),
            6
        );

        assert_eq!(
            super::closest_intersection(
                &parse_sequence("R75,D30,R83,U83,L12,D49,R71,U7,L72"),
                &parse_sequence("U62,R66,U55,R34,D71,R55,D58,R83")
            ),
            159
        );

        assert_eq!(
            super::closest_intersection(
                &parse_sequence("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51"),
                &parse_sequence("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7")
            ),
            135
        );
    }

    #[test]
    fn test_soonest_intersection() {
        assert_eq!(
            super::soonest_intersection(
                &parse_sequence("R8,U5,L5,D3"),
                &parse_sequence("U7,R6,D4,L4"),
            ),
            30
        );

        assert_eq!(
            super::soonest_intersection(
                &parse_sequence("R75,D30,R83,U83,L12,D49,R71,U7,L72"),
                &parse_sequence("U62,R66,U55,R34,D71,R55,D58,R83")
            ),
            610
        );

        assert_eq!(
            super::soonest_intersection(
                &parse_sequence("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51"),
                &parse_sequence("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7")
            ),
            410
        );
    }

    #[test]
    fn main() {
        do_main("../inputs/day_03.txt");
    }
}
