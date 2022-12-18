use regex::Regex;

pub fn parse_numbers(s: &str) -> Vec<isize> {
    let mut vec = Vec::new();
    let re = Regex::new(r"([-\d]+)").unwrap();
    for capture in re.captures_iter(&s) {
        for i in 1..capture.len() {
            if let Ok(num) = capture[i].parse::<isize>() {
                vec.push(num);
            }
        }
    }

    vec
}

#[derive(Debug, PartialEq, Clone, Eq)]
pub struct Interval {
    pub start: isize,
    pub end: isize,
}

impl Interval {
    pub fn new(start: isize, end: isize) -> Self {
        Interval { start, end }
    }
}

#[derive(Debug, PartialEq, Clone, Eq)]

pub struct Intervals {
    vec: Vec<Interval>,
}

impl Intervals {
    pub fn new() -> Self {
        Intervals { vec: vec![] }
    }

    pub fn get_start(&self) -> Option<&Interval> {
        self.vec.first()
    }

    pub fn get_end(&self) -> Option<&Interval> {
        self.vec.last()
    }

    pub fn vec(&self) -> &Vec<Interval> {
        &self.vec
    }

    pub fn add(&mut self, new: Interval) {
        self.vec.push(new);
        self.align();
    }

    fn align(&mut self) {
        self.vec.sort_by_key(|f| f.start);

        let mut i = 0;
        while i < self.vec.len() - 1 {
            let combine = {
                let prev = &self.vec[i];
                let next = &self.vec[i + 1];
                next.start <= prev.end
            };

            if combine {
                self.vec[i].end = self.vec[i + 1].end.max(self.vec[i].end);
                self.vec.remove(i + 1);
            } else {
                i += 1;
            }
        }
    }

    pub fn remove(&mut self, new: Interval) {
        //println!("Should remove: {:?}", new);
        let mut i = 0;
        while i < self.vec.len() {
            //println!("  Looking at: {:?}", self.vec[i]);

            if self.vec[i].start > new.end {
                return;
            }

            if self.vec[i].end < new.start {
                i += 1;
                continue;
            }

            //We have some overlap
            if self.vec[i].start >= new.start && self.vec[i].end <= new.end {
                //println!("    Total overlap");
                self.vec.remove(i);
                continue;
            } else if self.vec[i].start < new.start && self.vec[i].end > new.end {
                self.vec.push(Interval::new(self.vec[i].start, new.start - 1));
                self.vec.push(Interval::new(new.end + 1, self.vec[i].end));
                self.vec.remove(i);
                self.align();
                //println!("    Inside, creating two new invervals");
                return;
            } else if new.start <= self.vec[i].start {
                //println!("    Overlaps on left side");
                self.vec[i].start = new.end + 1;
            } else if new.start <= self.vec[i].end {
                //println!("    Overlaps on right side");
                self.vec[i].end = new.start - 1;
            } else {
                panic!("More to go! Should remove {:?} from {:?}", new, self.vec[i]);
            }

            i += 1;
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_intervals() {
        let mut intervals = Intervals::new();

        assert_eq!(None, intervals.get_start());
        assert_eq!(None, intervals.get_end());

        intervals.add(Interval::new(2, 5));
        assert_eq!(Some(&Interval::new(2, 5)), intervals.get_start());
        assert_eq!(Some(&Interval::new(2, 5)), intervals.get_end());

        intervals.add(Interval::new(8, 9));
        assert_eq!(Some(&Interval::new(2, 5)), intervals.get_start());
        assert_eq!(Some(&Interval::new(8, 9)), intervals.get_end());

        intervals.add(Interval::new(1, 10));
        assert_eq!(Some(&Interval::new(1, 10)), intervals.get_start());
        assert_eq!(Some(&Interval::new(1, 10)), intervals.get_end());

        intervals.add(Interval::new(5, 15));
        assert_eq!(Some(&Interval::new(1, 15)), intervals.get_start());
        assert_eq!(Some(&Interval::new(1, 15)), intervals.get_end());

        //Remove left side
        intervals.remove(Interval::new(0, 2));
        assert_eq!(Some(&Interval::new(3, 15)), intervals.get_start());
        assert_eq!(Some(&Interval::new(3, 15)), intervals.get_end());

        //remove right side
        intervals.remove(Interval::new(13, 16));
        assert_eq!(Some(&Interval::new(3, 12)), intervals.get_start());
        assert_eq!(Some(&Interval::new(3, 12)), intervals.get_end());

        //Remove middle
        intervals.remove(Interval::new(5, 8));
        assert_eq!(Some(&Interval::new(3, 4)), intervals.get_start());
        assert_eq!(Some(&Interval::new(9, 12)), intervals.get_end());

        //Remove all
        intervals.remove(Interval::new(0, 15));
        assert_eq!(None, intervals.get_start());
        assert_eq!(None, intervals.get_end());
    }

    #[test]
    fn test_intervals_1() {
        let mut intervals = Intervals::new();

        intervals.add(Interval::new(0, 1));
        intervals.add(Interval::new(3, 20));
        intervals.remove(Interval::new(3, 13));
        assert_eq!(Some(&Interval::new(0, 1)), intervals.get_start());
        assert_eq!(Some(&Interval::new(14, 20)), intervals.get_end());
    }
}
