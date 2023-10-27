//! Run this file with `cargo test --test 02_strip_prefix`.

// Implement a function called `strip_prefix`, which will take two strings (`needle` and `prefix`).
// It will return a substring of `needle` starting at the first character that does not begin with
// any character in `prefix`.
// See tests for examples.
// Hint: you can use `string.chars()` for iterating the Unicode characters of a string.

/// Below you can find a set of unit tests.
#[cfg(test)]
mod tests {
    use crate::strip_prefix;

    #[test]
    fn strip_prefix_basic() {
        assert_eq!(strip_prefix("foobar", "of"), "bar");
    }

    #[test]
    fn strip_prefix_full_result() {
        assert_eq!(strip_prefix("foobar", "x"), "foobar");
    }

    #[test]
    fn strip_prefix_empty_result() {
        assert_eq!(strip_prefix("foobar", "fbaro"), "");
    }

    #[test]
    fn strip_prefix_lifetime_check() {
        let needle = "foobar";
        let prefix = String::from("f");
        let result = strip_prefix(needle, &prefix);
        // Uncomment the `drop(prefix)` line.
        // Does the test still work? If not, fix `strip_prefix`!
        //HIDE
        drop(prefix);
        //-HIDE
        // drop(prefix);
        assert_eq!(result, "oobar");
    }
}
