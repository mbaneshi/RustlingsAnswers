// iterators4.rs

pub fn factorial(num: u64) -> u64 {
    /*if num == 1 {
        1
    } else {
        num * factorial(num - 1)
    }*/
    (1..num + 1).fold(1, |acc, x| acc * x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}
