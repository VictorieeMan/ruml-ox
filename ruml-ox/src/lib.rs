pub fn add(left: usize, right: usize) -> usize {
    left + right
}

/// Returns an epic greeting.
pub fn hello() -> String {
    "Well, hello there!".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn it_works_2() {
        assert_eq!(hello(), "Well, hello there!");
    }
}