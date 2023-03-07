pub fn square_digits(num: u64) -> u64 {
    num
        .to_string()
        .chars()
        .map(|i| i.to_digit(10).expect("character is not a digit").pow(2).to_string())
        .collect::<String>()
        .parse()
        .expect("result isn't u64 parsable")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_square_digits() {
        assert_eq!(square_digits(9119), 811181, "\nFailed with num 9119");
        assert_eq!(square_digits(765), 493625, "\nFaled with num 765");
    }
}