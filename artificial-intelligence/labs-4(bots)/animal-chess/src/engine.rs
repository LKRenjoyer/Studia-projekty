use rand::{self, Rng};

pub const WHITE: bool = false;
pub const BLACK: bool = true;

pub type Coords = (i8, i8);
type PlayerCoords = [Coords; 8];
pub type State = (PlayerCoords, PlayerCoords, bool);
pub type ZorbTable = [[[u64;8];7];9];
// white pieces, black pieces, to move
// CZARNY ZACZYNA
pub const RAT: usize = 0;
// pub const CAT: usize = 1;
// pub const DOG: usize = 2;
// pub const WOLF: usize = 3;
// pub const JAG: usize = 4;
pub const TIGER: usize = 5;
pub const LION: usize = 6;
pub const ELEP: usize = 7;

pub const TRAPS: [Coords; 6] = [(2, 0), (4, 0), (3, 1), (2, 8), (4, 8), (3, 7)];
pub const WHITE_BASE: Coords = (3, 0);
pub const BLACK_BASE: Coords = (3, 8);
pub const WATER: [Coords; 12] = [
    (1, 3),
    (2, 3),
    (4, 3),
    (5, 3),
    (1, 4),
    (2, 4),
    (4, 4),
    (5, 4),
    (1, 5),
    (2, 5),
    (4, 5),
    (5, 5),
];
const DIRS: [(i8, i8); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];
pub struct Jungle {
}
impl Jungle {
    pub fn init_state(who_starts : bool) -> State {
        {
            (
                [
                    (0, 2),
                    (5, 1),
                    (1, 1),
                    (4, 2),
                    (2, 2),
                    (6, 0),
                    (0, 0),
                    (6, 2),
                ],
                [
                    (6, 6),
                    (1, 7),
                    (5, 7),
                    (2, 6),
                    (4, 6),
                    (0, 8),
                    (6, 8),
                    (0, 6),
                ],
                who_starts,
            )
        }
    }
    pub fn zorb_tab() -> (ZorbTable,ZorbTable) {
        let mut white_table: ZorbTable = [[[0;8];7];9];
        let mut black_table: ZorbTable = [[[0;8];7];9];
        let mut rng = rand::rng();
        for i in 0..9 {
            for j in 0..7 {
                for k in 0..8 {
                    black_table[i][j][k] = rng.random();
                    white_table[i][j][k] = rng.random();
                }
            }
        }
        (white_table,black_table)
    }
    pub fn init_hash((whites, blacks) : (ZorbTable,ZorbTable)) -> u64 {
        let mut res = 0;
        res ^= whites[2][0][0] ^ whites[1][5][1] ^ whites[1][1][2] ^ whites[2][4][3];
        res ^= whites[2][2][4] ^ whites[0][6][5] ^ whites[0][0][6] ^ whites[2][6][7];
        res ^= blacks[6][6][0] ^ blacks[7][1][1] ^ blacks[7][5][2] ^ blacks[6][2][3];
        res ^= blacks[6][4][4] ^ blacks[8][0][5] ^ blacks[8][6][6] ^ blacks[6][0][7];
        res
    }
}
impl Jungle {
    pub fn terminal(state: &State) -> bool {
        (state.2 == WHITE && state.1.contains(&WHITE_BASE))
            || (state.2 == BLACK && state.0.contains(&BLACK_BASE))
    }
    pub fn utility(state: &State) -> i8 {
        if state.1.contains(&WHITE_BASE) {
            1
        } else if state.0.contains(&BLACK_BASE) {
            -1
        } else {
            0
        }
    }
    pub fn result(state: &State, piece: usize, delta: (i8, i8)) -> State {
        let mut new_state: State = state.clone();
        new_state.2 = !(state.2);
        if delta == (0,0) {
            new_state
        } else {
        if state.2 == WHITE {
            let new_coords = (state.0[piece].0 + delta.0, state.0[piece].1 + delta.1);
            new_state.0[piece] = new_coords;
            for coords in &mut new_state.1 {
                if *coords == new_coords {
                    *coords = (-1, -1);
                    break;
                }
            }
        } else {
            let new_coords = (state.1[piece].0 + delta.0, state.1[piece].1 + delta.1);
            new_state.1[piece] = new_coords;
            for coords in &mut new_state.0 {
                if *coords == new_coords {
                    *coords = (-1, -1);
                    break;
                }
            }
        }
        new_state
    }
    }
    pub fn hash_delta(state: &State, piece:usize, delta:(i8,i8), zorbs : (ZorbTable,ZorbTable)) -> u64 {
        let (my_tab,opp_tab) = if state.2==WHITE {(zorbs.0,zorbs.1)} else {(zorbs.1,zorbs.0)};
        let (my_piece,opp_piece) = if state.2 == WHITE {(state.0,state.1)} else {(state.1,state.0)};
        let (o_x,o_y) = (my_piece[piece].0 as usize,my_piece[piece].1 as usize);
        let (n_x,n_y) = (o_x +delta.0 as usize,o_y + delta.1 as usize);
        let old_hsh = my_tab[o_y][o_x][piece]^my_tab[n_y][n_x][piece];
        let new_hsh: u64 = if opp_piece.contains(&(n_x as i8,n_y as i8)) {
            let opp = opp_piece.iter()
                                        .position(|coords| *coords==(n_x as i8,n_y as i8))
                                        .unwrap();
            opp_tab[n_y][n_x][opp]
        } else {0};
        old_hsh ^ new_hsh 

    }
}
impl Jungle {
    fn in_bounds(c: Coords) -> bool {
        c.0 >= 0 && c.0 < 7 && c.1 >= 0 && c.1 < 9
    }
    fn generate_all_moves(pieces : &PlayerCoords) -> Vec<(usize,(i8,i8))> {
        let mut res = vec![];
        for piece in 0..8 {
            let old_coords : Coords = pieces[piece];
            if old_coords == (-1,-1) {continue;}
            for delta in DIRS {
                if WATER.contains(&(old_coords.0+delta.0,old_coords.1+delta.1)) {
                    match piece {
                        RAT => res.push((piece,delta)),
                        LION | TIGER => {
                            if delta.1 > 0 {res.push((piece,(0,delta.1*4)))}
                            else {res.push((piece,(delta.0*3,0)))}
                        }
                        _ => ()
                    } } else {res.push((piece,delta))}
            }
        }
        res
    }
    fn legal_std(state: &State, piece: usize, delta: (i8, i8)) -> bool {
        let (my_pieces, opp_pieces) = if state.2 == WHITE {
            (state.0, state.1)
        } else {
            (state.1, state.0)
        };
        let (c_x, c_y) = my_pieces[piece];

        let new_coords: Coords = (c_x + delta.0, c_y + delta.1);
        if !Jungle::in_bounds(new_coords) || my_pieces.contains(&new_coords) {  // wychodzenie za plansze
            false                                                               // albo w swoja bierke
        } else if WATER.contains(&new_coords) && piece != RAT {                 // wchodzenie do wody
            false
        } else if piece == RAT                                                  // bicie slonia szczurem
            && opp_pieces[ELEP] == new_coords   
            && !TRAPS.contains(&(c_x, c_y))
            && !WATER.contains(&(c_x, c_y))
        {
            true
        } else if (WHITE_BASE == new_coords && state.2 == WHITE)                // do swojej bazy
            || (BLACK_BASE == new_coords && state.2 == BLACK)
        {
            false
        } else if !opp_pieces.contains(&new_coords) {                           // wejscie na wolne pole
            true
        } else {                                                                 // bitka
            let my_strength:i8 = if TRAPS.contains(&(c_x,c_y)) || WATER.contains(&(c_x,c_y)) {-1} else {piece as i8};
            let opp_piece = opp_pieces
            .iter().zip(0..8)
            .find_map(|(coords,p)| if *coords==new_coords {Some(p)} else {None}).unwrap();
            let opp_strength:i8 = if TRAPS.contains(&new_coords) {-1} else {opp_piece};
            match (my_strength,opp_strength) {
                (7,0) => false,
                _ => my_strength>=opp_strength
            }
        }
    }
    fn legal_jump(state: &State, piece: usize, delta: (i8,i8)) -> bool {
        let (my_pieces,opp_pieces) = if state.2 == WHITE {(state.0,state.1)} else {(state.1,state.0)};
        let (c_x,c_y):Coords = my_pieces[piece];
        let new_coords:Coords = (c_x+delta.0,c_y+delta.1);
        let rats = [state.0[RAT],state.1[RAT]];
        let rats_on_way = match delta {
            (-3,0) => rats.contains(&(c_x-1,c_y)) || rats.contains(&(c_x-2,c_y)),
            (3,0) => rats.contains(&(c_x+1,c_y)) || rats.contains(&(c_x+2,c_y)),
            (0,4) => rats.contains(&(c_x,c_y+1)) || rats.contains(&(c_x,c_y+2)) || rats.contains(&(c_x,c_y+3)),
            (0,-4) => rats.contains(&(c_x,c_y-1)) || rats.contains(&(c_x,c_y-2)) || rats.contains(&(c_x,c_y-3)),
            _ => unimplemented!("WRONG DELTA FOR JUMP : {:?}",delta)
        };
        if rats_on_way {false}
        else {   
            for opp_coords in &opp_pieces[piece+1..8] {
                if *opp_coords == new_coords {return false;}
            }
        true}
    }
    pub fn actions(state : &State) -> Vec<(usize,(i8,i8))> {
        let pieces = if state.2 == WHITE {state.0} else {state.1};
        Jungle::generate_all_moves(&pieces)
                .into_iter()
                .filter(|(piece, delta)| {
                if delta.0.abs() == 3 || delta.1.abs() == 3 {
                    Jungle::legal_jump(&state, *piece, *delta)
                } else {
                    Jungle::legal_std(&state, *piece, *delta)
                }
            } ).collect()
            }
}



