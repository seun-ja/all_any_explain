#![cfg(test)]
fn all_predicate(input: &str) -> bool {
    input.chars().all(|char| char.is_alphabetic())
}

fn any_predicate(input: &str) -> bool {
    input.chars().any(|char| char.is_alphabetic())
}

#[cfg(test)]
mod test {
    use crate::{all_predicate, any_predicate};

    #[test]
    fn test_all_predicate() {
        let input = "PREDICATE";
        assert!(all_predicate(input))
    }

    #[test]
    fn test_all_predicate_fails() {
        let input = "2G45667";
        assert!(!all_predicate(input))
    }

    #[test]
    fn test_any_predicate() {
        let input = "2G45667";
        assert!(any_predicate(input))
    }

    #[test]
    fn test_any_predicate_fails() {
        let input = "2345667";
        assert!(!any_predicate(input))
    }
}
