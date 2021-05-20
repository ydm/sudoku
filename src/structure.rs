use std::fmt::{Display, Error, Formatter};
use std::str::FromStr;

// +--------+
// | Traits |
// +--------+

pub trait Rule {
    fn begin(&mut self) {}
    fn predicate(&self, target: usize, origin: usize) -> bool;
    fn consider(&mut self, _target: &mut Cell, _other: Cell) {}
    fn end(&self, _target: &mut Cell) {}
}

pub trait Solver {
    fn solve(&mut self) -> u8;
}

// +------+
// | Cell |
// +------+

#[derive(Clone, Copy, Debug)]
pub struct Cell {
    // Cell value
    value: u8,

    // Bitmask:
    // 128 64 32 16 8 4 2 1 0  # Bit value
    //   8  7  6  5 4 3 2 1 0  # Bit position
    //   9  8  7  6 5 4 3 2 1  # Corresponding cell value
    options: u16,
}

impl Cell {
    pub fn set(&mut self, value: u8) {
        self.value = value;
        self.options = 0;
        self.invariant();
    }

    pub fn shut(&mut self, value: u8) {
        self.options &= !Self::mask(value);
        self.invariant();
    }

    pub fn open(&mut self, value: u8) {
        self.options |= Self::mask(value);
        self.invariant();
    }

    pub fn single(&mut self, value: u8) {
        self.options = 0;
        self.open(value);
    }

    pub fn is_open(&self, value: u8) -> bool {
        (self.options & Self::mask(value)) != 0
    }

    pub fn num_options(&self) -> u32 {
        self.options.count_ones()
    }

    pub fn value(&self) -> u8 {
        self.value
    }

    pub fn options(&self) -> u16 {
        self.options
    }

    fn mask(value: u8) -> u16 {
        debug_assert!(1 <= value);
        debug_assert!(value <= 9);
        1u16 << (value - 1)
    }

    /// This is called after each internal change of Cell.
    fn invariant(&self) {
        // Produces a better stack trace, but leaves code in
        // production.
        //
        // if self.options == 0 {
        //     debug_assert!(1 <= self.value);
        //     debug_assert!(self.value <= 9);
        // } else {
        //     debug_assert!(self.value == 0);
        // }
        debug_assert!(
            self.options == 0 && 1 <= self.value && self.value <= 9
                || self.options != 0 && self.value == 0
        );
    }
}

impl Default for Cell {
    fn default() -> Self {
        Cell {
            value: 0,
            options: 0b1_1111_1111,
        }
    }
}

impl Solver for Cell {
    fn solve(&mut self) -> u8 {
        // If there is only one value this Cell can posses, set it.
        if self.options.count_ones() == 1 {
            self.value = (self.options.trailing_zeros() + 1) as u8;
            self.options = 0;
            self.invariant();
            1
        } else {
            0
        }
    }
}

// +--------+
// | Sudoku |
// +--------+

pub struct Sudoku {
    pub cells: [Cell; 81],
    pub rules: Vec<Box<dyn Rule>>,
}

impl Sudoku {
    pub fn is_solved(&self) -> bool {
        self.cells
            .iter()
            .map(|x| (x.value() != 0) as usize)
            .sum::<usize>()
            == self.cells.len()
    }

    pub fn fork(&self) -> Vec<Sudoku> {
        let min = self
            .cells
            .iter()
            .enumerate()
            .min_by_key(|(_, x)| x.num_options());
        match min {
            Some((_index, cell)) => {
                let xs = vec![];
                for value in 1u8..=9 {
                    if cell.is_open(value) {
                        // TODO: duplicate()
                    }
                }
                xs
            }
            None => {
                vec![]
            }
        }
    }
}

impl Default for Sudoku {
    fn default() -> Self {
        Sudoku {
            cells: [Default::default(); 81],
            rules: vec![],
        }
    }
}

impl Display for Sudoku {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        //  9 1 2 | 3 4 5 | 6 7 8
        //  3 4 5 | 6 7 8 | 9 1 2
        //  6 7 8 | 1 2 3 | 6 4 1
        // -------+-------+-------
        for (index, c) in self.cells.iter().enumerate() {
            if index != 0 {
                if index % 9 == 0 {
                    writeln!(f)?;
                    if index % 27 == 0 {
                        writeln!(f, "-------+-------+-------")?;
                    }
                } else if index % 3 == 0 {
                    write!(f, " |")?;
                }
            }
            if c.value != 0 {
                write!(f, " {}", c.value)?;
            } else {
                write!(f, "  ")?;
            }
        }
        Ok(())
    }
}

