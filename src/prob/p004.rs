fn isp(n: u32) -> bool {
    let s = n.to_string();
    let r = s.chars().rev().collect::<String>();
    s == r
}

pub fn aop() -> u32 {
    let mut p = 0;

    for i in (100..=999).rev() {
        for j in (10..=90).rev() {
            let m = 11 * j * i;
            if m > p && isp(m) {
                p = m;
            }
        }
    }

    p
}

mod tests {
    #[test]
    fn test_aop() {
        assert_eq!(crate::prob::p004::aop(), 906609);
    }
}
