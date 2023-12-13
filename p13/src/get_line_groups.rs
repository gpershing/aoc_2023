struct GroupIter<'a, Itr, T> {
    pub items: &'a mut Itr,
    pub empty: T,
    pub done: bool
}

impl<'a, Itr, T> Iterator for GroupIter<'a, Itr, T> where Itr: Iterator, <Itr as Iterator>::Item: PartialEq<T> {
    type Item = Vec<Itr::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done { None }
        else {
            let mut item = Vec::new();
            loop {
                match self.items.next() {
                    None => {
                        self.done = true;
                        break Some(item);
                    }
                    Some(line) => {
                        if line == self.empty {
                            break Some(item);
                        }
                        else {
                            item.push(line);
                        }
                    }
                }
            }
        }
    }
}

pub fn get_line_groups(lines: &mut impl Iterator<Item=String>) -> impl Iterator<Item=Vec<String>> + '_ {
    GroupIter {
        items: lines,
        empty: "",
        done: false
    }
}