pub fn remove_first_two_lines(input: &str) -> String {
    let mut lines = input.lines();
    let mut result = String::new();

    // Skip the first two lines
    lines.nth(0);
    lines.nth(0);

    for line in lines {
        result.push_str(line);
        result.push('\n');
    }

    result
}