const DESIRED_ROW: [Int; 16] = [3, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3];
const DESIRED_COL: [Int; 16] = [3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2];

const SOLVED_STATE: u64 = 0x123456789abcdef0;
type Int = i16;

const DEFAULT_STATES: [State; 150] = [State::default(); 150];

#[derive(Clone, Copy)]
struct Position {
    row: Int,
    col: Int,
}

#[derive(Clone, Copy)]
struct State {
    zero_pos: Position,
    board: u64,
    step: char,
    path_cost: Int,
}

impl State {
    fn new(tiles: Vec<u8>) -> Self {
        let tiles = tiles.into_iter();
        let zero_index = tiles.clone().position(|e| e == 0).unwrap() as Int;
        let compact_representation =
            tiles
                .clone()
                .map(|n| u64::from(n))
                .enumerate()
                .fold(0u64, |acc, (i, n)| {
                    let compressed_size = 4;
                    let shift = (15 - i) * compressed_size;
                    acc | (n << shift)
                });

        // let distance = tiles
        // 	.enumerate()
        // 	.map(|(i, n)| (i as i8, n as i8))
        // 	.fold(0, |acc, (index, mut number)| {
        // 		number -= 1;
        // 		let position = (index / 4, index % 4);
        // 		let desired_positio1n = (number / 4, number % 4);
        // 		acc + (position.0 - desired_position.0).abs()
        // 			+ (position.1 - desired_position.1).abs()
        // 	})
        // 	.try_into()
        // 	.unwrap();

        Self {
            zero_pos: Position {
                row: zero_index / 4,
                col: zero_index % 4,
            },
            board: compact_representation,
            path_cost: 0,
            step: Default::default(),
        }
    }

    const fn default() -> Self {
        Self {
            zero_pos: Position { row: 0, col: 0 },
            board: 0,
            path_cost: 0,
            step: '0',
        }
    }
}

struct PathExplorer {
    states: [State; 150],
    index: usize,
    path_cost_limit: Int,
    // split_thread: bool,
}

impl PathExplorer {
    fn new(tiles: Vec<u8>) -> PathExplorer {
        let mut states = DEFAULT_STATES;
        states[0] = State::new(tiles);

        Self {
            states,
            index: 0,
            path_cost_limit: 0,
        }
    }

    fn move_d(&mut self) {
        let state = self.curr_state();
        let zero_location = state.zero_pos.row * 4 + state.zero_pos.col;
        let num_location = (15 - 4 - zero_location) * 4;
        let num_bits = state.board & (15u64 << num_location);
        let if_bad_move = state.zero_pos.row < DESIRED_ROW[(num_bits >> num_location) as usize];

        self.states[self.index + 1] = State {
            step		: 'd',
            path_cost	: state.path_cost + if_bad_move as Int,
            board		: state.board - num_bits + (num_bits << 16),
            zero_pos	: Position {
				row: state.zero_pos.row + 1,
				col: state.zero_pos.col,
			},
        }
    }

    fn move_u(&mut self) {
        let state = self.curr_state();
        let zero_location = state.zero_pos.row * 4 + state.zero_pos.col;
        let num_location = (15 + 4 - zero_location) * 4;
		// println!("shift = {}, zero = {}, row {}, col {}", num_location, zero_location, state.zero_pos.row, state.zero_pos.col);
        let num_bits = state.board & (15u64 << num_location);
        let if_bad_move = state.zero_pos.row > DESIRED_ROW[(num_bits >> num_location) as usize];

        self.states[self.index + 1] = State {
            step		: 'u',
            path_cost	: state.path_cost + if_bad_move as Int,
            board		: state.board - num_bits + (num_bits >> 16),
            zero_pos	: Position {
				row: state.zero_pos.row - 1,
				col: state.zero_pos.col,
			},
        }
    }

    fn move_r(&mut self) {
        let state = self.curr_state();
        let zero_location = state.zero_pos.row * 4 + state.zero_pos.col;
        let num_location = (15 - 1 - zero_location) * 4;
        let num_bits = state.board & (15u64 << num_location);
        let if_bad_move = state.zero_pos.col < DESIRED_COL[(num_bits >> num_location) as usize];

        self.states[self.index + 1] = State {
            step		: 'r',
            path_cost	: state.path_cost + if_bad_move as Int,
            board		: state.board - num_bits + (num_bits << 4),
            zero_pos	: Position {
				row: state.zero_pos.row,
				col: state.zero_pos.col + 1,
			},
        }
    }

