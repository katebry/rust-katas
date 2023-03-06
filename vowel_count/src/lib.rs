pub fn get_count(string: &str) -> usize {
    string
        .chars()
        .filter(|&c| "aeiou".contains(c))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn my_tests() {
        assert_eq!(get_count("abracadabra"), 5);
        assert_eq!(get_count("abra"), 2);
        assert_eq!(get_count("zzzzzzz"), 0);
        assert_eq!(get_count("aeiou"), 5);
    }
}