impl Jungle {
    pub fn to_str(state: &State) -> String {
        let mut board = [['.'; 7]; 9];
        for (x, y) in TRAPS {
            board[y as usize][x as usize] = '#';
        }
        for (x, y) in [WHITE_BASE, BLACK_BASE] {
            board[y as usize][x as usize] = '*';
        }
        for (x, y) in WATER {
            board[y as usize][x as usize] = '~';
        }
        let white_pieces = state.0;
        let black_pieces = state.1;
        for ((x, y), c) in white_pieces.iter().zip("RCDWJTLE".chars()) {
            if *x != -1 {
                board[*y as usize][*x as usize] = c;
            }
        }
        for ((x, y), c) in black_pieces.iter().zip("rcdwjtle".chars()) {
            if *x != -1 {
                board[*y as usize][*x as usize] = c;
            }
        }
        board.map(|row| row.iter().collect::<String>()).join("\n")
    }
}
/* bialy gora
L.#*#.T 0
.D.#.C. 1
R.J.W.E 2
.~~.~~. 3
.~~.~~. 4
.~~.~~. 5
e.w.j.r 6
.c.#.d. 7
t.#*#.l 8
*/

mod engine_tests {
    use crate::engine::*;

    #[test]
    fn result_test_1() {
        let mut obj: State = Jungle::init_state(WHITE);
        obj = Jungle::result(&obj, RAT, (0, 4)); // 4 w dol
        println!("{}", Jungle::to_str(&obj));
        println!("{:?}", obj);
    }
    #[test]
    fn terminal_test_1() {
        let mut obj: State = Jungle::init_state(WHITE);
        obj = Jungle::result(&obj, RAT, (3, 6));
        let white_win = Jungle::terminal(&obj);
        obj = Jungle::init_state(BLACK);
        obj = Jungle::result(&obj, ELEP, (3, -6));
        let black_win = Jungle::terminal(&obj);
        assert!(black_win && white_win)
    }
    #[test]
    fn actions_test_1() {
        let jg = Jungle::init_state(WHITE);
        println!("{}\n", Jungle::to_str(&jg));
        println!("{:?}", Jungle::actions(&jg));

    }
}