    fn move_l(&mut self) {
        let state = self.curr_state();
        let zero_location = state.zero_pos.row * 4 + state.zero_pos.col;
        let num_location = (15 + 1 - zero_location) * 4;
        let num_bits = state.board & (15u64 << num_location);
        let if_bad_move = state.zero_pos.col > DESIRED_COL[(num_bits >> num_location) as usize];

		self.states[self.index + 1] = State {
            step		: 'l',
            path_cost	: state.path_cost + if_bad_move as Int,
            board		: state.board - num_bits + (num_bits >> 4),
            zero_pos	: Position {
				row: state.zero_pos.row,
				col: state.zero_pos.col - 1,
			},
        }
    }

    // fn print_state(state: u64) {
    // 	let uniform_size = 16.to_string().len();
    // 	for i in 0..16 {
    // 		let p = (15_u64 - i) * 4;
    // 		let n = (state & (15 << p)) >> p;
    // 		let n = if n == 0 { '_'.to_string() } else { n.to_string() };
    // 		let s = format!("{: >1$}", n, uniform_size);

    // 		if (i + 1) % 4 != 0 {
    // 			print!("{s} ");
    // 		} else {
    // 			println!("{s}");
    // 		};
    // 	}
    // 	println!("");
    // }

    fn curr_state(&self) -> &State {
        &self.states[self.index]
    }

    fn check(&mut self) -> bool {
        self.index += 1;
        // println!("move {}", self.curr_state().step);

        let state = self.curr_state();
        // println!(
        // 	"check state = {:?}, path_cost = {}",
        // 	state.board, state.path_cost
        // );

        if state.board == SOLVED_STATE
        || state.path_cost <= self.path_cost_limit
		&& self.explore() {
            return true;
        }

        self.index -= 1;
        false
    }

    fn explore(&mut self) -> bool {
        let &State {
            zero_pos,
            step: last_step,
            ..
        } = self.curr_state();

        if zero_pos.row != 3 && last_step != 'u' {
            self.move_d();
            if self.check() {
                return true;
            }
        }
        if zero_pos.row != 0 && last_step != 'd' {
            self.move_u();
            if self.check() {
                return true;
            }
        }
        if zero_pos.col != 3 && last_step != 'l' {
            self.move_r();
            if self.check() {
                return true;
            }
        }
        if zero_pos.col != 0 && last_step != 'r' {
            self.move_l();
            if self.check() {
                return true;
            }
        }
        false
    }

    fn solve(&mut self) -> String {
        while !self.explore() {
            self.index = 0;
            self.path_cost_limit += 2;
            println!("limit = {}", self.path_cost_limit);
        }
        self.states
            .iter()
            .skip(1)
            .take(self.index)
            .map(|state| state.step)
            .collect()
    }
}

fn inversions(tiles: &Vec<u8>) -> Int {
    let mut inversions = 0;

    for (i, number) in tiles.iter().copied().enumerate() {
        if number == 0 {
            continue;
        }
        let next_numbers = &tiles[i + 1..];
        for &next_number in next_numbers {
            if next_number > number {
                inversions += 1;
            }
        }
    }

    inversions
}

pub fn validate(tiles: &Vec<u8>) -> Result<(), String> {
    if tiles.len() != 16 {
        return Err("Incomplete input".into());
    }

    let mut sorted = tiles.clone();
    sorted.sort();
    if (0..16).ne(sorted.into_iter()) {
        return Err("Invalid or duplicate numbers".into());
    }

    let is_free_cell_row_odd = tiles.iter().position(|n| *n == 0).unwrap() as Int / 4 % 2;
    let is_inversions_odd = inversions(tiles) % 2;
    if is_inversions_odd != is_free_cell_row_odd {
        return Err("Invalid Permutation".into());
    }
    Ok(())
}

pub fn solver(tiles: Vec<u8>) -> Result<String, String> {
    validate(&tiles)?;
    let mut solver = PathExplorer::new(tiles);
    Ok(solver.solve())
}