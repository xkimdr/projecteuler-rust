const U: u32 = 999;
const X: u32 = 3;
const Y: u32 = 5;

fn son(n: u32) -> u32 {
    n * (n + 1) / 2
}

fn nom(n: u32) -> u32 {
    U / n
}

fn som(n: u32) -> u32 {
    n * son(nom(n))
}

pub fn aop() -> u32 {
    som(X) + som(Y) - som(X * Y)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_aop() {
        assert_eq!(crate::prob::p001::aop(), 233168);
    }
}
