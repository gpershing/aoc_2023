use std::collections::VecDeque;

struct LineGroupIter<TIter> {
    pub memory: VecDeque<String>,
    pub iter: TIter,
    pub empty_char: char,
    pub done: bool
}

impl<TIter> Iterator for LineGroupIter<TIter> where TIter: Iterator<Item=String> {
    type Item = [String; 3];

    fn next(&mut self) -> Option<Self::Item> {
        if self.done { return None }

        match self.iter.next() {
            Some(line) => {
                if self.memory.len() == 0 {
                    let empty_line = self.empty_char.to_string().repeat(line.len());
                    match self.iter.next() {
                        Some(line2) => {
                            self.memory.push_back(line.clone());
                            self.memory.push_back(line2.clone());
                            Some([empty_line, line, line2])
                        },
                        None => {
                            self.done = true;
                            Some([empty_line.clone(), line, empty_line])
                        }
                    }
                }
                else {
                    self.memory.push_back(line);
                    let pop = self.memory.pop_front().unwrap();
                    Some([pop, self.memory[0].clone(), self.memory[1].clone()])
                }
            },
            None => {
                self.done = true;
                if self.memory.len() == 0 {
                    None
                }
                else {
                    let empty_line = self.empty_char.to_string().repeat(self.memory[0].len());
                    let first = self.memory.pop_front().unwrap();
                    let second = self.memory.pop_front().unwrap();
                    Some([first, second, empty_line])
                }
            }
        }
    }
}

pub fn get_line_groups(lines: impl Iterator<Item=String>, empty_char: char) -> impl Iterator<Item=[String; 3]> {
    LineGroupIter { memory: VecDeque::new(), iter: lines, empty_char, done: false }
}

#[cfg(test)]
mod test {
    use super::get_line_groups;

    #[test]
    fn get_line_groups_empty() {
        let lines: [String; 0] = [];
        let line_groups: Vec<_> = get_line_groups(lines.into_iter(), ' ').collect();
        assert_eq!(0, line_groups.len());
    }

    #[test]
    fn get_line_groups_single_line() {
        let lines: [String; 1] = ["abc".to_string()];
        let empty_line = "...".to_string();
        let line_groups: Vec<_> = get_line_groups(lines.into_iter(), '.').collect();
        assert_eq!(1, line_groups.len());
        let group = &line_groups[0];
        assert_eq!(empty_line, group[0]);
        assert_eq!("abc", group[1]);
        assert_eq!(empty_line, group[2]);
    }

    #[test]
    fn get_line_groups_two_lines() {
        let line1 = "abc";
        let line2 = "123";
        let empty_line = "...".to_string();
        let lines: [String; 2] = [line1.to_string(), line2.to_string()];
        let line_groups: Vec<_> = get_line_groups(lines.into_iter(), '.').collect();

        assert_eq!(2, line_groups.len());
        assert_eq!(empty_line, line_groups[0][0]);
        assert_eq!(line1, line_groups[0][1]);
        assert_eq!(line2, line_groups[0][2]);
        assert_eq!(line1, line_groups[1][0]);
        assert_eq!(line2, line_groups[1][1]);
        assert_eq!(empty_line, line_groups[1][2]);
    }
}