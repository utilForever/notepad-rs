// TODO: 1. normal 2. match case 3. Match Whole Word 4. Regex
pub mod find {
    #[derive(Debug)]
    pub struct FindResult {
        line: usize,
        start: usize,
        end: usize,
    }

    fn prefix_suffix_max_len_table(pattern: &str) -> Vec<usize> {
        // String matching should consider Unicode characters
        let pattern: Vec<char> = pattern.chars().collect();
        let pattern_size = pattern.len();
        let mut table: Vec<usize> = vec![0; pattern_size];

        let mut j = 0;
        for i in 1..pattern_size {
            while j > 0 && pattern[j] != pattern[i] {
                j = table[j - 1];
            }
            if pattern[j] == pattern[i] {
                table[i] = j + 1;
                j += 1;
            }
        }

        table
    }

    fn kmp(haystack: &str, pattern: &str, table: &Vec<usize>) -> Option<Vec<usize>> {
        let haystack: Vec<char> = haystack.chars().collect();
        let haystack_size = haystack.len();
        let pattern: Vec<char> = pattern.chars().collect();
        let pattern_size = pattern.len();

        let mut results: Vec<usize> = Vec::new();

        let mut j = 0;

        for i in 0..haystack_size {
            while j > 0 && pattern[j] != haystack[i] {
                j = table[j - 1];
            }
            if pattern[j] == haystack[i] {
                if j == pattern_size - 1 {
                    results.push(i + 2 - pattern_size);
                    j = table[j];
                } else {
                    j += 1;
                }
            }
        }

        if results.is_empty() {
            None
        } else {
            Some(results)
        }
    }

    pub fn find_texts_in_file(src: &str, pattern: &str) -> Option<Vec<FindResult>> {
        let lines: Vec<&str> = src.split("\n").collect();
        let table = prefix_suffix_max_len_table(pattern);

        let mut matching: Vec<FindResult> = Vec::new();

        let i = 0;
        for line in lines {
            match kmp(line, pattern, &table) {
                Some(line_res) => {
                    for position in line_res {
                        matching.push(FindResult {
                            line: i,
                            start: position,
                            end: position + pattern.len(),
                        })
                    }
                }
                None => continue
            }
        }

        if matching.is_empty() {
            None
        } else {
            Some(matching)
        }
    }
}
