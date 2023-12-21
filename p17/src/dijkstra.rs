use std::{collections::HashMap, hash::Hash, ops::Add};

use crate::priority_queue::PriorityQueue;

pub struct DijkstraSearch<Node, Dist, NodeItr, NeighborF, DistanceF>
    where Node: Hash + Eq + Clone,
    Dist: PartialOrd + Clone,
    NodeItr: Iterator<Item=Node>,
    NeighborF: Fn(&Node) -> NodeItr,
    DistanceF: Fn(&Node, &Node) -> Dist {
    to_visit: PriorityQueue<Node, Dist>,
    distance: HashMap<Node, Dist>,
    neighbors_f: NeighborF,
    distance_f: DistanceF
}

impl<Node, Dist, NodeItr, NeighborF, DistanceF> DijkstraSearch<Node, Dist, NodeItr, NeighborF, DistanceF>
    where Node: Hash + Eq + Clone,
    Dist: PartialOrd + Clone,
    for<'a> &'a Dist: Add<&'a Dist, Output=Dist>,
    NodeItr: Iterator<Item=Node>,
    NeighborF: Fn(&Node) -> NodeItr,
    DistanceF: Fn(&Node, &Node) -> Dist {
    fn init_with(start: Node, neighbors: NeighborF, distance: DistanceF, starting_dist: Dist) -> Self {
        let mut to_visit = PriorityQueue::new();
        to_visit.insert(start, starting_dist);
        Self { to_visit, distance: HashMap::new(), neighbors_f: neighbors, distance_f: distance }
    }

    fn compute_step(&mut self) -> bool where NodeItr: Iterator<Item=Node> {
        if let Some(current) = self.to_visit.pop_min() {
            self.distance.insert(current.item.clone(), current.priority.clone());
            (self.neighbors_f)(&current.item).for_each(|neighbor| {
                let new_tentative = &(self.distance_f)(&current.item, &neighbor) + &current.priority;
                if match self.to_visit.get_priority(&neighbor) {
                    Some(entry) => &new_tentative < &entry,
                    None => true,
                } {
                    self.to_visit.insert_or_update(neighbor, new_tentative);
                }
            });
            true
        }
        else {
            false
        }
    }

    fn get_distance(&mut self, end: &Node) -> Option<Dist> {
        loop {
            if let Some(dist) = self.distance.get(end) {
                break Some(dist.clone());
            }
            if !self.compute_step() {
                break None;
            }
        }
    }

    fn compute_all(&mut self) {
        while self.compute_step() {}
    }

    fn distance_map(&self) -> &HashMap<Node, Dist> { &self.distance }
}

pub fn dijkstra_with<Node, Dist, NodeItr>(start: Node, end: &Node, neighbors: impl Fn(&Node) -> NodeItr, dist: impl Fn(&Node, &Node) -> Dist, starting_dist: Dist) -> Option<Dist>
    where Node: Hash + PartialEq + Eq + Clone,
    Dist: Ord + Clone,
    for<'a> &'a Dist: Add<Output=Dist>,
    NodeItr: Iterator<Item=Node> {
    let mut d = DijkstraSearch::init_with(start, neighbors, dist, starting_dist);
    d.get_distance(end)
}

pub fn dijkstra_all_with<Node, Dist, NodeItr>(start: Node, neighbors: impl Fn(&Node) -> NodeItr, dist: impl Fn(&Node, &Node) -> Dist, starting_dist: Dist) -> HashMap<Node, Dist>
    where Node: Hash + PartialEq + Eq + Clone,
    Dist: Ord + Clone,
    for<'a> &'a Dist: Add<Output=Dist>,
    NodeItr: Iterator<Item=Node> {
    let mut d = DijkstraSearch::init_with(start, neighbors, dist, starting_dist);
    d.compute_all();
    d.distance
}

pub fn dijkstra<Node, Dist, NodeItr>(start: Node, end: &Node, neighbors: impl Fn(&Node) -> NodeItr, dist: impl Fn(&Node, &Node) -> Dist) -> Option<Dist>
    where Node: Hash + PartialEq + Eq + Clone,
    Dist: Ord + Clone + Default,
    for<'a> &'a Dist: Add<Output=Dist>,
    NodeItr: Iterator<Item=Node> {
    dijkstra_with(start, end, neighbors, dist, Dist::default())
}

pub fn dijkstra_all<Node, Dist, NodeItr>(start: Node, neighbors: impl Fn(&Node) -> NodeItr, dist: impl Fn(&Node, &Node) -> Dist) -> HashMap<Node, Dist>
    where Node: Hash + PartialEq + Eq + Clone,
    Dist: Ord + Clone + Default,
    for<'a> &'a Dist: Add<Output=Dist>,
    NodeItr: Iterator<Item=Node> {
    dijkstra_all_with(start, neighbors, dist, Dist::default())
}

#[cfg(test)]
mod test {
    use super::dijkstra;

    #[test]
    fn test_a() {
        let result = dijkstra(
            0,
            &3, 
            |n| match n {
                0 => vec![1, 2, 3].into_iter(),
                1 => vec![0, 3].into_iter(),
                2 => vec![0, 3].into_iter(),
                3 => vec![0, 1, 2].into_iter(),
                _ => panic!()
            },
        |a, b| match (a, b) {
            (0, 0) => 0,
            (0, 1) => 1,
            (0, 2) => 48,
            (0, 3) => 50,
            (1, 0) => 1,
            (1, 1) => 0,
            (1, 3) => 50,
            (2, 0) => 48,
            (2, 2) => 0,
            (2, 3) => 1,
            (3, 0) => 50,
            (3, 1) => 50,
            (3, 2) => 1,
            (3, 3) => 0,
            _ => panic!()
        });
        assert_eq!(Some(49), result);
    }
}