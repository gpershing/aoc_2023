use std::collections::VecDeque;

pub struct Row {
    data: VecDeque<i32>
}

impl Row {
    pub fn from_line(line: String) -> Row {
        let data = line.split_ascii_whitespace()
            .map(|part| part.parse().unwrap())
            .collect();
        Row { data }
    }

    pub fn difference_row(&self) -> Row {
        let data = self.data.iter().zip(self.data.iter().skip(1))
            .map(|(first, second)| second - first)
            .collect();
        Row { data }
    }

    pub fn is_all_same(&self) -> bool {
        match self.data.front() {
            Some(first) => self.data.iter().all(|x| x == first),
            None => true
        }
    }

    pub fn is_all_zeros(&self) -> bool {
        self.data.iter().all(|x| *x == 0)
    }

    pub fn extend(&mut self, diff: i32) -> i32 {
        let val = self.data.back().map(|x| *x).unwrap_or(0) + diff;
        self.data.push_back(val);
        val
    }

    pub fn prepend(&mut self, diff: i32) -> i32 {
        let val = self.data.front().map(|x| *x).unwrap_or(0) - diff;
        self.data.push_front(val);
        val
    }
}

pub struct Tableau {
    rows: Vec<Row>
}

impl Tableau {
    pub fn new(starting_row: Row) -> Tableau {
        let mut rows = vec![starting_row];
        while !rows.last().unwrap().is_all_same() {
            rows.push(rows.last().unwrap().difference_row());
        }
        Tableau { rows }
    }

    pub fn extend(&mut self) -> i32 {
        self.rows.iter_mut().rev()
            .fold(0, |diff, row| row.extend(diff))
    }

    pub fn prepend(&mut self) -> i32 {
        self.rows.iter_mut().rev()
            .fold(0, |diff, row| row.prepend(diff))
    }
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    use super::{Tableau, Row};

    #[test]
    fn test_enable_vscode() {
        
    }

    #[test_case("", 0, 0, 0; "empty")]
    #[test_case("0 0 0", 0, 0, 0; "zeros")]
    #[test_case("1 1 1 1 1", 1, 1, 1; "constant")]
    #[test_case("1 2 3 4 5", 6, 7, 8; "linear")]
    #[test_case("2 5 10 17 26", 37, 50, 65; "n=2")]
    #[test_case("-10 -5 2 14 34", 65, 110, 172; "max")]
    fn test_extend(line: &str, v1: i32, v2: i32, v3: i32) {
        let mut tableau = Tableau::new(Row::from_line(line.into()));
        assert_eq!(v1, tableau.extend());
        assert_eq!(v2, tableau.extend());
        assert_eq!(v3, tableau.extend());
    }

    #[test_case(0, 0, 0, ""; "empty")]
    #[test_case(0, 0, 0, "0 0 0"; "zeros")]
    #[test_case(1, 1, 1, "1 1 1 1 1"; "constant")]
    #[test_case(1, 2, 3, "4 5 6 7 8"; "linear")]
    #[test_case(2, 5, 10, "17 26 37 50 65"; "n=2")]
    #[test_case(-10, -5, 2, "14 34 65 110 172"; "max")]
    fn test_prepend(v3: i32, v2: i32, v1: i32, line: &str) {
        let mut tableau = Tableau::new(Row::from_line(line.into()));
        assert_eq!(v1, tableau.prepend());
        assert_eq!(v2, tableau.prepend());
        assert_eq!(v3, tableau.prepend());
    }
}