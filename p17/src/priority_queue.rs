use std::{collections::HashMap, hash::Hash, mem::swap};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PriorityQueueEntry<Item, Priority> {
    pub item: Item,
    pub priority: Priority
}

struct PQueueTree<Item, Priority>
    where Item: Hash + Eq + Clone {
    tree: Vec<PriorityQueueEntry<Item, Priority>>,
    refr: HashMap<Item, usize>
}

enum PQueueTreeChildren {
    None,
    LeftOnly(usize),
    Both(usize, usize)
}

impl<Item, Priority> PQueueTree<Item, Priority>
    where Item: Hash + Eq + Clone,
    Priority: PartialOrd {
    pub fn new() -> Self {
        Self { tree: Vec::new(), refr: HashMap::new() }
    }

    pub fn len(&self) -> usize { self.tree.len() }

    pub fn contains_key(&self, item: &Item) -> bool {
        self.refr.contains_key(item)
    }

    pub fn get_priority(&self, item: &Item) -> Option<&Priority> {
        self.refr.get(item).and_then(|&idx| self.tree.get(idx)).map(|entry| &entry.priority)
    }

    fn parent(&self, index: usize) -> Option<usize> {
        if index > 0 {
            Some((index + 1) / 2 - 1)
        }
        else {
            None
        }
    }
    fn children(&self, index: usize) -> PQueueTreeChildren {
        match self.lchild(index) {
            Some(lchild) => match self.rchild(index) {
                Some(rchild) => PQueueTreeChildren::Both(lchild, rchild),
                None => PQueueTreeChildren::LeftOnly(lchild)
            }
            None => PQueueTreeChildren::None
        }
    }

    fn lchild(&self, index: usize) -> Option<usize> {
        let unbounded = (index + 1) * 2 - 1;
        if unbounded < self.tree.len() {
            Some(unbounded)
        }
        else {
            None
        }
    }
    fn rchild(&self, index: usize) -> Option<usize> {
        let unbounded = (index + 1) * 2;
        if unbounded < self.tree.len() {
            Some(unbounded)
        }
        else {
            None
        }
    }

    fn peek(&self) -> Option<&PriorityQueueEntry<Item, Priority>> {
        self.tree.first()
    }
    fn pop(&mut self) -> Option<PriorityQueueEntry<Item, Priority>> {
        if let Some(p) = self.tree.pop() {
            self.refr.remove(&p.item);
            Some(p)
        }
        else {
            None
        }
    }
    fn push(&mut self, item: Item, priority: Priority) {
        self.refr.insert(item.clone(), self.tree.len());
        self.tree.push(PriorityQueueEntry { item, priority });
    }
    fn swap(&mut self, a: usize, b: usize) {
        self.tree.swap(a, b);
        *self.refr.get_mut(&self.tree[a].item).unwrap() = a;
        *self.refr.get_mut(&self.tree[b].item).unwrap() = b;
    }
    fn update(&mut self, item: &Item, priority: Priority) {
        let index = self.refr[item];
        let mut priority = priority;
        swap(&mut self.tree[index].priority, &mut priority);
        if &priority < &self.tree[index].priority {
            self.bubble_down_from(index);
        }
        else {
            self.bubble_up_from(index);
        }
    }
    fn bubble_down_from(&mut self, index: usize) {
        let mut index = index;
        loop {
            match self.children(index) {
                PQueueTreeChildren::None => break,
                PQueueTreeChildren::LeftOnly(lchild) => {
                    let lchild_pri = &self.tree[lchild].priority;
                    if lchild_pri < &self.tree[index].priority {
                        self.swap(index, lchild);
                        index = lchild;
                    }
                    else {
                        break
                    }
                },
                PQueueTreeChildren::Both(lchild, rchild) => {
                    let lchild_pri = &self.tree[lchild].priority;
                    let rchild_pri = &self.tree[rchild].priority;
                    let (next_index, next_pri) = if lchild_pri < rchild_pri {
                        (lchild, lchild_pri)
                    }
                    else {
                        (rchild, rchild_pri)
                    };
                    if next_pri < &self.tree[index].priority {
                        self.swap(index, next_index);
                        index = next_index;
                    }
                    else {
                        break
                    }
                },
            }
        }
    }
    fn bubble_up_from(&mut self, index: usize) {
        let mut index = index;
        while let Some(pindex) = self.parent(index) {
            let parent_pri = &self.tree[pindex].priority;
            if parent_pri > &self.tree[index].priority {
                self.swap(index, pindex);
                index = pindex;
            }
            else {
                break
            }
        }
    }
    fn bubble_down(&mut self) { self.bubble_down_from(0); }
    fn bubble_up(&mut self) { self.bubble_up_from(self.tree.len() - 1); }
}

pub struct PriorityQueue<Item, Priority>
    where Item: Hash + Eq + Clone,
    Priority: PartialOrd {
    tree: PQueueTree<Item, Priority>
}

