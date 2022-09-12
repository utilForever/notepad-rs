mod edit;

#[cfg(test)]
mod test {
    // TODO: Complete unit test codes
    use super::edit::find;
    use super::edit::find::FindResult;

    #[test]
    fn find_match_case_works() {
        let text = "    #[test]
fn it_works() {
        let result = 2 + 2;\n
        assert_eq!(result, 4);\n
    }";
        let v = find::find(text, "result", true, false, false);
        assert_eq!(v, vec![FindResult { line: 1, start: 13, end: 19 },
                           FindResult { line: 2, start: 20, end: 26 }]);
    }

    #[test]
    fn normal_find_works() {
        let v = find::find("    #[test]\n
    fn it_works() {\n
        let result = 2 + 2;\n
        assert_eq!(result, 4);\n
    }", "ResUlT", false, false, false);

        assert_eq!(v, vec![FindResult { line: 1, start: 13, end: 19 },
                           FindResult { line: 2, start: 20, end: 26 }]);
    }

    #[test]
    fn normal_find_with_korean_works() {
        let v = find::find("안녕하세요.\n\
    제 이름은 jeff park입니다.", "JefF", false, false, false);

        assert_eq!(v, vec![FindResult { line: 1, start: 7, end: 11 }]);
    }

    #[test]
    fn find_words_works() {
        let v = find::find("find find_find", "find", false, true, false);

        assert_eq!(v, vec![FindResult { line: 1, start: 1, end: 5 }]);
    }
}