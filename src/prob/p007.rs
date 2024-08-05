const N: u32 = 10_001;

fn sf(n: u64) -> u64 {
    let sq = (n as f64).sqrt() as u64;

    for i in 2..=sq {
        if n % i == 0 {
            return i;
        }
    }

    n
}

fn isp(n: u64) -> bool {
    n == sf(n)
}

pub fn aop() -> u64 {
    let mut c = 1;
    let mut p = 2;

    while c <= N {
        if isp(p) {
            c += 1;
        }
        p += 1;
    }

    p - 1
}

mod tests {
    #[test]
    fn test_aop() {
        assert_eq!(crate::prob::p007::aop(), 104743);
    }
}
