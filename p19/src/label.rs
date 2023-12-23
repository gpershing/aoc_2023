#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Label {
    X, M, A, S
}

impl Label {
    pub fn from_char(c: char) -> Label {
        match c {
            'x' => Label::X,
            'm' => Label::M,
            'a' => Label::A,
            's' => Label::S,
            _ => panic!()
        }
    }
}