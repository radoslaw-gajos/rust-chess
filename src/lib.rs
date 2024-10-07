type Piece = (char, char);
type Board = [[Piece; 8]; 8];
type Coord = (u8, u8);
type Indices = (usize, usize);

fn mve(from: Coord, to: Coord, board: Board) -> Board {
    // getting the piece from board
    let (column, row) = indices(from);
    let piece = board[row][column];
    let mut board = board;
    board[row][column] = (' ',' ');

    // putting the piece to target
    let (column, row) = indices(to);
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
