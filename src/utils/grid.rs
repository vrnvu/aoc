use std::ops::{Add, Mul};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vector {
    pub i: isize,
    pub j: isize,
}

impl Vector {
    pub fn new(i: usize, j: usize) -> Self {
        Self {
            i: i as isize,
            j: j as isize,
        }
    }
}

pub const UP: Vector = Vector { i: -1, j: 0 };
pub const DOWN: Vector = Vector { i: 1, j: 0 };
pub const LEFT: Vector = Vector { i: 0, j: -1 };
pub const RIGHT: Vector = Vector { i: 0, j: 1 };
pub const UP_LEFT: Vector = Vector { i: -1, j: -1 };
pub const UP_RIGHT: Vector = Vector { i: -1, j: 1 };
pub const DOWN_LEFT: Vector = Vector { i: 1, j: -1 };
pub const DOWN_RIGHT: Vector = Vector { i: 1, j: 1 };

impl Add for Vector {
    type Output = Self;

    #[inline]
    fn add(self, other: Self) -> Self {
        Self {
            i: self.i + other.i,
            j: self.j + other.j,
        }
    }
}

impl Mul<isize> for Vector {
    type Output = Self;

    #[inline]
    fn mul(self, other: isize) -> Self {
        Self {
            i: self.i * other,
            j: self.j * other,
        }
    }
}

pub struct Grid<T> {
    grid: Vec<Vec<T>>,
    length: usize,
    width: usize,
}

