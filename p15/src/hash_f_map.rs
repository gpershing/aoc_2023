use std::collections::HashMap;

use crate::hash_f::hash_f;

struct Lens {
    pub label: String,
    pub focal_len: u32
}

impl Lens {
    fn focusing_power(&self, hash: u8, box_index: usize) -> u32 {
        self.focal_len * (hash as u32 + 1) * (box_index as u32 + 1)
    }
}

pub struct HashFMap {
    boxes: HashMap<u8, Vec<Lens>>    
}

impl HashFMap {
    pub fn new() -> Self { Self { boxes: HashMap::new() }}

    pub fn sub(&mut self, label: &str) {
        let hash = hash_f(label);
        if let Some(b) = self.boxes.get_mut(&hash) {
            if let Some((index, _)) = b.iter().enumerate().find(|(idx, lens)| lens.label == label) {
                b.remove(index);
            }
        }
    }

    pub fn eq(&mut self, label: &str, focal_len: u32) {
        let hash = hash_f(label);
        let b = self.boxes.entry(hash).or_insert_with(|| Vec::new());
        match b.iter_mut().find(|lens| lens.label == label) {
            Some (lens) => lens.focal_len = focal_len,
            None => b.push(Lens { label: label.to_string(), focal_len })
        }
    }

    pub fn total_power(&self) -> u32 {
        self.boxes.iter().map(|(hash, b)| b.iter().enumerate().map(|(box_index, l)| l.focusing_power(*hash, box_index)).sum::<u32>()).sum()
    }
}