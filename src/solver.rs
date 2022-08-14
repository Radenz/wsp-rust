use std::ops::Range;

use crate::types::{Direction, Matrix, Point, DIRECTIONS};

pub const UNDEFINED_COORD: Point = Point::new(-1, -1);

pub struct Solver<'a> {
    puzzle: &'a Matrix<char>,
    comparisons: i32,
    optimization: bool,
}

impl<'a> Solver<'a> {
    pub fn new(puzzle: &'a Matrix<char>) -> Self {
        Solver {
            puzzle,
            comparisons: 0,
            optimization: false,
        }
    }

    pub fn optimize(&mut self) {
        self.optimization = true;
    }

    pub fn search(&mut self, string: &String) -> SolveResult {
        self.comparisons = 0;
        let mut res: Point = UNDEFINED_COORD;
        let mut direction = Direction::Leftwards;

        for dir in DIRECTIONS.iter() {
            direction = *dir;
            res = self.search_at_dir(string, *dir);

            if res != UNDEFINED_COORD {
                break;
            };
        }

        SolveResult {
            found_at: res,
            dir: direction,
            comparisons: self.comparisons,
        }
    }

    fn get_horizontal_range(&self, length: i32, dir: Direction) -> Range<usize> {
        if dir.is_horizontally_static() || !self.optimization {
            0..self.puzzle.cols
        } else {
            let inc = length - 1;
            let start = if dir.is_leftwards() { inc } else { 0 };
            let mut end = self.puzzle.cols as i32;
            end -= if dir.is_rightwards() { inc } else { 0 };
            (start as usize)..(end as usize)
        }
    }

    fn get_vertical_range(&self, length: i32, dir: Direction) -> Range<usize> {
        if dir.is_vertically_static() || !self.optimization {
            0..self.puzzle.rows
        } else {
            let inc = length - 1;
            let start = if dir.is_upwards() { inc } else { 0 };
            let mut end = self.puzzle.cols as i32;
            end -= if dir.is_downwards() { inc } else { 0 };
            (start as usize)..(end as usize)
        }
    }

    fn search_at_dir(&mut self, string: &String, dir: Direction) -> Point {
        let vrange = self.get_vertical_range(string.len() as i32, dir);
        let hrange = self.get_horizontal_range(string.len() as i32, dir);

        for i in vrange {
            for j in hrange.clone() {
                if self.match_at(
                    string,
                    Point {
                        x: i as i32,
                        y: j as i32,
                    },
                    dir,
                ) {
                    return Point {
                        x: i as i32,
                        y: j as i32,
                    };
                }
            }
        }

        UNDEFINED_COORD
    }

    fn match_at(&mut self, string: &String, coord: Point, dir: Direction) -> bool {
        let h_inc = if dir.is_horizontally_static() {
            0
        } else if dir.is_rightwards() {
            1
        } else {
            -1
        };

        let v_inc = if dir.is_vertically_static() {
            0
        } else if dir.is_downwards() {
            1
        } else {
            -1
        };

        let Point { mut x, mut y } = coord;
        let mut i = 0;
        let mut matched = true;
        let char_vec: Vec<char> = string.chars().collect();
        let length = char_vec.len();

        while matched && i < length {
            if !self.puzzle.has_indices(x as usize, y as usize) {
                matched = false;
                break;
            }

            let char_to_search = char_vec.get(i).unwrap().to_owned();
            let char_in_puzzle = self.puzzle.get(x as usize, y as usize).unwrap();

            self.comparisons += 1;

            if char_to_search == char_in_puzzle {
                i += 1;
                x += v_inc;
                y += h_inc;
            } else {
                matched = false;
            }
        }

        matched
    }
}

pub struct SolveResult {
    pub found_at: Point,
    pub dir: Direction,
    pub comparisons: i32,
}
