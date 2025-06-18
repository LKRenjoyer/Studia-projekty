use ::rand::Rng;

pub const WHITE: usize = 1;
pub const BLACK: usize = 2;
const FLIP: usize = 3;
const SIZE: usize = 8;
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
pub const FIELD_SCORE: [[i32; 8]; 8] = [
    [170, -25, 20, 5, 5, 20, -25, 170],
    [-25, -50, -5, -5, -5, -5, -50, -25],
    [20, -5, 15, 3, 3, 15, -5, 20],
    [5, -5, 3, 3, 3, 3, -5, 5],
    [5, -5, 3, 3, 3, 3, -5, 5],
    [20, -5, 15, 3, 3, 15, -5, 20],
    [-25, -50, -5, -5, -5, -5, -50, -25],
    [170, -25, 20, 5, 5, 20, -25, 170],
];
pub type State = (u128, u64, i8);
pub type ZorbTable = [[[u64; 4]; SIZE]; SIZE];

#[derive(Debug)]
pub struct Reversi {
    pub current_hash: u64,
    pub current_board: u128,
    zorbist_table: ZorbTable,
    pub current_player: usize,
    move_list: Vec<State>,
    pub free_fields: i8,
}

impl Reversi {
    pub fn new(zorb_tab: ZorbTable) -> Self {
        Reversi {
            zorbist_table: zorb_tab,
            current_board: Self::init_board(),
            current_player: BLACK,
            current_hash: Self::init_hash(zorb_tab),
            move_list: vec![(Self::init_board(), Self::init_hash(zorb_tab), 60)],
            free_fields: 60,
        }
    }

    pub fn set_zorbist_table() -> ZorbTable {
        let mut rng = rand::rng();
        let mut zorb_tab = [[[0; 4]; SIZE]; SIZE];
        for i in 0..SIZE {
            for j in 0..SIZE {
                let wh = rng.random::<u64>();
                let bl = rng.random::<u64>();
                zorb_tab[i][j] = [0, wh, bl, wh ^ bl];
            }
        }
        zorb_tab
    }
    fn init_board() -> u128 {
        let mut b = 0;
        b |= 1u128 << Reversi::index(3, 3, WHITE);
        b |= 1u128 << Reversi::index(4, 4, WHITE);
        b |= 1u128 << Reversi::index(3, 4, BLACK);
        b |= 1u128 << Reversi::index(4, 3, BLACK);
        b
    }

