pub mod base;
pub mod caesar;
pub mod constants;
pub mod extract;
pub mod general;
pub mod manipulation;
pub mod morse;
pub mod services;
pub mod utils;
pub mod xor;
// pub mod hasher;
// pub mod analyze;

// TODO: add test cases for each module.
// TODO: add better corner cases for each function.

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
        assert_eq!(
            manipulation::reverse("A retarded SENTENCE!!"),
            "!!ECNETNES dedrater A"
        );
    }

    // general.rs
    #[test]
    fn check_general() {
        assert_eq!(general::a1z26("14-9-3-5"), "nice");
        assert_eq!(general::atbash("nice"), "mrxv");
        assert_eq!(general::ascii("65 65"), "AA");
        assert!(
            general::bacon("AAABB AAABA ABBAB AAABB AABAA AAAAB AAAAA AAABA ABBAB ABBAA")
                .contains("dcodebacon")
        );
        assert_eq!(
            general::url_encode("https://www.twitter.com"),
            "https%3A%2F%2Fwww.twitter.com"
        );
        assert_eq!(
            general::url_decode("https%3A%2F%2Fwww.twitter.com"),
            "https://www.twitter.com"
        );
    }

    // xor.rs
    #[test]
    fn check_xor() {
        assert!(xor::hex_x_byte(
            "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736"
        )
        .contains("Cooking MC's like a pound of bacon"));
        assert_eq!(
            xor::hex_x_hex("6578616d706c6520737472696e67", "6e696365"),
            "0b1102081e0506451d1d110c000e"
        );
    }

    // Check services.rs
    #[test]
    fn check_services() {
        assert_eq!(services::factordb("12"), "2 3");
    }

    #[test]
    fn check_extractor() {
        assert!(extract::extractor("email", "Cargo.toml").contains("aquibjavedt007@gmail.com"));
        assert!(extract::extractor("phone", "todo.norg").contains("2333897193"));
        assert!(extract::extractor("ip", "todo.norg").contains("1.1.1.1"));
    }

    #[test]
    fn check_utils() {
        assert!(utils::is_all_in("234234234", &['2', '3', '4']));
        assert!(utils::is_hex_repr("deadbeef"));
    }
}
