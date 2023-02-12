#[allow(dead_code)]
type CommentBlock<'a> = Vec<&'a str>;

#[allow(dead_code)]
pub fn get_comment_blocks(_text: &str) -> Vec<CommentBlock> {
    vec![CommentBlock::new()]
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test_get_comment_blocks() {
        let test_values: HashMap<&str, Vec<CommentBlock>> = HashMap::from([
            ("", vec![CommentBlock::new()]),
            ("// foo\n//bar \n", vec![CommentBlock::from(["foo", "bar"])]),
            (
                "// foo\na non-comment line\n//bar \n",
                vec![CommentBlock::from(["foo"]), CommentBlock::from(["bar"])],
            ),
        ]);
        for (input, expected) in &test_values {
            let result = get_comment_blocks(input);
            assert_eq!(result, *expected);
        }
    }
}
