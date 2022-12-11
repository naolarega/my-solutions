use std::io::stdin;

pub struct Solution;

impl Solution {
    pub fn domino_piling() {
        let board_dimension = Self::accept_board_size().unwrap();

        let mut game_board = GameBoard::new(board_dimension);

        let maximum_dominos = game_board.max_dominos();

        println!("Your board can only fit {maximum_dominos} dominos");
    }

    fn accept_board_size() -> Result<BoardDimension, ()> {
        use std::cmp::Ordering::*;

        let mut board_size_input = String::new();

        stdin()
            .read_line(&mut board_size_input)
            .unwrap();

        let ref mut split_board_size = board_size_input.split(" ");

        if let Equal = split_board_size.count().cmp(&2) {
            let length_str = split_board_size.next();
            let width_str = split_board_size.next();
            
            let length = Self::parse_dimension(length_str);
            let width = Self::parse_dimension(width_str);

            Ok(BoardDimension { length, width })
        } else {
            Err(())
        }
    }

    fn parse_dimension(dimension: Option<&str>) -> i8 {
        if let Some(dimension) = dimension {
            let dimension: i8 = dimension.parse().unwrap();
            return  dimension;
        } else {
            return 0;
        }
    }
}

struct BoardDimension {
    length: i8,
    width: i8
}

struct Domino(i8, i8);

impl From<(i8, i8)> for Domino {
    fn from(coordinate: (i8, i8)) -> Self {
        Self(coordinate.0, coordinate.1)
    }
}

struct GameBoard {
    board_dimension: BoardDimension,
    dominos: Vec<Domino>
}

impl GameBoard {
    fn new(board_dimension: BoardDimension) -> Self {
        Self{ 
            board_dimension,
            dominos: Vec::new()
        }
    }

    fn max_dominos(&mut self) -> i8 {
        if !self.board_valid() {
            panic!("board dimensions are not valid!");
        }
        
        let ref mut coordinate = (0, 0);

        while self.can_fit_domino(coordinate) {
            self.dominos.push(Domino::from(*coordinate))
        }

        return self.dominos.len() as i8;
    }

    fn can_fit_domino(&self, coordinate: &mut (i8, i8)) -> bool {
        false
    }

    fn board_valid(&self) -> bool {
        self.board_dimension.length >= 1 &&
        self.board_dimension.length < self.board_dimension.width &&
        self.board_dimension.width <= 16
    }
}