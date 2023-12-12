use std::{collections::HashMap, borrow::Borrow, hash::{Hash, Hasher}};

use crate::row::Tile;

pub trait MyKey {
    fn a(&self) -> &[Tile];
    fn b(&self) -> &[usize];
}

impl<'a> Borrow<dyn MyKey + 'a> for (Vec<Tile>, Vec<usize>) {
    fn borrow(&self) -> &(dyn MyKey + 'a) {
        self
    }
}

impl MyKey for (Vec<Tile>, Vec<usize>) {
    fn a(&self) -> &[Tile] {
        &self.0
    }

    fn b(&self) -> &[usize] {
        &self.1
    }
}
impl MyKey for (&[Tile], &[usize]) {
    fn a(&self) -> &[Tile] {
        self.0
    }

    fn b(&self) -> &[usize] {
        self.1
    }
}

impl Hash for dyn MyKey + '_ {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.a().hash(state);
        self.b().hash(state);
    }
}

impl PartialEq for dyn MyKey + '_ {
    fn eq(&self, other: &Self) -> bool {
        self.a() == other.a() && self.b() == other.b()
    }
}

impl Eq for dyn MyKey + '_ {}


//https://stackoverflow.com/questions/45786717/how-to-implement-hashmap-with-two-keys
// trait KeyPair<A, B> {
//     fn a(&self) -> &A;
//     fn b(&self) -> &B;
// }

// impl<'a, A, B> Borrow<dyn KeyPair<A, B> + 'a> for (A, B)
// where
//     A: Eq + Hash + 'a,
//     B: Eq + Hash + 'a,
// {
//     fn borrow(&self) -> &(dyn KeyPair<A, B> + 'a) {
//         self
//     }
// }

// impl<A: Hash, B: Hash> Hash for dyn KeyPair<A, B> + '_ {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         self.a().hash(state);
//         self.b().hash(state);
//     }
// }

// impl<A: Eq, B: Eq> PartialEq for dyn KeyPair<A, B> + '_ {
//     fn eq(&self, other: &Self) -> bool {
//         self.a() == other.a() && self.b() == other.b()
//     }
// }

// impl<A: Eq, B: Eq> Eq for dyn KeyPair<A, B> + '_ {}

// impl<A, B> KeyPair<A, B> for (A, B) {
//     fn a(&self) -> &A {
//         &self.0
//     }
//     fn b(&self) -> &B {
//         &self.1
//     }
// }
// impl<A, B> KeyPair<A, B> for (&A, &B) {
//     fn a(&self) -> &A {
//         self.0
//     }
//     fn b(&self) -> &B {
//         self.1
//     }
// }

pub type Cache = HashMap<(Vec<Tile>, Vec<usize>), usize>;

pub fn new_cache() -> Cache {
    HashMap::new()
}