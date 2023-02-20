pub fn demo(word: &str) -> bool {
    let letters: Vec<_> = word.chars().collect();
    is_palindrome(&letters) //use reference
}

fn is_palindrome(items: &[char]) -> bool {
    match items {
        [first, middle @ .., last] => first == last && is_palindrome(middle),
        [] | [_] => true,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_palindromes() {
        assert!(!demo("word"));
        assert!(demo(""));
        assert!(demo("aba"));
        assert!(!demo("abdjkre"));
        assert!(demo("oxo"));
    }
}