impl Solver for Sudoku {
    fn solve(&mut self) -> u8 {
        let mut sum = 0u8;
        let mut iterations: u8 = 0;

        while !self.is_solved() && iterations < 3 {
            for target in 0..self.cells.len() {
                // For each new target, begin() is called.
                for rule in &mut self.rules {
                    rule.begin();
                }
                for other in 0..self.cells.len() {
                    let copy = self.cells[other];
                    let mut x = &mut self.cells[target];

                    for rule in &mut self.rules {
                        if target != other && rule.predicate(target, other) {
                            rule.consider(&mut x, copy);
                        }
                    }
                }
                // For each target, end is called after successful
                // iteration over all other cells.
                for rule in &self.rules {
                    rule.end(&mut self.cells[target]);
                }
            }

            // Count how many cells are newly solved.  In case 0, we
            // still want to continue as options might be updated.
            let solved: u8 = self.cells.iter_mut().map(Cell::solve).sum();
            sum += solved;
            iterations += (solved <= 0) as u8;
        }
        sum
    }
}

#[derive(Debug)]
pub struct ParseSudokuError {}

impl FromStr for Sudoku {
    type Err = ParseSudokuError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Collect only digits out of the given input string.
        let xs: Vec<u8> = s
            .chars()
            .map(|x| x.to_digit(10))
            .filter(Option::is_some)
            .map(|x| x.unwrap() as u8)
            .collect();

        // Make sure we have the exact number of digits we need.
        if xs.len() != 81 {
            return Err(ParseSudokuError {});
        }

