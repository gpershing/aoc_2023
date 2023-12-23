use std::collections::HashMap;

use crate::label::Label;

pub struct Part {
    items: HashMap<Label, u32>    
}

impl Part {
    pub fn from_str(s: &str) -> Self {
        let parts = (&s[1..s.len()-1]).split(',');
        let mut items = HashMap::new();
        parts.for_each(|part| {
            let mut s = part.split('=');
            let name = Label::from_char(s.next().unwrap().chars().next().unwrap());
            let count: u32 = s.next().unwrap().parse().unwrap();
            items.insert(name, count);
        });
        Self { items }
    }

    pub fn get(&self, c: &Label) -> u32 { self.items[c] }

    pub fn sum(&self) -> u32 { self.items.values().sum() }
}