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
    fn find_works() {
        match find::find_texts_in_file("    #[test]\n
    fn it_works() {\n
        let result = 2 + 2;\n
        assert_eq!(result, 4);\n
    }", "result") {
            Some(res) => {
                for i in res {
                    println!("{:?}", i)
                }
            },
            _ => ()
        }
    }
}