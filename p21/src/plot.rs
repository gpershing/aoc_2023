use std::collections::{HashMap, VecDeque};

use crate::{vector2::Vector2, direction::Direction, argm::Argm};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Garden,
    Rock
}

pub struct Plot {
    data: Vec<Vec<Tile>>,
    start: Vector2<usize>
}

#[derive(Debug, Clone, Copy)]
struct Values {
    if_even: u32,
    if_odd: u32,
    max: u32
}

impl Values {
    fn for_parity(&self, x: u32) -> u32 {
        if x % 2 == 0 { self.if_even } else { self.if_odd }
    }
}

#[derive(Debug)]
struct FillResult {
    end_border_normalized: Vec<u32>,
    norm_diff: u32,
    fill: HashMap<Vector2<usize>, u32>,
    values: Values
}

impl Plot {
    pub fn from_lines(lines: impl Iterator<Item=String>) -> Self {
        let mut start = Vector2::new(0, 0);
        let data = lines.enumerate().map(|(y, line)| line.chars().enumerate().map(|(x, c)| match c {
            '.' => Tile::Garden,
            '#' => Tile::Rock,
            'S' => {
                start = Vector2 { x, y };
                Tile::Garden
            },
            _ => panic!()
        }).collect()).collect();
        Self { data, start }
    }

    fn height(&self) -> usize { self.data.len() }
    fn width(&self) -> usize { self.data.first().map(|row| row.len()).unwrap_or(0) }

    fn move_idx(&self, p: Vector2<usize>, d: Direction) -> Option<Vector2<usize>> {
        let pi = Vector2::new(p.x as i64, p.y as i64);
        let v = d.moved(&pi);
        if v.y < self.height() as i64 && v.x < self.width() as i64 {
            v.try_into().ok()
        }
        else {
            None
        }
    }

    fn get_next_explore(true_queue: &mut VecDeque<(Vector2<usize>, u32)>, initial_queue: &mut VecDeque<(Vector2<usize>, u32)>) -> Option<(Vector2<usize>, u32)> {
        if initial_queue.is_empty() {
            true_queue.pop_front()
        }
        else if true_queue.is_empty() {
            initial_queue.pop_front()
        }
        else {
            if true_queue.front().unwrap().1 < initial_queue.front().unwrap().1 {
                true_queue.pop_front()
            }
            else {
                initial_queue.pop_front()
            }
        }
    }

    fn get_distances_starting(&self, mut to_explore_starting: VecDeque<(Vector2<usize>, u32)>) -> HashMap<Vector2<usize>, u32> {
        let mut map = HashMap::new();
        let mut to_explore = VecDeque::new();
        while let Some((explore, step_count)) = Self::get_next_explore(&mut to_explore, &mut to_explore_starting) {
            if !map.contains_key(&explore) { 
                map.insert(explore, step_count);

                Direction::all().into_iter().filter_map(|dir| self.move_idx(explore, dir))
                    .filter(|idx| self.data[idx.y][idx.x] == Tile::Garden)
                    .for_each(|idx| {
                        to_explore.push_back((idx, step_count + 1));
                    });
            }
        }
        map
    }

    pub fn get_distances(&self) -> HashMap<Vector2<usize>, u32> {
        let mut to_explore = VecDeque::new();
        to_explore.push_back((self.start, 0));
        self.get_distances_starting(to_explore)
    }

    fn get_border_indicies(&self, axis: Direction) -> Vec<Vector2<usize>> {
        match axis {
            Direction::North => (0..self.width()).map(|x| Vector2 { x, y : 0 }).collect(),
            Direction::East => (0..self.height()).map(|y| Vector2 { x: self.width() - 1, y }).collect(),
            Direction::South => (0..self.width()).map(|x| Vector2 { x, y : self.height() - 1 }).collect(),
            Direction::West => (0..self.height()).map(|y| Vector2 { x: 0, y }).collect(),
        }
    }

    fn get_start_index_from_border_index(&self, axis: Direction, index: usize) -> Vector2<usize> {
        match axis {
            Direction::North => Vector2 { x: index, y: self.height() - 1 },
            Direction::East => Vector2 { x: 0, y: index },
            Direction::South => Vector2 { x: index, y: 0 },
            Direction::West => Vector2 { x: self.width() - 1, y: index }
        }
    }

    fn get_end_border(&self, axis: Direction, fill: &HashMap<Vector2<usize>, u32>) -> Vec<u32> {
        self.get_border_indicies(axis).into_iter().map(|idx| fill[&idx]).collect()
    }

    fn normalize_end_border(&self, border: &mut Vec<u32>) -> u32 {
        let min = *border.iter().min().unwrap();
        border.iter_mut().for_each(|x| *x -= min);
        min
    }

