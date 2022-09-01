mod edit;

#[cfg(test)]
mod tests {
    use super::edit::find;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn find_match_case_works() {
        println!("===find_match_case_works===");
        let v = find::find("    #[test]\n
    fn it_works() {\n
        let result = 2 + 2;\n
        assert_eq!(result, 4);\n
    }", "result", true, false, false);
        for i in v {
            println!("{:?}", i)
        }
    }

    #[test]
    fn normal_find_works() {
        println!("===normal_find_works===");
        let v = find::find("    #[test]\n
    fn it_works() {\n
        let result = 2 + 2;\n
        assert_eq!(result, 4);\n
    }", "ResUlT", false, false, false);
        for i in v {
            println!("{:?}", i)
        }
    }

    #[test]
    fn normal_find_with_korean_works() {
        println!("===normal_find_with_korean_works===");
        let v = find::find("안녕하세요.\n\
    제 이름은 jeff park입니다.", "JefF", false, false, false);
        for i in v {
            println!("{:?}", i)
        }
    }

    #[test]
    fn find_words_works() {
        println!("===find_words_works===");
        let v = find::find("find find_find", "find", false, true, false);
        for i in v {
            println!("{:?}", i)
        }
    }
}