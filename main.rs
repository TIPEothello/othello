use std::fmt::Display;

enum Case {
	Empty,
	White,
	Black
}

struct Board {
	cases: Vec<Vec<Case>>
	
}

impl Board {
    fn new() -> Self {
        Board {
            cases: vec![vec![Case::Empty; 8]; 8];
        }
    }

    fn 
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut string = String::new();
        for line in self.cases.iter() {
            for case in line.iter() {
                    match case {
                        Case::Empty => {
                            string += "\033[91m□\033[0m";
                        },
                        Case::White => {
                            string += "\033[97m□\033[0m";
                        },
                        Case::Black => {
                            string += "\033[30m□\033[0m";
                        },
                }
            }
            string += "\n";
        }
        write!(f, string)
    }
}

fn main() {
	let board = Board::new();
    println!("{}", board);
}