    fn get_explore_start_from_end_border(&self, axis: Direction, normalized_end_border: &Vec<u32>) -> VecDeque<(Vector2<usize>, u32)> {
        let mut to_explore_v: Vec<_> = normalized_end_border.iter().enumerate().map(|(idx, v)| {
            (self.get_start_index_from_border_index(axis, idx), *v)
        }).collect();
        to_explore_v.sort_by_key(|(_, v)| *v);
        to_explore_v.into_iter().collect()
    }

    fn compute_values(&self, fill: &HashMap<Vector2<usize>, u32>) -> Values {
        let (e, o) = fill.values().fold((0, 0), |(e, o), v| {
            if v % 2 == 0 { (e + 1, o) } else { (e, o + 1) }
        });
        Values { if_even: e, if_odd: o, max: *fill.values().max().unwrap() }
    }

    fn detect_cycle(&self, results: &Vec<FillResult>, border: &Vec<u32>) -> Option<usize> {
        // Need to skip the first result as its norm is different
        results.iter().enumerate().skip(1).find_map(|(idx, r)| {
            if &r.end_border_normalized == border {
                Some(idx)
            } else { None }
        })
    }

    fn get_fills_along_axis(&self, axis: Direction, initial_border: Vec<u32>, results: &mut Vec<FillResult>) -> usize {
        let mut border = initial_border;
        loop {
            let to_explore = self.get_explore_start_from_end_border(axis, &border);
            let fill = self.get_distances_starting(to_explore);
            let values = self.compute_values(&fill);

            let mut next_border = self.get_end_border(axis, &fill);
            let norm = self.normalize_end_border(&mut next_border);

            results.push(FillResult {
                end_border_normalized: border,
                norm_diff: norm + 1,
                fill,
                values
            });
            border = next_border;

            if let Some(cycle) = self.detect_cycle(&results, &border) {
                break cycle;
            }
        }
    }

    fn count_values_normed(&self, fill: &HashMap<Vector2<usize>, u32>, norm: u32, steps: u32) -> u64 {
        fill.values().filter(|v| (*v + norm) % 2 == steps % 2 && (*v + norm) <= steps).count() as u64
    }

    fn total_along_axis_before_cycle(&self, results: &[FillResult], norm: u32, steps: u32) -> (u32, u64) {
        results.iter().fold((norm, 0), |(norm, tiles_total), result| {
            let tiles = if norm + result.values.max <= steps {
                result.values.for_parity(steps - norm) as u64
            }
            else {
                self.count_values_normed(&result.fill, norm, steps)
            };
            (norm + result.norm_diff, tiles_total + tiles)
        })
    }

    fn total_along_axis_after_cycle(&self, results: &[FillResult], norm: u32, steps: u32) -> u64 {
        let cycle_norm_diff: u32 = results.iter().map(|result| result.norm_diff).sum();
        let mut start_norm = norm;
        let mut tiles_total = 0u64;
        results.iter().for_each(|result| {
            let n = if start_norm + result.values.max <= steps { u32::argmin_gt(steps - start_norm - result.values.max, cycle_norm_diff) } else { 0 };
            if n > 0 {
                let second_n = n / 2;
                let first_n = n - second_n;
                let first = result.values.for_parity(steps - start_norm) * first_n;
                let second = result.values.for_parity(steps - start_norm + cycle_norm_diff) * second_n;
                tiles_total += (first + second) as u64;
            }

            let mut tail_norm = start_norm + cycle_norm_diff * n;
            while tail_norm < steps {
                let d =  self.count_values_normed(&result.fill, tail_norm, steps);
                tiles_total += d;
                tail_norm += cycle_norm_diff;
            }

            start_norm += result.norm_diff;
        });
        tiles_total
    }

    fn total_along_axis(&self, axis: Direction, initial_fill: &HashMap<Vector2<usize>, u32>, steps: u32) -> u64 {
        let mut results: Vec<FillResult> = Vec::new();
        let fill = results.last().map(|r| &r.fill).unwrap_or(&initial_fill);
        let mut border = self.get_end_border(axis, fill);
        let norm = self.normalize_end_border(&mut border) + 1;
        let cycle_index = self.get_fills_along_axis(axis, border, &mut results);

        let (norm, tiles_before) = self.total_along_axis_before_cycle(&results[0..cycle_index], norm, steps);
        let tiles_after = self.total_along_axis_after_cycle(&results[cycle_index..], norm, steps);
        tiles_before + tiles_after
    }

    fn get_corner_result(&self, corner: Vector2<usize>) -> FillResult {
        let mut to_explore = VecDeque::new();
        to_explore.push_back((Vector2::new(self.width() - 1 - corner.x, self.height() - 1 - corner.y) , 0));
        let fill = self.get_distances_starting(to_explore);
        let values = self.compute_values(&fill);
        // assume the puzzle is a square
        assert_eq!(self.width(), self.height());

        FillResult { end_border_normalized: vec![], norm_diff: self.width() as u32, fill, values }
    }

