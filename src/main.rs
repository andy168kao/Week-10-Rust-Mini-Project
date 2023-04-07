fn main() {
    let digits = "23".to_string();
    let result = letter_combinations(digits);
    println!("{:?}", result);
}

fn letter_combinations(digits: String) -> Vec<String> {
    let digit_to_letter: Vec<&str> = vec![
        "", "", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"
    ];

    let mut result = Vec::new();

    if digits.is_empty() {
        return result;
    }

    let mut current = String::new();
    dfs(&digit_to_letter, &digits, &mut result, &mut current, 0);

    result
}

fn dfs(
    digit_to_letter: &[&str],
    digits: &str,
    result: &mut Vec<String>,
    current: &mut String,
    index: usize,
) {
    if index == digits.len() {
        result.push(current.clone());
        return;
    }

    let digit = digits.chars().nth(index).unwrap() as usize - '0' as usize;
    let letters = digit_to_letter[digit];

    for letter in letters.chars() {
        current.push(letter);
        dfs(digit_to_letter, digits, result, current, index + 1);
        current.pop();
    }
}
