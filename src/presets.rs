use crate::structure::Sudoku;

pub fn load_easy() -> Sudoku {
    let mut s = Sudoku::default();
    s.cells[0].set(3);
    s.cells[1].set(4);
    s.cells[7].set(7);
    s.cells[9].set(8);
    s.cells[12].set(4);
    s.cells[14].set(7);
    s.cells[15].set(2);
    s.cells[16].set(5);
    s.cells[18].set(7);
    s.cells[20].set(6);
    s.cells[21].set(8);
    s.cells[24].set(3);
    s.cells[26].set(9);
    s.cells[28].set(1);
    s.cells[29].set(3);
    s.cells[32].set(6);
    s.cells[33].set(4);
    s.cells[38].set(7);
    s.cells[41].set(4);
    s.cells[43].set(1);
    s.cells[47].set(4);
    s.cells[51].set(6);
    s.cells[53].set(3);
    s.cells[55].set(7);
    s.cells[56].set(9);
    s.cells[57].set(6);
    s.cells[58].set(5);
    s.cells[60].set(1);
    s.cells[62].set(2);
    s.cells[66].set(7);
    s.cells[69].set(5);
    s.cells[70].set(9);
    s.cells[71].set(8);
    s.cells[73].set(3);
    s.cells[75].set(2);
    s.cells[76].set(9);
    s.cells[77].set(1);
    s.cells[78].set(7);
    s
}

pub fn load_hard() -> Sudoku {
    "0 0 7 0 0 0 0 0 5
     5 0 0 4 2 0 0 0 1
     0 4 0 0 0 5 6 0 0
     6 0 5 1 0 0 0 0 0
     0 0 0 0 0 8 0 0 0
     2 0 0 0 0 0 0 8 0
     9 2 0 0 7 0 0 5 0
     0 7 3 0 0 6 0 0 0
     0 0 1 0 0 9 0 0 2"
        .parse()
        .unwrap()
}

pub fn load_hard2() -> Sudoku {
    " 1 0 7 | 6 4 9 | 3 0 2
      2 0 0 | 3 7 5 | 0 1 6
      3 0 6 | 8 2 1 | 4 0 0
     -------+-------+-------
      0 1 0 | 0 9 6 | 7 3 8
      7 6 3 | 0 1 8 | 0 4 9
      0 0 0 | 0 3 7 | 6 0 1
     -------+-------+-------
      6 7 0 | 1 5 3 | 0 0 4
      8 3 1 | 9 6 0 | 0 0 0
      0 0 0 | 7 8 0 | 1 6 3"
        .parse()
        .unwrap()
}

pub fn load_expert() -> Sudoku {
    "0 4 0 9 0 0 0 0 7
     1 9 0 6 0 0 0 0 4
     5 0 0 0 0 0 0 1 0
     0 8 0 0 3 0 0 7 0
     2 0 0 0 0 4 5 0 8
     0 0 0 5 0 0 0 0 0
     0 0 0 0 0 0 0 2 0
     0 0 0 0 0 0 3 4 0
     0 7 0 0 0 6 0 0 1"
        .parse()
        .unwrap()
}