    fn init_hash(zorb_tab: ZorbTable) -> u64 {
        let mut hsh: u64 = 0;
        hsh ^= zorb_tab[3][3][WHITE];
        hsh ^= zorb_tab[4][4][WHITE];
        hsh ^= zorb_tab[3][4][BLACK];
        hsh ^= zorb_tab[4][3][BLACK];
        hsh
    }
}
impl Reversi {
    fn index(x: i8, y: i8, color: usize) -> usize {
        if x >= 0 && x < SIZE as i8 && y >= 0 && y < SIZE as i8 {
            let offset = if color == WHITE { 0 } else { 64 };
            offset + y as usize * SIZE + x as usize
        } else {
            404
        }
    }
    fn elem(&self, idx: usize) -> bool {
        (self.current_board & (1u128 << idx)) != 0
    }
    fn does_flip(&self, dx: i8, dy: i8, x: i8, y: i8, color: usize) -> i8 {
        let mut _x = x + dx;
        let mut _y = y + dy;
        let mut seg = 0;
        loop {
            let my_idx = Reversi::index(_x, _y, color);
            if my_idx > 200 {
                break;
            }
            let my_elem = self.elem(my_idx);
            let opp_elem = self.elem(Reversi::index(_x, _y, color ^ 3));
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
    fn legal(&self, x: i8, y: i8, color: usize) -> bool {
        if self.elem(Reversi::index(x, y, color)) || self.elem(Reversi::index(x, y, color ^ 3)) {
            false
        } else {
            DIRS.iter()
                .any(|(dx, dy)| 0 < self.does_flip(*dx, *dy, x, y, color))
        }
    }
}
impl Reversi {
    pub fn actions(&self, color: usize) -> Vec<(i8, i8)> {
        let mut res = vec![];
        for i in 0..(SIZE as i8) {
            for j in 0..(SIZE as i8) {
                if self.legal(i, j, color) {
                    res.push((i, j))
                }
            }
        }
        if res.len() > 0 {
            res
        } else {
            vec![(-1, -1)]
        }
    }
    pub fn result(&mut self, x: i8, y: i8, color: usize) {
        if x == -1 {
            self.move_list
                .push((self.current_board, self.current_hash, self.free_fields));
            self.current_player ^= 3;
            return;
        }
        let mut board = self.current_board;
        let mut hsh = self.current_hash;
        hsh ^= self.zorbist_table[x as usize][y as usize][color];
        board |= 1 << Reversi::index(x, y, color);

        for (dx, dy) in DIRS {
            for i in 0..self.does_flip(dx, dy, x, y, color) {
                let (_x, _y) = (x + dx * (1 + i), y + dy * (i + 1));
                let idx = Reversi::index(_x, _y, WHITE);
                board ^= (1 << idx) | (1 << (idx + 64));
                hsh ^= self.zorbist_table[_x as usize][_y as usize][FLIP];
            }
        }
        self.current_player ^= 3;
        self.free_fields -= 1;
        self.move_list.push((board, hsh, self.free_fields));
        self.current_board = board;
        self.current_hash = hsh;
    }
    pub fn undo(&mut self) {
        self.move_list.pop();
        (self.current_board, self.current_hash, self.free_fields) = *self.move_list.last().unwrap();
        self.current_player ^= 3;
    }
    #[allow(dead_code)]
    pub fn to_str(&self) -> String {
        let mut brd = [['.'; SIZE]; SIZE];
        for x in 0..SIZE as i8 {
            for y in 0..SIZE as i8 {
                let is_white = self.elem(Reversi::index(x, y, WHITE));
                let is_black = self.elem(Reversi::index(x, y, BLACK));
                if is_white {
                    brd[y as usize][x as usize] = '#';
                }
                if is_black {
                    brd[y as usize][x as usize] = 'o';
                }
                if is_white && is_black {
                    brd[y as usize][x as usize] = '?';
                }
            }
        }
        let res: String = brd.join(&'\n').iter().collect();
        format!("{}\n{}", res, self.current_hash)
    }

    pub fn terminal(&self) -> bool {
        self.actions(BLACK)[0] == (-1, -1) && self.actions(WHITE)[0] == (-1, -1)
    }
    pub fn utility(&self) -> i32 {
        ((self.current_board >> 64) as u64).count_ones() as i32
            - (self.current_board as u64).count_ones() as i32
    }
    pub fn heuristic_eval(board: u128) -> i32 {
        let black_bb = (board >> 64) as u64;
        let white_bb = board as u64;
        let mut score = 0i32;
        for row in 0..8 {
            for col in 0..8 {
                let idx = (row * 8 + col) as u32;
                let mask = 1u64 << idx;
                if (black_bb & mask) != 0 {
                    score += FIELD_SCORE[row as usize][col as usize];
                } else if (white_bb & mask) != 0 {
                    score -= FIELD_SCORE[row as usize][col as usize];
                }
            }
        }
        score
    }
}

type Move = (i8, i8);
pub struct LightReversi {}
impl LightReversi {
    pub fn init_board() -> u128 {
        (1u128 << LightReversi::index((3, 3), WHITE))
            + (1u128 << LightReversi::index((4, 4), WHITE))
            + (1u128 << LightReversi::index((3, 4), BLACK))
            + (1u128 << LightReversi::index((4, 3), BLACK))
    }
}
impl LightReversi {
    fn index(mv: Move, color: usize) -> i8 {
        let (x, y) = mv;
        let offset = if color == 1 { 0 } else { 64 };
        if x >= 0 && x < 8 && y >= 0 && y < 8 {
            offset + 8 * y + x
        } else {
            -1
        }
    }
    fn elem(board: &u128, idx: i8) -> bool {
        (board & (1u128 << idx)) != 0
    }
    fn does_flip(board: &u128, dx: i8, dy: i8, mv: Move, color: usize) -> i8 {
        let mut _x = mv.0 + dx;
        let mut _y = mv.1 + dy;
        let mut seg = 0;
        loop {
            let my_idx = Self::index((_x, _y), color);
            if my_idx == -1 {
                break;
            }
            let my_elem = Self::elem(&board, my_idx);
            let opp_elem = Self::elem(&board, Self::index((_x, _y), color ^ 3));
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
    fn legal(board: &u128, mv: Move, color: usize) -> bool {
        if Self::elem(&board, Self::index(mv, color))
            || Self::elem(&board, Self::index(mv, color ^ 3))
        {
            false
        } else {
            DIRS.iter()
                .any(|(dx, dy)| Self::does_flip(&board, *dx, *dy, mv, color) > 0)
        }
    }
}
impl LightReversi {
    pub fn actions(board: &u128, color: usize) -> Vec<Move> {
        let mut res = vec![];
        for i in 0..8 {
            for j in 0..8 {
                if Self::legal(&board, (i, j), color) {
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
    pub fn result(board: &u128, mv: Move, color: usize) -> u128 {
        if mv == (-1, -1) {
            *board
        } else {
            let mut new_board = board.clone();
            new_board |= 1 << Self::index(mv, color);
            DIRS.iter().for_each(|(dx, dy)| {
                for i in 0..Self::does_flip(&board, *dx, *dy, mv, color) {
                    let (_x, _y) = (mv.0 + dx * (1 + i), mv.1 + dy * (i + 1));
                    let idx = Self::index((_x, _y), 1);
                    new_board ^= (1 << idx) | (1 << (idx + 64));
                }
            });

            new_board
        }
    }
    pub fn terminal(board: &u128) -> bool {
        Self::actions(board, BLACK)[0] == (-1, -1) && Self::actions(board, WHITE)[0] == (-1, -1)
    }
    pub fn utility(board: &u128) -> i32 {
        ((board >> 64) as u64).count_ones() as i32 - (*board as u64).count_ones() as i32
    }
}

#[test]
fn test1() {
    let obj = LightReversi::init_board();
    println!("{:?}", LightReversi::actions(&obj, BLACK));
}