impl<Item, Priority> PriorityQueue<Item, Priority>
    where Item: Hash + Eq + Clone,
    Priority: PartialOrd {
    pub fn new() -> Self {
        Self { tree: PQueueTree::new() }
    }

    pub fn peek_min(&self) -> Option<&PriorityQueueEntry<Item, Priority>> { self.tree.peek() }
    pub fn pop_min(&mut self) -> Option<PriorityQueueEntry<Item, Priority>> {
        if self.tree.len() > 0 {
            self.tree.swap(0, self.tree.len() - 1);
        }
        let min = self.tree.pop();
        self.tree.bubble_down();
        min
    }

    pub fn contains_key(&self, item: &Item) -> bool {
        self.tree.contains_key(item)
    }
    pub fn get_priority(&self, item: &Item) -> Option<&Priority> {
        self.tree.get_priority(item)
    }

    pub fn insert(&mut self, item: Item, priority: Priority) {
        self.tree.push(item, priority);
        self.tree.bubble_up();
    }
    pub fn update(&mut self, item: &Item, priority: Priority) {
        self.tree.update(item, priority);
    }
    pub fn insert_or_update(&mut self, item: Item, priority: Priority) {
        if self.contains_key(&item) {
            self.update(&item, priority);
        }
        else {
            self.insert(item, priority);
        }
    }
}

#[cfg(test)]
mod test {
    use crate::priority_queue::PriorityQueueEntry;

    use super::PriorityQueue;

    #[test]
    fn peek_empty() {
        let mut pq: PriorityQueue<i32, i32> = PriorityQueue::new();
        assert_eq!(None, pq.peek_min());
    }

    #[test]
    fn pop_empty() {
        let mut pq: PriorityQueue<i32, i32> = PriorityQueue::new();
        assert_eq!(None, pq.pop_min());
    }

    #[test]
    fn peek_first() {
        let mut pq = PriorityQueue::new();
        pq.insert(0, 0);
        assert_eq!(Some(&PriorityQueueEntry { item: 0, priority: 0 }), pq.peek_min());
    }

    #[test]
    fn pop_first() {
        let mut pq = PriorityQueue::new();
        pq.insert(0, 0);
        assert_eq!(Some(PriorityQueueEntry { item: 0, priority: 0 }), pq.pop_min());
    }

    #[test]
    fn peek_after_pop() {
        let mut pq = PriorityQueue::new();
        pq.insert(0, 0);
        pq.pop_min();
        assert_eq!(None, pq.peek_min());
    }

    #[test]
    fn pop_after_pop() {
        let mut pq = PriorityQueue::new();
        pq.insert(0, 0);
        pq.pop_min();
        assert_eq!(None, pq.pop_min());
    }

    #[test]
    fn pop_min_in_order() {
        let mut pq = PriorityQueue::new();
        pq.insert(0, 0);
        pq.insert(1, 1);
        assert_eq!(Some(PriorityQueueEntry { item: 0, priority: 0 }), pq.pop_min());
        assert_eq!(Some(PriorityQueueEntry { item: 1, priority: 1 }), pq.pop_min());
    }

    #[test]
    fn pop_min_rev_order() {
        let mut pq = PriorityQueue::new();
        pq.insert(1, 1);
        pq.insert(0, 0);
        assert_eq!(Some(PriorityQueueEntry { item: 0, priority: 0 }), pq.pop_min());
        assert_eq!(Some(PriorityQueueEntry { item: 1, priority: 1 }), pq.pop_min());
    }

    #[test]
    fn pop_many() {
        let mut pq = PriorityQueue::new();
        pq.insert(2, 2);
        pq.insert(0, 0);
        pq.insert(4, 4);
        pq.insert(1, 1);
        pq.insert(5, 5);
        pq.insert(3, 3);
        assert_eq!(Some(PriorityQueueEntry { item: 0, priority: 0 }), pq.pop_min());
        assert_eq!(Some(PriorityQueueEntry { item: 1, priority: 1 }), pq.pop_min());
        assert_eq!(Some(PriorityQueueEntry { item: 2, priority: 2 }), pq.pop_min());
        assert_eq!(Some(PriorityQueueEntry { item: 3, priority: 3 }), pq.pop_min());
        assert_eq!(Some(PriorityQueueEntry { item: 4, priority: 4 }), pq.pop_min());
        assert_eq!(Some(PriorityQueueEntry { item: 5, priority: 5 }), pq.pop_min());
        assert_eq!(None, pq.pop_min());
    }

    fn update_priority() {
        let mut pq = PriorityQueue::new();
        pq.insert(0, 0);
        pq.insert(1, 1);
        pq.update(&0, 2);
        assert_eq!(Some(PriorityQueueEntry { item: 0, priority: 0 }), pq.pop_min());
        assert_eq!(Some(PriorityQueueEntry { item: 1, priority: 1 }), pq.pop_min());
        assert_eq!(None, pq.pop_min());
    }

}