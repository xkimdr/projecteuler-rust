const N: u32 = 100;

pub fn aop() -> u32 {
    (N - 1) * N * (N + 1) * (3 * N + 2) / 12
}

mod tests {
    #[test]
    fn test_aop() {
        assert_eq!(crate::prob::p006::aop(), 25164150);
    }
}
