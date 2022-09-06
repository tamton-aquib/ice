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
    }
    #[test]
    fn check_caesar() {
        assert!(caesar::caesar("nice").contains("[25] mhbd"))
    }

    // morse.rs
    #[test]
    fn check_morse_encode() {
        assert_eq!(morse::morse_encode("nice"), "_. .. _._. . ")
    }
    #[test]
    fn check_morse_decode() {
        assert_eq!(morse::morse_decode("_. .. _._. . "), "nice")
    }

    // base.rs
    #[test]
    fn check_b64() {
        assert_eq!(base::b64("bmljZQ=="), "nice")
    }
    #[test]
    fn check_b32() {
        assert_eq!(base::b32("NZUWGZI="), "nice")
    }
    #[test]
    fn check_b16() {
        assert_eq!(base::hexadecimal("6e696365"), "nice")
    }
    #[test]
    fn check_b8() {
        assert_eq!(base::octal("156 151 143 145 "), "nice")
    }
    #[test]
    fn check_b2() {
        assert_eq!(base::binary("01101110 01101001 01100011 01100101 "), "nice")
    }

    // manipulation.rs
    #[test]
    fn check_remove_whitespace() {
        assert_eq!(
            manipulation::remove_whitespace("an example with    whitespace"),
            "anexamplewithwhitespace"
        )
    }
    #[test]
    fn check_lower() {
        assert_eq!(
            manipulation::lower("A retarded SENTENCE!!"),
            "a retarded sentence!!"
        )
    }
    #[test]
    fn check_upper() {
        assert_eq!(
            manipulation::upper("A retarded SENTENCE!!"),
            "A RETARDED SENTENCE!!"
        )
    }
}