impl<T> Grid<T> {
    pub fn new(grid: Vec<Vec<T>>) -> Self {
        let length = grid.len();
        let width = grid[0].len();
        Self {
            grid,
            length,
            width,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.grid.is_empty()
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn iter_positions(&self) -> impl Iterator<Item = (Vector, &T)> {
        (0..self.length)
            .flat_map(move |i| (0..self.width).map(move |j| (Vector::new(i, j), &self.grid[i][j])))
    }

    pub fn get(&self, vector: Vector) -> Option<&T> {
        let new_i = vector.i;
        let new_j = vector.j;

        if new_i < 0 || new_i >= self.length as isize || new_j < 0 || new_j >= self.width as isize {
            return None;
        }

        Some(&self.grid[new_i as usize][new_j as usize])
    }

    pub fn get_from(&self, origin: Vector, vector: Vector) -> Option<&T> {
        let new_i = origin.i + vector.i;
        let new_j = origin.j + vector.j;

        if new_i < 0 || new_i >= self.length as isize || new_j < 0 || new_j >= self.width as isize {
            return None;
        }

        Some(&self.grid[new_i as usize][new_j as usize])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_get() {
        let grid = Grid::new(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);
        assert_eq!(grid.get(Vector { i: 0, j: 0 }), Some(&1));
        assert_eq!(grid.get(Vector { i: 0, j: 1 }), Some(&2));
        assert_eq!(grid.get(Vector { i: 0, j: 2 }), Some(&3));
        assert_eq!(grid.get(Vector { i: 1, j: 0 }), Some(&4));
        assert_eq!(grid.get(Vector { i: 1, j: 1 }), Some(&5));
        assert_eq!(grid.get(Vector { i: 1, j: 2 }), Some(&6));
        assert_eq!(grid.get(Vector { i: 2, j: 0 }), Some(&7));
        assert_eq!(grid.get(Vector { i: 2, j: 1 }), Some(&8));
        assert_eq!(grid.get(Vector { i: 2, j: 2 }), Some(&9));
        assert_eq!(grid.get(Vector { i: 3, j: 3 }), None);
    }

    #[test]
    fn test_grid_get_from() {
        // 1 2 3
        // 4 5 6
        // 7 8 9
        let grid = Grid::new(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);

        // Define expected values for each position and direction
        let tests = [
            ((0, 0), UP, None),
            ((0, 0), DOWN, Some(&4)),
            ((0, 0), LEFT, None),
            ((0, 0), RIGHT, Some(&2)),
            ((0, 0), UP_LEFT, None),
            ((0, 0), UP_RIGHT, None),
            ((0, 0), DOWN_LEFT, None),
            ((0, 0), DOWN_RIGHT, Some(&5)),
            ((0, 1), UP, None),
            ((0, 1), DOWN, Some(&5)),
            ((0, 1), LEFT, Some(&1)),
            ((0, 1), RIGHT, Some(&3)),
            ((0, 1), UP_LEFT, None),
            ((0, 1), UP_RIGHT, None),
            ((0, 1), DOWN_LEFT, Some(&4)),
            ((0, 1), DOWN_RIGHT, Some(&6)),
            ((0, 2), UP, None),
            ((0, 2), DOWN, Some(&6)),
            ((0, 2), LEFT, Some(&2)),
            ((0, 2), RIGHT, None),
            ((0, 2), UP_LEFT, None),
            ((0, 2), UP_RIGHT, None),
            ((0, 2), DOWN_LEFT, Some(&5)),
            ((0, 2), DOWN_RIGHT, None),
            ((1, 0), UP, Some(&1)),
            ((1, 0), DOWN, Some(&7)),
            ((1, 0), LEFT, None),
            ((1, 0), RIGHT, Some(&5)),
            ((1, 0), UP_LEFT, None),
            ((1, 0), UP_RIGHT, Some(&2)),
            ((1, 0), DOWN_LEFT, None),
            ((1, 0), DOWN_RIGHT, Some(&8)),
            ((1, 1), UP, Some(&2)),
            ((1, 1), DOWN, Some(&8)),
            ((1, 1), LEFT, Some(&4)),
            ((1, 1), RIGHT, Some(&6)),
            ((1, 1), UP_LEFT, Some(&1)),
            ((1, 1), UP_RIGHT, Some(&3)),
            ((1, 1), DOWN_LEFT, Some(&7)),
            ((1, 1), DOWN_RIGHT, Some(&9)),
            ((1, 2), UP, Some(&3)),
            ((1, 2), DOWN, Some(&9)),
            ((1, 2), LEFT, Some(&5)),
            ((1, 2), RIGHT, None),
            ((1, 2), UP_LEFT, Some(&2)),
            ((1, 2), UP_RIGHT, None),
            ((1, 2), DOWN_LEFT, Some(&8)),
            ((1, 2), DOWN_RIGHT, None),
            ((2, 0), UP, Some(&4)),
            ((2, 0), DOWN, None),
            ((2, 0), LEFT, None),
            ((2, 0), RIGHT, Some(&8)),
            ((2, 0), UP_LEFT, None),
            ((2, 0), UP_RIGHT, Some(&5)),
            ((2, 0), DOWN_LEFT, None),
            ((2, 0), DOWN_RIGHT, None),
            ((2, 1), UP, Some(&5)),
            ((2, 1), DOWN, None),
            ((2, 1), LEFT, Some(&7)),
            ((2, 1), RIGHT, Some(&9)),
            ((2, 1), UP_LEFT, Some(&4)),
            ((2, 1), UP_RIGHT, Some(&6)),
            ((2, 1), DOWN_LEFT, None),
            ((2, 1), DOWN_RIGHT, None),
            ((2, 2), UP, Some(&6)),
            ((2, 2), DOWN, None),
            ((2, 2), LEFT, Some(&8)),
            ((2, 2), RIGHT, None),
            ((2, 2), UP_LEFT, Some(&5)),
            ((2, 2), UP_RIGHT, None),
            ((2, 2), DOWN_LEFT, None),
            ((2, 2), DOWN_RIGHT, None),
        ];

        for (origin, dir, expected) in tests {
            assert_eq!(
                grid.get_from(
                    Vector {
                        i: origin.0,
                        j: origin.1
                    },
                    dir
                ),
                expected,
                "Failed at position {:?} with direction {:?}",
                origin,
                dir
            );
        }
    }

    #[test]
    fn test_vector_addition() {
        // Basic cardinal direction combinations
        assert_eq!(UP + UP, Vector { i: -2, j: 0 });
        assert_eq!(DOWN + DOWN, Vector { i: 2, j: 0 });
        assert_eq!(LEFT + LEFT, Vector { i: 0, j: -2 });
        assert_eq!(RIGHT + RIGHT, Vector { i: 0, j: 2 });

        // Opposite directions cancel out
        assert_eq!(UP + DOWN, Vector { i: 0, j: 0 });
        assert_eq!(LEFT + RIGHT, Vector { i: 0, j: 0 });

        // Diagonal combinations
        assert_eq!(UP + LEFT, UP_LEFT);
        assert_eq!(UP + RIGHT, UP_RIGHT);
        assert_eq!(DOWN + LEFT, DOWN_LEFT);
        assert_eq!(DOWN + RIGHT, DOWN_RIGHT);

        // More complex combinations
        assert_eq!(UP_LEFT + DOWN_RIGHT, Vector { i: 0, j: 0 });
        assert_eq!(UP_RIGHT + DOWN_LEFT, Vector { i: 0, j: 0 });
        assert_eq!(UP_LEFT + RIGHT, Vector { i: -1, j: 0 });
        assert_eq!(DOWN_RIGHT + LEFT, Vector { i: 1, j: 0 });
    }

    #[test]
    fn test_vector_multiplication() {
        // Positive scalar multiplication
        assert_eq!(UP * 2, Vector { i: -2, j: 0 });
        assert_eq!(DOWN * 3, Vector { i: 3, j: 0 });
        assert_eq!(LEFT * 4, Vector { i: 0, j: -4 });
        assert_eq!(RIGHT * 5, Vector { i: 0, j: 5 });

        // Negative scalar multiplication
        assert_eq!(UP * -1, DOWN);
        assert_eq!(LEFT * -1, RIGHT);
        assert_eq!(UP_LEFT * -1, DOWN_RIGHT);
        assert_eq!(UP_RIGHT * -1, DOWN_LEFT);

        // Zero multiplication
        assert_eq!(UP * 0, Vector { i: 0, j: 0 });
        assert_eq!(DOWN_RIGHT * 0, Vector { i: 0, j: 0 });
    }
}
