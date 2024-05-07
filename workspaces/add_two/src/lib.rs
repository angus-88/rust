//! Utility library for adding two

/// adds two to a given integer and returns result
/// ## Example
/// ```
/// let result = add_two::add_two(3);
/// assert_eq!(result, 5)
/// ```
pub fn add_two(x: i32) -> i32 {
    x + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_two() {
        let result = add_two(2);
        assert_eq!(result, 4);
    }
}
