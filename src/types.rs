use rand::Rng;
use std::fmt::{Display, Formatter, Result};

type Int = i32;

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: Int,
    pub y: Int,
}

impl Point {
    pub const fn new(x: Int, y: Int) -> Self {
        Point { x, y }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

#[derive(Clone)]
pub struct Matrix<T> {
    pub rows: usize,
    pub cols: usize,
    values: Vec<Vec<Option<T>>>,
}

impl<T> Matrix<T> {
    pub fn new(rows: usize, cols: usize) -> Self {
        let mut values: Vec<Vec<Option<T>>> = Vec::new();

        for _ in 0..rows {
            let mut col = Vec::new();
            for _ in 0..cols {
                col.push(None);
            }
            values.push(col);
        }

        Matrix { rows, cols, values }
    }

    pub fn has_indices(&self, row: usize, col: usize) -> bool {
        row < self.rows && col < self.cols
    }

    pub fn set(&mut self, row: usize, col: usize, elem: T) {
        let column = self.values.get_mut(row).unwrap();
        column[col] = Some(elem);
    }

    pub fn get(&self, row: usize, col: usize) -> &Option<T> {
        self.values.get(row).unwrap().get(col).unwrap()
    }

    pub fn get_mut(&mut self, row: usize, col: usize) -> &mut Option<T> {
        self.values.get_mut(row).unwrap().get_mut(col).unwrap()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Leftwards,
    Rightwards,
    Downwards,
    Upwards,
    LeftDownwards,
    RightDownwards,
    LeftUpwards,
    RightUpwards,
}

pub const DIRECTIONS: [Direction; 8] = [
    Direction::Leftwards,
    Direction::Rightwards,
    Direction::Downwards,
    Direction::Upwards,
    Direction::LeftDownwards,
    Direction::RightDownwards,
    Direction::LeftUpwards,
    Direction::RightUpwards,
];

impl Direction {
    pub fn is_leftwards(&self) -> bool {
        match *self {
            Direction::Leftwards => true,
            Direction::LeftDownwards => true,
            Direction::LeftUpwards => true,
            _ => false,
        }
    }

    pub fn is_rightwards(&self) -> bool {
        match *self {
            Direction::Rightwards => true,
            Direction::RightDownwards => true,
            Direction::RightUpwards => true,
            _ => false,
        }
    }

    pub fn is_downwards(&self) -> bool {
        match *self {
            Direction::Downwards => true,
            Direction::LeftDownwards => true,
            Direction::RightDownwards => true,
            _ => false,
        }
    }

    pub fn is_upwards(&self) -> bool {
        match *self {
            Direction::Upwards => true,
            Direction::LeftUpwards => true,
            Direction::RightUpwards => true,
            _ => false,
        }
    }

    pub fn is_horizontally_static(&self) -> bool {
        !self.is_leftwards() && !self.is_rightwards()
    }

    pub fn is_vertically_static(&self) -> bool {
        !self.is_upwards() && !self.is_downwards()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Color {
    code: u32,
}

impl Color {
    const fn new(code: u32) -> Color {
        Color { code }
    }

    pub fn wrap(&self, filler: &String) -> String {
        if self.code == 0 {
            filler.to_owned()
        } else {
            format!("\x1b[1;38;5;{}m{}\x1b[0m", self.code, filler)
        }
    }

    pub fn random() -> Color {
        let i = rand::thread_rng().gen_range(0..9);
        COLORS[i]
    }

    pub fn is_plain(&self) -> bool {
        self.code == 0
    }
}

pub const GREEN: Color = Color::new(10);
pub const RED: Color = Color::new(9);
pub const BLUE: Color = Color::new(45);
pub const MAGENTA: Color = Color::new(13);
pub const CYAN: Color = Color::new(14);
pub const LIME: Color = Color::new(48);
pub const BROWN: Color = Color::new(94);
pub const PURPLE: Color = Color::new(93);
pub const ORANGE: Color = Color::new(202);
pub const PLAIN: Color = Color::new(0);

pub const COLORS: [Color; 9] = [GREEN, RED, BLUE, MAGENTA, CYAN, LIME, BROWN, PURPLE, ORANGE];

pub struct ColoredMatrix<'a, T> {
    matrix: &'a Matrix<T>,
    colors: Matrix<Color>,
}

impl<'a, T> ColoredMatrix<'a, T> {
    pub fn new(matrix: &'a Matrix<T>) -> Self {
        let mut colors: Matrix<Color> = Matrix::new(matrix.rows, matrix.cols);
        for row in 0..matrix.rows {
            for col in 0..matrix.cols {
                colors.set(row, col, PLAIN);
            }
        }

        ColoredMatrix { matrix, colors }
    }

    pub fn colorize(&mut self, coord: Point, dir: Direction, length: Int, color: Color) {
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

        for _ in 0..length {
            self.colors.set(x as usize, y as usize, color);
            x += v_inc;
            y += h_inc;
        }
    }
}

impl<'a, T> Display for ColoredMatrix<'a, T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for i in 0..self.matrix.rows {
            for j in 0..self.matrix.cols {
                if j != 0 {
                    write!(f, " ")?;
                }

                let maybe_filler = self.matrix.get(i, j);
                match maybe_filler {
                    &Some(ref filler) => write!(
                        f,
                        "{}",
                        self.colors
                            .get(i, j)
                            .to_owned()
                            .unwrap()
                            .wrap(&filler.to_string())
                    )?,
                    &None => write!(f, " ")?,
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}
