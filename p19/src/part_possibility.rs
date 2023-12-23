use std::ops::Range;

use crate::label::Label;

#[derive(Debug, Clone)]
pub struct PartPossibility {
    x: Range<u32>,
    m: Range<u32>,
    a: Range<u32>,
    s: Range<u32>   
}

impl PartPossibility {
    pub fn full(min: u32, max: u32) -> Self {
        Self { x: min..(max+1), m: min..(max+1), a: min..(max+1), s: min..(max+1) }
    }

    pub fn empty() -> Self {
        Self { x: 0..0, m: 0..0, a: 0..0, s: 0..0 }
    }

    pub fn get(&self, label: Label) -> &Range<u32> {
        match label {
            Label::X => &self.x,
            Label::M => &self.m,
            Label::A => &self.a,
            Label::S => &self.s,
        }
    }

    pub fn get_mut(&mut self, label: Label) -> &mut Range<u32> {
        match label {
            Label::X => &mut self.x,
            Label::M => &mut self.m,
            Label::A => &mut self.a,
            Label::S => &mut self.s,
        }
    }

    pub fn replacing(&self, label: Label, range: Range<u32>) -> PartPossibility {
        match label {
            Label::X => Self { x: range, ..self.clone() },
            Label::M => Self { m: range, ..self.clone() },
            Label::A => Self { a: range, ..self.clone() },
            Label::S => Self { s: range, ..self.clone() },
        }
    }

    pub fn separate_lt(&mut self, label: Label, part: u32) -> PartPossibility {
        let range = self.get_mut(label);
        if part < range.start {
            Self::empty()
        }
        else if part >= range.end {
            let full_range = range.clone();
            *range = 0..0;
            self.replacing(label, full_range)
        }
        else {
            let start = range.start;
            *range = part..range.end;
            let rest = start..part;
            self.replacing(label, rest)
        }
    }

    pub fn separate_gt(&mut self, label: Label, part: u32) -> PartPossibility {
        let range = self.get_mut(label);
        if part + 1 >= range.end {
            Self::empty()
        }
        else if part < range.start {
            let full_range = range.clone();
            *range = 0..0;
            self.replacing(label, full_range)
        }
        else {
            let end = range.end;
            *range = range.start..(part+1);
            let rest = (part+1)..end;
            self.replacing(label, rest)
        }
    }

    pub fn size(&self) -> usize {
        self.x.len() * self.m.len() * self.a.len() * self.s.len()
    }
}