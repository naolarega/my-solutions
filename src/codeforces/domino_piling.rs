use std::io::stdin;

pub struct Solution;

impl Solution {
    pub fn domino_piling() {
        let board_dimension = Self::accept_board_size();

        let mut game_board = GameBoard::new(board_dimension);

        let maximum_dominos = game_board.max_dominos();

        println!("{maximum_dominos}");
    }

    fn accept_board_size() -> BoardDimension {
        let mut board_size_input = String::new();

        stdin()
            .read_line(&mut board_size_input)
            .unwrap();

        let ref mut split_board_size = board_size_input.split(" ");

        let length_str = split_board_size.next();
        let width_str = split_board_size.next();
    
        let length = Self::parse_dimension(length_str);
        let width = Self::parse_dimension(width_str);

        BoardDimension { length, width }
    }

    fn parse_dimension(dimension: Option<&str>) -> i16 {
        if let Some(dimension) = dimension {
            let dimension: i16 = dimension
                .trim()
                .parse()
                .unwrap();

            return dimension;
        } else {
            return 0;
        }
    }
}

struct BoardDimension {
    length: i16,
    width: i16,
}

struct Domino(i16, i16);

impl From<(i16, i16)> for Domino {
    fn from(coordinate: (i16, i16)) -> Self {
        Self(coordinate.0, coordinate.1)
    }
}

struct GameBoard {
    board_dimension: BoardDimension,
    dominos: Vec<Domino>,
}

impl GameBoard {
    fn new(board_dimension: BoardDimension) -> Self {
        Self {
            board_dimension,
            dominos: Vec::new(),
        }
    }

    fn max_dominos(&mut self) -> i16 {
        if !self.board_valid() {
            panic!("board dimensions are not valid!");
        }

        let ref mut coordinate = (0, 0);
    
        while self.can_fit_domino(coordinate) {
            self.dominos.push(Domino::from(*coordinate))
        }

        return self.dominos.len() as i16;
    }

    fn can_fit_domino(&self, coordinate: &mut (i16, i16)) -> bool {
        let left_space = (self.board_dimension.length * self.board_dimension.width) - (self.dominos.len() * 2) as i16;

        if left_space >= 2 {
            coordinate.0 = 0;
            coordinate.1 = 0;
            
            return true;
        }
        return false;
    }

    fn board_valid(&self) -> bool {
        self.board_dimension.length >= 1
            && self.board_dimension.length <= self.board_dimension.width
            && self.board_dimension.width <= 16
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_domino_piling() {
        let mut game_board = GameBoard::new(BoardDimension {
            length: 2,
            width: 4
        });
        assert_eq!(game_board.max_dominos(), 4);
    }
}