    fn total_along_corner(&self, corner: Vector2<usize>, initial_fill: &HashMap<Vector2<usize>, u32>, steps: u32) -> u64 {
        let result = self.get_corner_result(corner);
        let corner_v = initial_fill[&corner];
        let norm = corner_v + 2;
        
        let n = if norm + result.values.max <= steps { u32::argmin_gt(steps - norm - result.values.max, result.norm_diff) } else { 0 };
        let k = if norm <= steps { u32::argmin_gt(steps - norm, result.norm_diff) } else { 0 };
        let mut tiles_total = 0u64;
        (0..k).for_each(|idx| {
            let current_norm = norm + result.norm_diff * idx;
            if idx < n {
                tiles_total += (result.values.for_parity(steps - current_norm) * (idx + 1)) as u64;
            }
            else {
                tiles_total += self.count_values_normed(&result.fill, current_norm, steps) * (idx + 1) as u64;
            }
        });
        tiles_total
    }

    pub fn get_steps_repeating(&self, steps: u32) -> u64 {
        let initial_fill = self.get_distances();
        let center = initial_fill.values().filter(|v| *v % 2 == steps % 2 && **v <= steps).count() as u64;
        let axis_n = self.total_along_axis(Direction::North, &initial_fill, steps);
        let axis_e = self.total_along_axis(Direction::East, &initial_fill, steps);
        let axis_s = self.total_along_axis(Direction::South, &initial_fill, steps);
        let axis_w = self.total_along_axis(Direction::West, &initial_fill, steps);
        let corner_nw = self.total_along_corner(Vector2 { x: 0, y: 0 }, &initial_fill, steps);
        let corner_sw= self.total_along_corner(Vector2 { x: 0, y: self.height() - 1 }, &initial_fill, steps);
        let corner_se = self.total_along_corner(Vector2 { x: self.width() - 1, y: self.height() - 1 }, &initial_fill, steps);
        let corner_ne= self.total_along_corner(Vector2 { x: self.width() - 1, y: 0 }, &initial_fill, steps);
        center + axis_n + axis_e + axis_s + axis_w + corner_ne + corner_nw + corner_se + corner_sw
    }

    pub fn get_steps_repeating_brute(&self, steps: u32) {
        #[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
        struct LayerIndex {
            index: Vector2<usize>,
            layer: Vector2<i32>
        }

        let mut to_explore = VecDeque::new();
        to_explore.push_back((LayerIndex { index: self.start, layer: Vector2::new(0, 0) }, 0));
        let mut map = HashMap::new();
        while let Some((explore, step_count)) = to_explore.pop_front() {
            if step_count > steps { break }
            if !map.contains_key(&explore) { 
                map.insert(explore, step_count);

                Direction::all().into_iter().map(|dir| self.move_idx(explore.index, dir).map(|index| LayerIndex { index, layer: explore.layer }).unwrap_or_else(|| match dir {
                    Direction::North => LayerIndex { index: Vector2 { x: explore.index.x, y: self.height() - 1 }, layer: explore.layer + dir.as_vector::<i32>() },
                    Direction::East => LayerIndex { index: Vector2 { x: 0, y: explore.index.y }, layer: explore.layer + dir.as_vector::<i32>() },
                    Direction::South => LayerIndex { index: Vector2 { x: explore.index.x, y: 0 }, layer: explore.layer + dir.as_vector::<i32>() },
                    Direction::West => LayerIndex { index: Vector2 { x: self.width() - 1, y: explore.index.y }, layer: explore.layer + dir.as_vector::<i32>() }
                })).filter(|li| self.data[li.index.y][li.index.x] == Tile::Garden).for_each(|li| {
                    to_explore.push_back((li, step_count + 1))
                });
            }
        }
        let mut center = 0;
        let mut axis_n = 0;
        let mut axis_e = 0;
        let mut axis_s = 0;
        let mut axis_w = 0;
        let mut corner_ne = 0;
        let mut corner_se = 0;
        let mut corner_sw = 0;
        let mut corner_nw = 0;
        let mut total = 0;
        map.iter().for_each(|(k,v)| {
            if v % 2 == steps % 2 && *v <= steps {
                total += 1;
                match (k.layer.x, k.layer.y) {
                    (0, 0) => center += 1,
                    (0, 1..) => axis_s += 1,
                    (0, ..=-1) => axis_n += 1,
                    (1.., 0) => axis_e += 1,
                    (..=-1, 0) => axis_w += 1,
                    (1.., 1..) => corner_se += 1,
                    (..=-1, 1..) => corner_sw += 1,
                    (1.., ..=-1) => corner_ne += 1,
                    (..=-1, ..=-1) => corner_nw += 1,
                    _ => panic!()
                }
            }
        });
        println!("{center}");
        println!("{axis_n} {axis_e} {axis_s} {axis_w}");
        println!("{corner_ne} {corner_se} {corner_sw} {corner_nw}");
        println!("{total}");
    }
}
