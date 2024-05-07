//! Utility library for adding one

/// adds one to a given integer and returns result
/// ## Example
/// ```
/// let result = add_one::add_one(3);
/// assert_eq!(result, 4)
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_one() {
        let result = add_one(2);
        assert_eq!(result, 3);
    }
}
