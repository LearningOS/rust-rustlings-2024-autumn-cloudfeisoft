#[derive(Debug)]
struct Stack<T> {
    data: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Self {
            data: Vec::new(),
        }
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn clear(&mut self) {
        self.data.clear();
    }

    fn push(&mut self, val: T) {
        self.data.push(val);
    }

    fn pop(&mut self) -> Option<T> {
        if let Some(val) = self.data.pop() {
            Some(val)
        } else {
            None
        }
    }

    fn peek(&self) -> Option<&T> {
        self.data.last()
    }

    fn peek_mut(&mut self) -> Option<&mut T> {
        self.data.last_mut()
    }

    fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    fn iter(&self) -> Iter<T> {
        Iter {
            iter: self.data.iter(),
        }
    }

    fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            iter: self.data.iter_mut(),
        }
    }
}

struct IntoIter<T>(Stack<T>);

impl<T: Clone> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.0.is_empty() {
            self.0.pop()
        } else {
            None
        }
    }
}

struct Iter<'a, T: 'a> {
    iter: std::slice::Iter<'a, T>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

struct IterMut<'a, T: 'a> {
    iter: std::slice::IterMut<'a, T>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

fn bracket_match(bracket: &str) -> bool {
    let mut stack = Stack::new();

    // HashMap to hold the mapping of closing to opening brackets
    let mut seen_brackets: std::collections::HashMap<char, char> = std::collections::HashMap::new();
    seen_brackets.insert(')', '(');
    seen_brackets.insert(']', '[');
    seen_brackets.insert('}', '{');

    for bracket in bracket.chars() {
        match bracket {
            '(' | '[' | '{' => {
                stack.push(bracket);
            }
            ')' | ']' | '}' => {
                if stack.is_empty() {
                    return false;
                }
                match stack.pop() {
                    Some(top) => {
                        if seen_brackets[&bracket] != top {
                            return false;
                        }
                    },
                    None => return false,
                }
            }
            _ => {} // Ignore any characters that are not brackets
        }
    }

    stack.is_empty()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bracket_matching_1() {
        let s = "(2+3){func}[abc]";
        assert_eq!(bracket_match(s), true);
    }

    #[test]
    fn bracket_matching_2() {
        let s = "(2+3)*(3-1";
        assert_eq!(bracket_match(s), false);
    }

    #[test]
    fn bracket_matching_3() {
        let s = "{{([])})}";
        assert_eq!(bracket_match(s), true);
    }

    #[test]
    fn bracket_matching_4() {
        let s = "{{(}[)]}";
        assert_eq!(bracket_match(s), false);
    }

    #[test]
    fn bracket_matching_5() {
        let s = "[[[]]]]]]]]]]";
        assert_eq!(bracket_match(s), false);
    }

    #[test]
    fn bracket_matching_6() {
        let s = "";
        assert_eq!(bracket_match(s), true);
    }
}