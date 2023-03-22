pub fn simple_multiplication(number: u8) -> u8 {
    if number % 2 == 0 { number * 8 } else { number * 9 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(simple_multiplication(1), 9);
        assert_eq!(simple_multiplication(2), 16);
        assert_eq!(simple_multiplication(4), 32);
        assert_eq!(simple_multiplication(5), 45);
    }
}
