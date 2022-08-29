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
        match find::find_match_case("    #[test]\n
    fn it_works() {\n
        let result = 2 + 2;\n
        assert_eq!(result, 4);\n
    }", "result") {
            Some(res) => {
                for i in res {
                    println!("{:?}", i)
                }
            }
            _ => ()
        }
    }

    #[test]
    fn normal_find_works() {
        println!("===normal_find_works===");
        match find::find("    #[test]\n
    fn it_works() {\n
        let result = 2 + 2;\n
        assert_eq!(result, 4);\n
    }", "ResUlT") {
            Some(res) => {
                for i in res {
                    println!("{:?}", i)
                }
            }
            _ => ()
        }
    }

    #[test]
    fn normal_find_with_korean_works() {
        println!("===normal_find_with_korean_works===");
        match find::find("안녕하세요.\n\
    제 이름은 jeff park입니다.", "JefF") {
            Some(res) => {
                for i in res {
                    println!("{:?}", i)
                }
            }
            _ => ()
        }
    }
}