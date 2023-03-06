pub fn disemvowel(s: &str) -> String {
    s.replace(&['a', 'A', 'e', 'E', 'i', 'I', 'o', 'O', 'u', 'U'], "")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_expected() {
        assert_eq!(disemvowel("This website is for losers LOL!"), "Ths wbst s fr lsrs LL!");
        assert_eq!(disemvowel("This kata sucks LOL!"), "Ths kt scks LL!");
        assert_eq!(disemvowel("Crap"), "Crp");
    }
}
