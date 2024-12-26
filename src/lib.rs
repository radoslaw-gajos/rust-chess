type Piece = (char, char);
type Board = [[Piece; 8]; 8];
type Coord = (u8, u8);
type Indices = (usize, usize);

pub mod app;

fn mve(from: Coord, to: Coord, board: Board) -> Board {
    change_position(from, to, board)
}

fn change_position(from: Coord, to: Coord, board: Board) -> Board {
    let mut board = board;

    let piece = get_piece(from, &board);
    board = remove_piece(from, board);
    board = insert_piece(to, piece, board);

    board
}

fn get_piece(coord: Coord, board: &Board) -> Piece {
    let (column, row) = indices(coord);
    let piece = board[row][column];
    piece
}

fn remove_piece(coord: Coord, board: Board) -> Board {
    let mut board = board;

    let (column, row) = indices(coord);
    board[row][column] = (' ',' ');
    board
}

fn insert_piece(coord: Coord, piece: Piece, board: Board) -> Board {
    let mut board = board;

    let (column, row) = indices(coord);
    board[row][column] = piece;
    board
}

/// Transforms human readable form to indeces. Invalid coordinates
/// will make the code panic.
fn indices(coord: Coord) -> Indices {
    let (column, row) = coord;

    if column < b'A' || column > b'H' {
        panic!("Invalid column in coordinate: Got {column:?}, expected value between b'A' \
        and b'H'.");
    }
    if row < b'1' || row > b'8' {
        panic!("Invalid column in coordinate: Got {column:?}, expected value between b'A' \
        and b'H'.");
    }

    let column = column - b'A';
    let row = 8 - (row - b'0');

    (column.into(), row.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn should_translate_coord_to_indices() {
        // given
        let coord = (b'A',b'1');
        let expected = (0,7);

        // when
        let indices = indices(coord);
        
        // then
        assert_eq!(indices, expected);
    }
    
    #[test]
    fn white_pawn_move_one_field() {
        //given
        let board = [
            [(' ',' '),(' ',' '),(' ',' '),(' ',' '),(' ',' '),(' ',' '),(' ',' '),(' ',' ')],
            [(' ',' '),(' ',' '),(' ',' '),(' ',' '),(' ',' '),(' ',' '),(' ',' '),(' ',' ')],
            [(' ',' '),(' ',' '),(' ',' '),(' ',' '),(' ',' '),(' ',' '),(' ',' '),(' ',' ')],
            [(' ',' '),(' ',' '),(' ',' '),(' ',' '),(' ',' '),(' ',' '),(' ',' '),(' ',' ')],
            [(' ',' '),(' ',' '),(' ',' '),(' ',' '),(' ',' '),(' ',' '),(' ',' '),(' ',' ')],
            [(' ',' '),(' ',' '),(' ',' '),(' ',' '),(' ',' '),(' ',' '),(' ',' '),(' ',' ')],
            [(' ',' '),(' ',' '),(' ',' '),(' ',' '),(' ',' '),(' ',' '),(' ',' '),(' ',' ')],
            [('P','W'),(' ',' '),(' ',' '),(' ',' '),(' ',' '),(' ',' '),(' ',' '),(' ',' ')],
        ];
        let from = (b'A',b'1');
        let to = (b'A',b'2');
        let expected = [
            [(' ',' '),(' ',' '),(' ',' '),(' ',' '),(' ',' '),(' ',' '),(' ',' '),(' ',' ')],
            [(' ',' '),(' ',' '),(' ',' '),(' ',' '),(' ',' '),(' ',' '),(' ',' '),(' ',' ')],
            [(' ',' '),(' ',' '),(' ',' '),(' ',' '),(' ',' '),(' ',' '),(' ',' '),(' ',' ')],
            [(' ',' '),(' ',' '),(' ',' '),(' ',' '),(' ',' '),(' ',' '),(' ',' '),(' ',' ')],
            [(' ',' '),(' ',' '),(' ',' '),(' ',' '),(' ',' '),(' ',' '),(' ',' '),(' ',' ')],
            [(' ',' '),(' ',' '),(' ',' '),(' ',' '),(' ',' '),(' ',' '),(' ',' '),(' ',' ')],
            [('P','W'),(' ',' '),(' ',' '),(' ',' '),(' ',' '),(' ',' '),(' ',' '),(' ',' ')],
            [(' ',' '),(' ',' '),(' ',' '),(' ',' '),(' ',' '),(' ',' '),(' ',' '),(' ',' ')],
        ];

        // when
        let result = mve(from, to, board);

        // then
        assert_eq!(result, expected);
    }
}
