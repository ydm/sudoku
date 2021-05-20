#[allow(unused_imports)]
use crate::structure::{Cell, Rule};

// +------------+
// | Predicates |
// +------------+

fn same_row(a: usize, b: usize) -> bool {
    a / 9 == b / 9
}

fn same_col(a: usize, b: usize) -> bool {
    a % 9 == b % 9
}

fn same_square(a: usize, b: usize) -> bool {
    let row = |i| i / 27;
    let col = |i| i % 9 / 3;
    row(a) == row(b) && col(a) == col(b)
}

// +---------------+
// | ExclusionRule |
// +---------------+

// Target cell cannot have the value of other cells form the same
// subgroup.
pub struct ExclusionRule {
    predicate_fn: fn(usize, usize) -> bool,
}

impl Rule for ExclusionRule {
    fn predicate(&self, target: usize, other: usize) -> bool {
        (self.predicate_fn)(target, other)
    }

    fn consider(&mut self, target: &mut Cell, other: Cell) {
        if other.value() != 0 {
            target.shut(other.value());
        }
    }
}

impl ExclusionRule {
    pub fn new_row() -> Self {
        ExclusionRule {
            predicate_fn: same_row,
        }
    }

    pub fn new_col() -> Self {
        ExclusionRule {
            predicate_fn: same_col,
        }
    }

    pub fn new_square() -> Self {
        ExclusionRule {
            predicate_fn: same_square,
        }
    }
}

// +---------------+
// | SingleOptRule |
// +---------------+

// Single option rules check if the target cell is the only one in the
// subgroup (row / col / square) that may have a particular value.
pub struct SingleOptRule {
    options: [u8; 10],
    predicate_fn: fn(usize, usize) -> bool,
}

impl SingleOptRule {
    pub fn new_row() -> Self {
        SingleOptRule {
            options: [0; 10],
            predicate_fn: same_row,
        }
    }

    pub fn new_col() -> Self {
        SingleOptRule {
            options: [0; 10],
            predicate_fn: same_col,
        }
    }

    pub fn new_square() -> Self {
        SingleOptRule {
            options: [0; 10],
            predicate_fn: same_square,
        }
    }
}

impl Rule for SingleOptRule {
    fn begin(&mut self) {
        for i in 0..10 {
            self.options[i] = 0;
        }
    }

    fn predicate(&self, target: usize, other: usize) -> bool {
        (self.predicate_fn)(target, other)
    }

    fn consider(&mut self, _target: &mut Cell, other: Cell) {
        if other.value() == 0 {
            for v in 1u8..=9 {
                self.options[v as usize] += other.is_open(v) as u8;
            }
        }
    }

    fn end(&self, target: &mut Cell) {
        for v in 1u8..=9 {
            if self.options[v as usize] == 0 && target.is_open(v) {
                return target.single(v);
            }
        }
    }
}

// +------+
// | Test |
// +------+

#[cfg(test)]
mod test {
    use super::*;
    use crate::presets;
    use crate::structure::{Solver, Sudoku};

    #[test]
    fn test_row_rule() {
        let mut s = presets::load_easy();
        s.rules.push(Box::new(ExclusionRule::new_row()));

        assert_eq!(s.solve(), 0);
        assert_eq!(s.cells[54].value(), 0);

        s.cells[59].set(8);
        assert_eq!(s.solve(), 0);
        assert_eq!(s.cells[54].value(), 0);

        s.cells[61].set(3);
        assert_eq!(s.solve(), 1);
        assert_eq!(s.cells[54].value(), 4);
    }

    #[test]
    fn test_col_rule() {
        let mut s = presets::load_easy();
        s.rules.push(Box::new(ExclusionRule::new_col()));

        assert_eq!(s.solve(), 0);
        assert_eq!(s.cells[42].value(), 0);

        s.cells[6].set(8);
        assert_eq!(s.solve(), 1);
        assert_eq!(s.cells[42].value(), 9);
    }

    #[test]
    fn test_square_rule() {
        let mut s = presets::load_easy();
        s.rules.push(Box::new(ExclusionRule::new_square()));

        assert_eq!(s.solve(), 0);
        assert_eq!(s.cells[59].value(), 0);

        s.cells[67].set(4);
        assert_eq!(s.solve(), 0);
        assert_eq!(s.cells[59].value(), 0);

        s.cells[68].set(3);
        assert_eq!(s.solve(), 1);
        assert_eq!(s.cells[59].value(), 8);
    }

    #[test]
    fn test_single_opt_square_rule() {
        let mut s = presets::load_easy();
        s.rules.push(Box::new(SingleOptRule::new_square()));

        // Make sure it's still unsolvable.
        assert_eq!(s.solve(), 0);
        assert_eq!(s.cells[67].value(), 0);

        // Shut down the possibility of cell #59 to have a 4.
        assert!(s.cells[59].is_open(4));
        s.cells[59].shut(4);
        assert_eq!(s.cells[59].is_open(4), false);

        // Make sure it's still unsolvable.
        assert_eq!(s.solve(), 0);
        assert_eq!(s.cells[67].value(), 0);

        // Shut down the possibility of cell #68 to have a 4.
        assert!(s.cells[68].is_open(4));
        s.cells[68].shut(4);
        assert_eq!(s.cells[68].is_open(4), false);

        // Make sure it's now solvable.
        assert_eq!(s.solve(), 1);
        assert_eq!(s.cells[67].value(), 4);
    }

    #[test]
    fn test_rules() {
        let mut s = presets::load_easy();
        s.rules.push(Box::new(ExclusionRule::new_row()));
        s.rules.push(Box::new(ExclusionRule::new_col()));
        s.rules.push(Box::new(ExclusionRule::new_square()));
        s.rules.push(Box::new(SingleOptRule::new_row()));
        s.rules.push(Box::new(SingleOptRule::new_col()));
        s.rules.push(Box::new(SingleOptRule::new_square()));
        assert_eq!(s.solve(), 43);
    }
}
