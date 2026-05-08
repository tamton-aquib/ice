pub mod analysis;
pub mod app;
pub mod base;
pub mod ciphers;
pub mod hasher;
pub mod utils;

#[cfg(test)]
mod tests {
    use crate::{
        analysis::{extract, manipulation},
        base::base::{b32, b45, b58, b62, b64, b85, binary, hexadecimal, octal},
        ciphers::general::general,
        ciphers::xor,
        ciphers::{caesar, morse},
        hasher::hasher,
        utils::{services, utils},
    };

    #[test]
    fn check_rot() {
        assert_eq!(caesar::rot13("nice"), "avpr");
        assert!(caesar::caesar("nice", None)
            .unwrap()
            .contains("[25] mhbd"));
    }

    #[test]
    fn check_morse() {
        assert_eq!(morse::morse_encode("nice"), "_. .. _._. . ");
        assert_eq!(morse::morse_decode("_. .. _._. . "), "nice");
        assert_eq!(morse::morse("_. .. _._. ."), "nice");
    }

    #[test]
    fn check_base() {
        assert_eq!(b64("bmljZQ==").unwrap(), "nice");
        assert_eq!(b32("NZUWGZI=").unwrap(), "nice");
        assert_eq!(hexadecimal("6e696365").unwrap(), "nice");
        assert_eq!(octal("156 151 143 145 ").unwrap(), "nice");
        assert_eq!(binary("01101110 01101001 01100011 01100101 ").unwrap(), "nice");
    }

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

    #[test]
    fn check_general() {
        assert_eq!(general::a1z26("14-9-3-5").unwrap(), "nice");
        assert_eq!(general::atbash("nice").unwrap(), "mrxv");
        assert_eq!(general::ascii("65 65").unwrap(), "AA");
        assert!(general::bacon("AAABB AAABA ABBAB AAABB AABAA AAAAB AAAAA AAABA ABBAB ABBAA")
            .unwrap()
            .contains("dcodebacon"));
        assert_eq!(
            general::url_encode("https://www.twitter.com").unwrap(),
            "https%3A%2F%2Fwww.twitter.com"
        );
        assert_eq!(
            general::url_decode("https%3A%2F%2Fwww.twitter.com").unwrap(),
            "https://www.twitter.com"
        );
    }

    #[test]
    fn check_xor() {
        assert!(
            xor::hex_x_byte(
                "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736"
            )
            .unwrap()
            .contains("Cooking MC's like a pound of bacon")
        );
        assert_eq!(
            xor::hex_x_hex("6578616d706c6520737472696e67", "6e696365").unwrap(),
            "0b1102081e0506451d1d110c000e"
        );
    }

    #[test]
    fn check_base_new() {
        assert!(b45("hello").is_ok());
        assert!(b58("hello").is_ok());
        assert!(b62("hello").is_ok());
        assert!(b85("hello").is_ok());
    }

    #[test]
    fn check_utils() {
        assert!(utils::is_all_in("234234234", &['2', '3', '4']));
        assert!(utils::is_hex_repr("deadbeef"));
    }

    #[test]
    fn check_hasher() {
        assert!(hasher::md5("hello world")
            .unwrap()
            .contains("5eb63bbbe01eeed093cb22bb8f5acdc3"));
        assert!(hasher::sha1("hello world")
            .unwrap()
            .contains("2aae6c35c94fcfb415dbe95f408b9ce91ee846ed"));
        assert!(hasher::sha256("hello world")
            .unwrap()
            .contains("b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"));
        assert!(hasher::sha512("hello world")
            .unwrap()
            .contains("309ecc489c12d6eb4cc40f50c902f2b4d0ed77ee511a7c7a9bcd3ca86d4cd86f989dd35bc5ff499670da34255b45b0cfd830e81f605dcf7dc5542e93ae9cd76f"));
    }

    #[test]
    fn check_services() {
        assert_eq!(services::factordb("12").unwrap(), "2 3");
    }

    #[test]
    fn check_extractor() {
        assert!(extract::extractor("email", "Cargo.toml")
            .unwrap()
            .contains("aquibjavedt007@gmail.com"));
    }
}
