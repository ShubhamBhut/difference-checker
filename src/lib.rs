pub fn diff_strings_char(string1: &str, string2: &str) -> Vec<(usize, usize, char)> {
    let mut diffs = Vec::new();
    let lines1: Vec<&str> = string1.lines().collect();
    let lines2: Vec<&str> = string2.lines().collect();
    let mut line = 0;
    let mut character = 0;

    for (i, (line1, line2)) in lines1.iter().zip(lines2.iter()).enumerate() {
        let chars1: Vec<char> = line1.chars().collect();
        let chars2: Vec<char> = line2.chars().collect();
        let min_len = chars1.len().min(chars2.len());

        for j in 0..min_len {
            if chars1[j] != chars2[j] {
                diffs.push((line, character, chars2[j]));
            }
            character += 1;
        }

        if chars1.len() != chars2.len() {
            let max_len = chars1.len().max(chars2.len());
            let remaining_chars = if chars1.len() > chars2.len() {
                &chars1[min_len..max_len]
            } else {
                &chars2[min_len..max_len]
            };

            for ch in remaining_chars {
                diffs.push((line, character, *ch));
                character += 1;
            }
        }

        line += 1;
        character = 0;
    }

    diffs
}

pub fn diff_strings_word(string1: &str, string2: &str) -> Vec<(usize, Vec<String>)> {
    let mut diffs = Vec::new();
    let words1: Vec<&str> = string1.split_whitespace().collect();
    let words2: Vec<&str> = string2.split_whitespace().collect();
    let mut line = 0;

    for (i, (word1, word2)) in words1.iter().zip(words2.iter()).enumerate() {
        if word1 != word2 {
            let mut word_diff = Vec::new();
            word_diff.push(String::from(*word2));
            diffs.push((line, word_diff));
        }
        line += 1;
    }

    diffs
}

