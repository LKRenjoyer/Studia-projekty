pub const WHITE: bool = false;
pub const BLACK: bool = true;
pub type State = (u128, bool);
pub type Move = (i8, i8);
const DIRS: [(i8, i8); 8] = [
    (-1, -1),
    (-1, 0),
    (0, -1),
    (0, 1),
    (1, 0),
    (1, 1),
    (1, -1),
    (-1, 1),
];
pub struct Reversi {}
impl Reversi {
    pub fn init_state() -> State {
        (
            (1u128 << Reversi::index((3, 3), WHITE))
                + (1u128 << Reversi::index((4, 4), WHITE))
                + (1u128 << Reversi::index((3, 4), BLACK))
                + (1u128 << Reversi::index((4, 3), BLACK)),
            BLACK,
        )
    }
}
impl Reversi {
    fn index(mv: Move, color: bool) -> i8 {
        let (x, y) = mv;
        let offset = if color == WHITE { 0 } else { 64 };
        if x >= 0 && x < 8 && y >= 0 && y < 8 {
            (offset + 8 * y + x) as i8
        } else {
            -1
        }
    }
    fn elem(board: &u128, idx: i8) -> bool {
        (board & (1u128 << idx)) != 0
    }
    fn does_flip(board: &u128, dx: i8, dy: i8, mv: Move, color: bool) -> i8 {
        let mut _x = mv.0 + dx;
        let mut _y = mv.1 + dy;
        let mut seg = 0;
        loop {
            let my_idx = Self::index((_x, _y), color);
            if my_idx == -1 {
                break;
            }
            let my_elem = Self::elem(&board, my_idx);
            let opp_elem = Self::elem(&board, Self::index((_x, _y), !color));
            match (my_elem, opp_elem) {
                (false, false) => return 0,
                (true, false) => return seg,
                (false, true) => seg += 1,
                (true, true) => panic!("Got true true on {}, {}", _x, _y),
            }
            _x += dx;
            _y += dy;
        }
        0
    }
    fn legal(board: &u128, mv: Move, color: bool) -> bool {
        if Self::elem(&board, Self::index(mv, color)) || Self::elem(&board, Self::index(mv, !color))
        {
            false
        } else {
            DIRS.iter()
                .any(|(dx, dy)| Self::does_flip(&board, *dx, *dy, mv, color) > 0)
        }
    }
}
impl Reversi {
    pub fn actions(state: &State) -> Vec<Move> {
        let (board, color) = state;
        let mut res = vec![];
        for i in 0..8 {
            for j in 0..8 {
                if Self::legal(&board, (i, j), *color) {
                    res.push((i, j));
                }
            }
        }
        if res.is_empty() {
            vec![(-1, -1)]
        } else {
            res
        }
    }
    pub fn result(state: &State, mv: Move) -> State {
        let (board, color) = state;
        if mv == (-1, -1) {
            (*board, !color)
        } else {
            let mut new_board = state.0.clone();
            new_board |= 1 << Self::index(mv, *color);
            DIRS.iter().for_each(|(dx, dy)| {
                for i in 0..Self::does_flip(&board, *dx, *dy, mv, *color) {
                    let (_x, _y) = (mv.0 + dx * (1 + i), mv.1 + dy * (i + 1));
                    let idx = Self::index((_x, _y), WHITE);
                    new_board ^= (1 << idx) | (1 << (idx + 64));
                }
            });

            (new_board, !color)
        }
    }
    pub fn terminal(state: &State) -> bool {
        Self::actions(&(state.0, BLACK))[0] == (-1, -1)
            && Self::actions(&(state.0, WHITE))[0] == (-1, -1)
    }
    pub fn utility(state: &State) -> i32 {
        let (board, _) = state;
        -(((board >> 64) as u64).count_ones() as i32 - (*board as u64).count_ones() as i32)
    }
}
