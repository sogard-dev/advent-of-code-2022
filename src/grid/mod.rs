use std::collections::{HashMap, HashSet, VecDeque};

pub type Coordinate = (isize, isize);

#[derive(Debug, PartialEq, Clone)]
pub struct Grid<Model> {
    pub width: isize,
    pub height: isize,
    nodes: HashMap<Coordinate, Model>,
    pub connections: HashMap<Coordinate, HashSet<Coordinate>>,
}

impl<T> Grid<T> {
    pub fn new(s: &str, mut node_parser: impl FnMut(Coordinate, char) -> T) -> Grid<T> {
        let height = s.lines().count() as isize;
        let width = s.lines().into_iter().next().unwrap().len() as isize;

        let mut grid = Grid { width, height, nodes: HashMap::new(), connections: HashMap::new() };

        let lines: Vec<&str> = s.lines().collect();
        for (row_index, line) in lines.iter().enumerate() {
            for (column_index, char) in line.chars().enumerate() {
                let my_pos = (row_index as isize, column_index as isize);
                let new_node = node_parser(my_pos, char);
                grid.nodes.insert(my_pos, new_node);
            }
        }

        grid
    }

    pub fn new_with_chars(s: &str, amt: usize, mut node_parser: impl FnMut(Coordinate, String) -> T) -> Grid<T> {
        let height = s.lines().count() as isize;
        let width = (s.lines().into_iter().next().unwrap().len() / 2) as isize;

        let mut grid = Grid { width, height, nodes: HashMap::new(), connections: HashMap::new() };

        let lines: Vec<&str> = s.lines().collect();
        for (row_index, line) in lines.iter().enumerate() {
            let chars: Vec<char> = line.chars().collect();
            for column_index in (0..(width as usize) * 2 - 1).step_by(amt) {
                let mut str = String::new();
                str.push(chars[column_index]);
                str.push(chars[column_index+1]);
                let my_pos = (row_index as isize, column_index as isize);
                let new_node = node_parser(my_pos, str);
                grid.nodes.insert(my_pos, new_node);
            }
        }

        grid
    }

    pub fn new_from_list(s: &str, mut node_parser: impl FnMut(Coordinate, &str) -> T) -> Grid<T> {
        let height = s.lines().count() as isize;

        let mut grid = Grid { width: 1, height, nodes: HashMap::new(), connections: HashMap::new() };

        let lines: Vec<&str> = s.lines().collect();
        for (row_index, line) in lines.iter().enumerate() {
            let my_pos = (row_index as isize, 0 as isize);
            let new_node = node_parser(my_pos, line);
            grid.nodes.insert(my_pos, new_node);
        }

        grid
    }

    pub fn add_directional(&mut self, pos_1: Coordinate, pos_2: Coordinate) {
        self.connections.entry(pos_1).or_insert_with(|| HashSet::new()).insert(pos_2);
    }

    pub fn get_model_offset(&self, pos: &Coordinate, delta: &Coordinate) -> Option<&T> {
        self.nodes.get(&(pos.0 - delta.0, pos.1 - delta.1))
    }

    pub fn get_model_offset_mut(&mut self, pos: &Coordinate, delta: &Coordinate) -> Option<&mut T> {
        self.nodes.get_mut(&(pos.0 - delta.0, pos.1 - delta.1))
    }

    pub fn get_model(&self, pos: &Coordinate) -> Option<&T> {
        self.get_model_offset(pos, &(0_isize, 0_isize))
    }

    pub fn get_model_mut(&mut self, pos: &Coordinate) -> Option<&mut T> {
        self.get_model_offset_mut(pos, &(0_isize, 0_isize))
    }