        // Set each non-0 digit at the corresponding cell.
        let mut s = Sudoku::default();
        for (index, value) in xs.iter().enumerate().filter(|(_, x)| **x != 0) {
            s.cells[index].set(*value);
        }
        Ok(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::presets;

    #[test]
    fn test_cell_is_open() {
        let c = Cell::default();
        assert_eq!(c.num_options(), 9);
        for i in 1..=9 {
            assert!(c.is_open(i), "i={}", i);
        }
    }

    #[test]
    #[should_panic]
    fn test_cell_is_open_panic() {
        // TODO: Why isn't there should_panic_debug()?  Tests in
        // release will fail.
        let c = Cell::default();
        c.is_open(0);
    }

    #[test]
    #[should_panic]
    fn test_cell_is_open_panic2() {
        let c = Cell::default();
        c.is_open(10);
    }

    #[test]
    fn test_cell_shut() {
        let mut c = Cell::default();
        assert!(c.options() == 0b1_1111_1111);
        c.shut(7);
        assert!(c.options() == 0b1_1011_1111);
        c.shut(9);
        assert!(c.options() == 0b0_1011_1111);
    }

    #[test]
    fn test_cell_shut_open() {
        let mut c = Cell::default();
        assert!(c.is_open(1));
        c.shut(1);
        assert!(!c.is_open(1));
        c.open(1);
        assert!(c.is_open(1));
    }

    #[test]
    fn test_cell_solve() {
        // Crate a cell.
        let mut c = Cell::default();
        assert_eq!(c.num_options(), 9);
        assert_eq!(c.solve(), 0);

        // Shut all options except for x.
        let x = 7;
        for i in 1..=9 {
            if i != x {
                c.shut(i);
            }
        }
        assert_eq!(c.num_options(), 1);

        // State before solve()ing.
        assert_eq!(c.options, 0b0_0100_0000);
        assert_eq!(c.value, 0);

        // Solve.
        assert_eq!(c.solve(), 1);

        // State after solve()ing.
        assert_eq!(c.options, 0b0_0000_0000);
        assert_eq!(c.value, x);

        // Make sure calling solve() again returns 0 and does not
        // alter the state.
        assert_eq!(c.solve(), 0);
        assert_eq!(c.options, 0b0_0000_0000);
        assert_eq!(c.value, x);
    }

    #[test]
    fn test_sudoku_display() {
        let have = presets::load_easy().to_string();
        let want = "
 3 4   |       |   7  
 8     | 4   7 | 2 5  
 7   6 | 8     | 3   9
-------+-------+-------
   1 3 |     6 | 4    
     7 |     4 |   1  
     4 |       | 6   3
-------+-------+-------
   7 9 | 6 5   | 1   2
       | 7     | 5 9 8
   3   | 2 9 1 | 7    "
            .strip_prefix('\n')
            .unwrap();
        assert_eq!(have, want);
    }

    #[test]
    fn test_sudoku_solve() {
        let mut s = Sudoku::default();

        // Make sure no solve is possible.
        assert_eq!(s.solve(), 0);
        assert_eq!(s.cells[0].value, 0);
        for i in 1..=8 {
            s.cells[0].shut(i)
        }

        // Make sure solve() now works.
        assert_eq!(s.solve(), 1);
        assert_eq!(s.cells[0].value, 9);
    }

    #[test]
    fn test_sudoku_parse() {
        let s: Sudoku = "
0 0 7 0 0 0 0 0 5
5 0 0 4 2 0 0 0 1
0 4 0 0 0 5 6 0 0
6 0 5 1 0 0 0 0 0
0 0 0 0 0 8 0 0 0
2 0 0 0 0 0 0 8 0
9 2 0 0 7 0 0 5 0
0 7 3 0 0 6 0 0 0
0 0 1 0 0 9 0 0 2"
            .parse()
            .unwrap();
        assert_eq!(s.cells[0].value(), 0);
        assert_eq!(s.cells[1].value(), 0);
        assert_eq!(s.cells[2].value(), 7);
        assert_eq!(s.cells[3].value(), 0);
        assert_eq!(s.cells[4].value(), 0);
        assert_eq!(s.cells[5].value(), 0);
        assert_eq!(s.cells[6].value(), 0);
        assert_eq!(s.cells[7].value(), 0);
        assert_eq!(s.cells[8].value(), 5);
        assert_eq!(s.cells[9].value(), 5);
        assert_eq!(s.cells[10].value(), 0);
        assert_eq!(s.cells[11].value(), 0);
        assert_eq!(s.cells[12].value(), 4);
        assert_eq!(s.cells[13].value(), 2);
        assert_eq!(s.cells[14].value(), 0);
        assert_eq!(s.cells[15].value(), 0);
        assert_eq!(s.cells[16].value(), 0);
        assert_eq!(s.cells[17].value(), 1);
        assert_eq!(s.cells[18].value(), 0);
        assert_eq!(s.cells[19].value(), 4);
        assert_eq!(s.cells[20].value(), 0);
        assert_eq!(s.cells[21].value(), 0);
        assert_eq!(s.cells[22].value(), 0);
        assert_eq!(s.cells[23].value(), 5);
        assert_eq!(s.cells[24].value(), 6);
        assert_eq!(s.cells[25].value(), 0);
        assert_eq!(s.cells[26].value(), 0);
        assert_eq!(s.cells[27].value(), 6);
        assert_eq!(s.cells[28].value(), 0);
        assert_eq!(s.cells[29].value(), 5);
        assert_eq!(s.cells[30].value(), 1);
        assert_eq!(s.cells[31].value(), 0);
        assert_eq!(s.cells[32].value(), 0);
        assert_eq!(s.cells[33].value(), 0);
        assert_eq!(s.cells[34].value(), 0);
        assert_eq!(s.cells[35].value(), 0);
        assert_eq!(s.cells[36].value(), 0);
        assert_eq!(s.cells[37].value(), 0);
        assert_eq!(s.cells[38].value(), 0);
        assert_eq!(s.cells[39].value(), 0);
        assert_eq!(s.cells[40].value(), 0);
        assert_eq!(s.cells[41].value(), 8);
        assert_eq!(s.cells[42].value(), 0);
        assert_eq!(s.cells[43].value(), 0);
        assert_eq!(s.cells[44].value(), 0);
        assert_eq!(s.cells[45].value(), 2);
        assert_eq!(s.cells[46].value(), 0);
        assert_eq!(s.cells[47].value(), 0);
        assert_eq!(s.cells[48].value(), 0);
        assert_eq!(s.cells[49].value(), 0);
        assert_eq!(s.cells[50].value(), 0);
        assert_eq!(s.cells[51].value(), 0);
        assert_eq!(s.cells[52].value(), 8);
        assert_eq!(s.cells[53].value(), 0);
        assert_eq!(s.cells[54].value(), 9);
        assert_eq!(s.cells[55].value(), 2);
        assert_eq!(s.cells[56].value(), 0);
        assert_eq!(s.cells[57].value(), 0);
        assert_eq!(s.cells[58].value(), 7);
        assert_eq!(s.cells[59].value(), 0);
        assert_eq!(s.cells[60].value(), 0);
        assert_eq!(s.cells[61].value(), 5);
        assert_eq!(s.cells[62].value(), 0);
        assert_eq!(s.cells[63].value(), 0);
        assert_eq!(s.cells[64].value(), 7);
        assert_eq!(s.cells[65].value(), 3);
        assert_eq!(s.cells[66].value(), 0);
        assert_eq!(s.cells[67].value(), 0);
        assert_eq!(s.cells[68].value(), 6);
        assert_eq!(s.cells[69].value(), 0);
        assert_eq!(s.cells[70].value(), 0);
        assert_eq!(s.cells[71].value(), 0);
        assert_eq!(s.cells[72].value(), 0);
        assert_eq!(s.cells[73].value(), 0);
        assert_eq!(s.cells[74].value(), 1);
        assert_eq!(s.cells[75].value(), 0);
        assert_eq!(s.cells[76].value(), 0);
        assert_eq!(s.cells[77].value(), 9);
        assert_eq!(s.cells[78].value(), 0);
        assert_eq!(s.cells[79].value(), 0);
        assert_eq!(s.cells[80].value(), 2);
    }
}
