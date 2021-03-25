use uwuifier::uwu_ify_sse;

fn round_up(a: usize, b: usize) -> usize {
    (a + b - 1) / b * b
}

pub fn uwuify(input: &str) -> String {
    // should be small enough so stuff fits in L1/L2 cache
    // but big enough so each thread has enough work to do
    const LEN: usize = 1 << 16;

    let bytes = input.as_bytes();
    let bytes_buf_len = round_up(bytes.len(), 16);
    let bytes_buf = vec![0u8; bytes_buf_len];
    let mut temp_bytes1 = vec![0u8; LEN * 16];
    let mut temp_bytes2 = vec![0u8; LEN * 16];

    let res = uwu_ify_sse(bytes, bytes.len(), &mut temp_bytes1, &mut temp_bytes2);

    String::from_utf8_lossy(res).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uwufiy() {
        let input = "this is an extremely important bot and I expect it to have a major impact on human culture.";
        let expected = "foo";
        let output = uwuify(input);

        assert_eq!(expected, output);
    }
}
