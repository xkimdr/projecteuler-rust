const N: u64 = 600_851_475_143;

fn sf(n: u64) -> u64 {
    let sq = (n as f64).sqrt() as u64;

    for i in 2..=sq {
        if n % i == 0 {
            return i;
        }
    }

    n
}

pub fn aop() -> u64 {
    let mut n = N;
    let mut f = N;

    while n != 1 {
        f = sf(n);
        n /= f;
    }

    f
}

mod tests {
    #[test]
    fn test_aop() {
        assert_eq!(crate::prob::p003::aop(), 6857);
    }
}
