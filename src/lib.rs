pub mod base;
pub mod caesar;
pub mod general;
pub mod morse;
pub mod xor;
// pub mod hasher;

// TODO: add test cases for each module.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_rot() {
        assert_eq!(caesar::rot13("nice"), "avpr")
    }
    #[test]
    fn check_caesar() {
        assert_eq!(caesar::caesar("nice")[25], "[25] mhbd")
    }
    #[test]
    fn check_morse_encode() {
        assert_eq!(morse::morse_encode("nice"), "_. .. _._. . ")
    }
    #[test]
    fn check_morse_decode() {
        assert_eq!(morse::morse_decode("_. .. _._. . "), "nice")
    }
}
