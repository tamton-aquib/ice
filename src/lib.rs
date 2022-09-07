pub mod base;
pub mod caesar;
pub mod general;
pub mod manipulation;
pub mod morse;
pub mod xor;
// pub mod hasher;

// TODO: add test cases for each module.

#[cfg(test)]
mod tests {
    use super::*;

    // caesar.rs
    #[test]
    fn check_rot() {
        assert_eq!(caesar::rot13("nice"), "avpr");
        assert!(caesar::caesar("nice").contains("[25] mhbd"));
    }

    // morse.rs
    #[test]
    fn check_morse() {
        assert_eq!(morse::morse_encode("nice"), "_. .. _._. . ");
        assert_eq!(morse::morse_decode("_. .. _._. . "), "nice");
        assert_eq!(morse::morse("_. .. _._. ."), "nice");
    }

    // base.rs
    #[test]
    fn check_base() {
        assert_eq!(base::b64("bmljZQ=="), "nice");
        assert_eq!(base::b32("NZUWGZI="), "nice");
        assert_eq!(base::hexadecimal("6e696365"), "nice");
        assert_eq!(base::octal("156 151 143 145 "), "nice");
        assert_eq!(base::binary("01101110 01101001 01100011 01100101 "), "nice");
    }

    // manipulation.rs
    #[test]
    fn check_manipulation() {
        assert_eq!(
            manipulation::remove_whitespace("an example with    whitespace"),
            "anexamplewithwhitespace"
        );
        assert_eq!(
            manipulation::lower("A retarded SENTENCE!!"),
            "a retarded sentence!!"
        );
        assert_eq!(
            manipulation::upper("A retarded SENTENCE!!"),
            "A RETARDED SENTENCE!!"
        );
    }

    #[test]
    fn check_general() {
        assert_eq!(general::a1z26("14-9-3-5"), "nice");
        assert_eq!(general::atbash("nice"), "mrxv")
    }
}
