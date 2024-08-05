const U: u32 = 4_000_000;

fn n(x: u32, y: u32) -> u32 {
    x + 4 * y
}

pub fn aop() -> u32 {
    let mut x = 2;
    let mut y = 8;
    let mut z = n(x, y);
    let mut sum = x + y;

    while z <= U {
        sum += z;
        x = y;
        y = z;
        z = n(x, y);
    }

    sum
}

mod tests {
    #[test]
    fn test_aop() {
        assert_eq!(crate::prob::p002::aop(), 4613732);
    }
}
