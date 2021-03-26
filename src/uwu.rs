use uwuifier::uwuify_str_sse;

pub fn uwuify(input: &str) -> String {
    uwuify_str_sse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uwufiy() {
        let input = "this is an extremely important bot and I expect it to have a major impact on human culture.";
        let expected = "this is an extwemewy i-impowtant bot a-and i expect it to have a majow i-impact on human cuwtuwe.";

        let output = uwuify(input);

        assert_eq!(expected, output);
    }
}
