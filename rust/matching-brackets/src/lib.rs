pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = Vec::new();

    for c in string.chars() {
        if "({[".contains(c) {
            stack.push(c);
        } else if ")}]".contains(c) {
            if let Some(open) = stack.pop() {
                if open
                    != match c {
                        ']' => '[',
                        ')' => '(',
                        _ => '{',
                    }
                {
                    return false;
                }
            } else {
                return false;
            }
        }
    }

    stack.is_empty()
}
