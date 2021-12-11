mod compression;

fn main() {
    compression::Compressor{};
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
    fn test_encode_decode(#[case] s: &str) {
        let plain_text = s.to_string();
        let compressor = super::compression::Compressor{}; 
        let encoded_text: Vec<(u32, u32, u8)> = compressor.compress(&plain_text);
        let decoded_text: String = compressor.decompress(encoded_text);
        assert_eq!(plain_text, decoded_text);
    }
}