pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = Vec::with_capacity(string.len() / 2);
    for c in string.chars() {
        match c {
            // opening first
            '[' => {
                stack.push(']');
            }
            '{' => {
                stack.push('}');
            }
            '(' => {
                stack.push(')');
            }
            ']' | '}' | ')' => {
                if stack.pop() != Some(c) {
                    return false;
                }
            }
            _ => (),
        }
    }

    stack.is_empty()
}
