// TODO: 1. normal 2. match case 3. Match Whole Word 4. Regex
pub mod find {
    #[derive(Debug)]
    pub struct FindResult {
        line: usize,
        start: usize,
        end: usize,
    }

    fn prefix_suffix_max_len_table(pattern: &Vec<char>) -> Vec<usize> {
        // String matching should consider Unicode characters
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

    fn kmp(haystack: &Vec<char>, pattern: &Vec<char>, table: &Vec<usize>) -> Option<Vec<usize>> {
        let haystack_size = haystack.len();
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

    fn find_match_case(src: &Vec<Vec<char>>, pattern: &Vec<char>) -> Option<Vec<FindResult>> {
        let table = prefix_suffix_max_len_table(pattern);

        let mut matching: Vec<FindResult> = Vec::new();

        let mut i = 1;
        for line in src {
            match kmp(line, pattern, &table) {
                Some(line_res) => {
                    for position in line_res {
                        matching.push(FindResult {
                            line: i,
                            start: position,
                            end: position + pattern.len(),
                        })
                    }
                    i += 1;
                }
                None => ()
            }
        }

        if matching.is_empty() {
            None
        } else {
            Some(matching)
        }
    }

    // Disclaimer: I am not sure about capitalized Cyrill characters
    pub fn find(src: &str,
                pattern: &str,
                is_match_case: bool,
                is_words: bool,
                is_regex: bool) -> Option<Vec<FindResult>> {
        if is_regex {
            None
        } else {
            let mut lines = src.to_string();
            let mut pat = pattern.to_string();

            if !is_match_case {
                lines = lines.to_ascii_lowercase();
                pat = pat.to_ascii_lowercase();
            }

            let lines: Vec<&str> = lines.split('\n').collect();
            let lines: Vec<Vec<char>> = lines.iter().map(|x| x.chars().collect()).collect();
            let pat: Vec<char> = pat.chars().collect();

            let result = find_match_case(&lines, &pat);

            if is_words {
                let side_check = |x: &FindResult| {
                    let line_num = x.line - 1;

                    if (x.start >= 2
                        && (lines[line_num][x.start - 2].is_alphanumeric()
                        || lines[line_num][x.start - 2] == '_')
                    ) || (x.end - 1 < lines[line_num].len()
                        && (lines[line_num][x.end - 1].is_alphanumeric()
                        || lines[line_num][x.end - 1] == '_')) {
                        return false;
                    }

                    return true;
                };

                match result {
                    Some(v) => {
                        let tmp: Option<Vec<FindResult>> = Some(v.into_iter().filter(side_check).collect());
                        return tmp;
                    }
                    None => ()
                }
            }

            result
        }
    }
}
