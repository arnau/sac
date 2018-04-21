use ring::*;

// TODO: Generalise algorithm
pub fn digest(s: &str) -> digest::Digest {
    digest::digest(&digest::SHA256, s.as_bytes())
}

pub fn to_hex(xs: &[u8]) -> String {
    xs.iter().map(|b| format!("{:02x}", b)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn digest_baseline() {
        let raw = r#"{"field1":"a","field2":"b"}"#;
        let hash = "129332749e67eb9ab7390d7da2e88173367d001ac3e9e39f06e41690cd05e3ae";
        let d = digest(raw);
        let bs = test::from_hex(hash).unwrap();

        assert_eq!(d.as_ref(), &bs[..]);
        assert_eq!(&to_hex(d.as_ref()), hash);
    }
}