    pub fn swap_if(&mut self, s1: &Coordinate, s2: &Coordinate, swap_if_fn: impl Fn(&T, &T) -> bool) -> bool {
        match (self.get_model(s1), self.get_model(s2)) {
            (Some(m1), Some(m2)) => {
                if swap_if_fn(m1, m2) {
                    let o1 = self.nodes.remove(s1).unwrap();
                    let o2 = self.nodes.remove(s2).unwrap();

                    self.nodes.insert(*s1, o2);
                    self.nodes.insert(*s2, o1);

                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    pub fn bfs(&self, from: &Coordinate, mut f: impl FnMut(&Coordinate, isize)) {
        let init = *from;

        let mut to_visit = VecDeque::new();
        let mut added_to_visit = HashSet::new();

        to_visit.push_back(init);
        added_to_visit.insert(init);

        let mut distance = -1;
        while !to_visit.is_empty() {
            distance += 1;

            let mut seen = vec![];

            while let Some(pos_to_check) = to_visit.pop_front() {
                f(&pos_to_check, distance);

                for neighbour_pos in self.connections.get(&pos_to_check).unwrap() {
                    seen.push(*neighbour_pos);
                }
            }

            for pos in seen {
                if !added_to_visit.contains(&pos) {
                    added_to_visit.insert(pos);
                    to_visit.push_back(pos);
                }
            }
        }
    }

    pub fn print(&self, f: impl Fn(Coordinate, &T) -> String) {
        for row_index in 0..self.height {
            for column_index in 0..self.width {
                print!("{}", f((row_index, column_index), self.nodes.get(&(row_index, column_index)).unwrap()));
            }
            println!();
        }
        println!();
    }

    pub fn for_every(&self, mut f: impl FnMut(&Coordinate, &T)) {
        for row_index in 0..self.height {
            for column_index in 0..self.width {
                f(&(row_index, column_index), self.get_model(&(row_index, column_index)).unwrap());
            }
        }
    }

    pub fn sum(&self, mut f: impl FnMut(&Coordinate, &T) -> isize) -> isize {
        let mut sum = 0;
        for (pos, obj) in self.nodes.iter() {
            sum += f(pos, obj);
        }

        sum
    }

    pub fn find(&self, f: impl Fn(&Coordinate, &T) -> bool) -> Option<Coordinate> {
        for (pos, obj) in self.nodes.iter() {
            if f(pos, obj) {
                return Some(*pos);
            }
        }
        None
    }

    pub fn for_every_delta(&self, mut f: impl FnMut(&Coordinate, &T, &Coordinate, &T), to_visit: Vec<Coordinate>) {
        self.for_every(|pos, m1| {
            for delta in &to_visit {
                let n_x = ((pos.0 as isize) - delta.0) as isize;
                let n_y = ((pos.1 as isize) - delta.1) as isize;
                let neighbour_pos = (n_x, n_y);

                if let Some(m2) = self.nodes.get(&neighbour_pos) {
                    f(pos, m1, &neighbour_pos, m2);
                }
            }
        });
    }

    pub fn is_on_edge(&self, pos: Coordinate) -> bool {
        return pos.0 == 0 || pos.1 == 0 || pos.0 == self.width - 1 || pos.1 == self.height - 1;
    }

    pub fn add_connections(&mut self, connections: Vec<(Coordinate, Coordinate)>)  {
        for (from, to) in connections {
            self.add_directional(from, to);
        }
    }

    pub fn bfs_path(&self, from: &Coordinate, to: &Coordinate) -> Option<Vec<Coordinate>> {

        let init = *from;
        let mut to_visit = VecDeque::new();
        let mut added_to_visit = HashSet::new();

        to_visit.push_back(init);
        added_to_visit.insert(init);

        let mut came_from = HashMap::new();

        let mut distance = -1;
        while !to_visit.is_empty() {
            distance += 1;


            while let Some(pos_to_check) = to_visit.pop_front() {
                for neighbour_pos in self.connections.get(&pos_to_check).unwrap() {

                    if !added_to_visit.contains(&neighbour_pos) {
                        came_from.insert(neighbour_pos, pos_to_check);
                        added_to_visit.insert(*neighbour_pos);
                        to_visit.push_back(*neighbour_pos);
                    }
                }
            }
        }

        if came_from.get(to).is_none() {
            return None;
        }

        let mut path = vec![];
        path.push(*to);

        let mut at = *to;
        while let Some(n) = came_from.get(&at) {
            path.push(*n);
            at = *n;
        }
        path.reverse();

        Some(path)
    }
}

#[derive(Debug, PartialEq, Hash, Eq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

pub fn get_direction(from: &Coordinate, to: &Coordinate) -> Direction {
    if is_left(from, to) {
        Direction::Left
    } else if is_right(from, to) {
        Direction::Right
    } else if is_up(from, to) {
        Direction::Up
    } else if is_down(from, to) {
        Direction::Down
    } else {
        panic!("Ohh noes!");
    }
}

pub fn is_left(a: &Coordinate, b: &Coordinate) -> bool {
    a.1 == b.1 && a.0 < b.0
}

pub fn is_right(a: &Coordinate, b: &Coordinate) -> bool {
    a.1 == b.1 && a.0 > b.0
}

pub fn is_up(a: &Coordinate, b: &Coordinate) -> bool {
    a.0 == b.0 && a.1 < b.1
}

pub fn is_down(a: &Coordinate, b: &Coordinate) -> bool {
    a.0 == b.0 && a.1 > b.1
}
