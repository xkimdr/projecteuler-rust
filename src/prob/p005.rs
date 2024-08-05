const F: u32 = 1;
const U: u32 = 20;

fn gcd(mut x: u32, mut y: u32) -> u32 {
    while y != 0 {
        let t = y;
        y = x % y;
        x = t;
    }

    x
}

fn lcm(x: u32, y: u32) -> u32 {
    x * (y / gcd(x, y))
}

pub fn aop() -> u32 {
    let mut l = 1;
    let mut f = F;

    while f <= U {
        l = lcm(l, f);
        f += 1;
    }

    l
}

mod tests {
    #[test]
    fn test_aop() {
        assert_eq!(crate::prob::p005::aop(), 232792560);
    }
}
