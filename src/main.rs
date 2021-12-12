mod compression;


fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use rstest::*;

    #[rstest]
    #[case("")]
    #[case("s")]
    #[case("*")]
    #[case("swag")]
    #[case("aabaa")]
    #[case("ababcbababaa")]
    #[case("ababcbababacbaaa")]
    fn test_encode_decode(#[case] s: &str) {
        let plain_text = s.to_string();
        let naive_matcher = super::compression::matchers::NaiveMatcher {};
        let compressor = super::compression::Compressor{
            matcher: Box::new(naive_matcher),
        }; 
        let encoded_text = compressor.compress(&plain_text);
        println!("printing encoded text");
        for entry in &encoded_text {
            println!("({}, {}, {})", entry.0, entry.1, entry.2);
        }
        let decoded_text: String = compressor.decompress(encoded_text);
        assert_eq!(plain_text, decoded_text);
    }
}