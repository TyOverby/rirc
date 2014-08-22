use std::collections::ringbuf::RingBuf;
use std::collections::Deque;

pub struct LineBuffer {
    lines: Vec<String>
}

fn split_line_on_width<'a>(line: &'a str, width: uint) -> Vec<&'a str> {
    fn helper<'a>(line: &'a str, width: uint, acc: &mut Vec<&'a str>) {
        if line.len() <= width {
            acc.push(line.trim_chars(' '));
            return;
        }

        let head = line.slice_to(width);
        let last_space = head.rfind(' ');

        match last_space {
            Some(n) => {
                acc.push(line.slice_to(n).trim_chars(' '));
                helper(line.slice_from(n + 1), width, acc);
            }
            None => {
                acc.push(head.trim_chars(' '));
                helper(line.slice_from(width), width, acc);
            }
        }
    }
    let mut vec = Vec::new();
    helper(line, width, &mut vec);
    vec
}

impl LineBuffer {
    pub fn new() -> LineBuffer {
        LineBuffer{ lines: vec![] }
    }
    pub fn add(&mut self, line: String) {
        self.lines.push(line);
    }

    pub fn last_n_truncated(&self, n: uint, width: uint) -> RingBuf<&str> {
        let mut truncated_lines = RingBuf::new();
        truncated_lines.reserve_exact(n);

        for line in self.lines.iter() {
            for trunc_line in split_line_on_width(line.as_slice(), width).iter() {
                if truncated_lines.len() == n {
                    truncated_lines.pop_front();
                }
                truncated_lines.push(*trunc_line);
            }
        }

        truncated_lines
    }
}

#[test]
fn test_split() {
    let input1 = "this is a test";
    assert_eq!(split_line_on_width(input1, 6),
        vec!("this", "is a", "test"));

    let input2 = "abcdefghijklmnop";
    assert_eq!(split_line_on_width(input2, 6),
        vec!("abcdef", "ghijkl", "mnop"));

    let input2 = "abcde  fgh ijkl mnop  ";
    assert_eq!(split_line_on_width(input2, 6),
        vec!("abcde", "fgh", "ijkl", "mnop"));
}
