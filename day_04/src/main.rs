fn main() {
    do_main();
}

fn do_main() {
    let mut count = 0;

    for i in 123257..=647015 {
        if check(i) {
            count += 1;
        }
    }

    println!("Valid passwords: {}", count);
    assert_eq!(count, 2220);
}

fn check(i: usize) -> bool {
    let stringy = format!("{}", i);

    adjacent_digits(&stringy) && in_order(&stringy)
}

fn adjacent_digits(i: &str) -> bool {
    for (i, j) in i.chars().zip(i.chars().skip(1)) {
        if i == j {
            return true;
        }
    }

    false
}

fn in_order(i: &str) -> bool {
    let mut last = i
        .chars()
        .nth(0)
        .expect("in_order check for an empty string");

    for c in i.chars().skip(1) {
        if c < last {
            return false;
        }
        last = c;
    }

    true
}

#[cfg(test)]
mod test {
    #[test]
    fn check() {
        assert!(super::check(122345));
        assert!(super::check(111123));
        assert!(super::in_order("135679"));
        assert!(super::check(111111));
        assert!(!super::check(223450));
        assert!(!super::check(123789));
    }

    #[test]
    fn main() {
        super::do_main();
    }
}