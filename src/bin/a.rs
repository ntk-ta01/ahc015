use proconio::{input, source::line::LineSource};
use rand::prelude::*;
use std::io::{self, BufReader, Stdin};

const TIMELIMIT: f64 = 1.95;

const DIJ: [(usize, usize); 4] = [(1, 0), (0, 1), (!0, 0), (0, !0)];
const DIR: [char; 4] = ['B', 'R', 'F', 'L']; // 下、右、上、左

const N: usize = 10;
const M: usize = 3;

fn main() {
    let mut source = LineSource::new(BufReader::new(io::stdin()));
    let input = parse_input(&mut source);
    let mut rng = rand_chacha::ChaCha20Rng::seed_from_u64(0);
    let mut state = State::new();
    for turn in 0..N * N - 1 {
        input! {
            from &mut source,
            pos: usize,
        }
        state.place_candy(&input, pos);
        let dir = monte_carlo(
            &input,
            &state,
            turn,
            TIMELIMIT / (N * N - 1) as f64,
            &mut rng,
        );
        println!("{}", dir);
        state.apply_move(dir);
    }
    // input! {
    //     from &mut source,
    //     pos: usize,
    // }
    // state.place_candy(&input, pos);
    eprintln!("{}", compute_score(&input, &state));
}

// モンテカルロ法　時間いっぱいランダムプレイアウトして次の手を決める
fn monte_carlo<T: Rng>(
    input: &Input,
    state: &State,
    turn: usize,
    duration: f64,
    rng: &mut T,
) -> char {
    // てりーさんのやつを読む https://atcoder.jp/contests/ahc015/submissions/36099779
    // 参考資料 https://qiita.com/thun-c/items/058743a25c37c87b8aa4#%E5%8E%9F%E5%A7%8B%E3%83%A2%E3%83%B3%E3%83%86%E3%82%AB%E3%83%AB%E3%83%AD%E6%B3%95
    let timer = Timer::new();

    let mut scores = [0; 4];
    let mut counts = [0; 4];

    for init_dir in (0..4).cycle() {
        // ルールベースに従って、時間いっぱいランダムプレイアウト
        if duration < timer.get_time() {
            break;
        }

        let mut state = state.clone();
        state.apply_move(DIR[init_dir]);
        for turn in (turn + 1)..N * N {
            let pos = rng.gen_range(1, N * N + 1 - turn);
            state.place_candy(input, pos);

            if turn + 1 == N * N {
                break;
            }

            let dir = rule_based(input, &state, state.t);
            state.apply_move(dir);
        }
        let score = compute_score(input, &state);
        scores[init_dir] += score;
        counts[init_dir] += 1;
    }

    let mut best_score = 0.0;
    let mut best_dir = !0;

    for dir in 0..4 {
        let score = scores[dir] as f64 / counts[dir] as f64;
        if best_score < score {
            best_score = score;
            best_dir = dir;
        }
    }

    DIR[best_dir]
}

fn rule_based(input: &Input, state: &State, turn: usize) -> char {
    // 次に来るやつを見て決める（一番最初に来た奴は見ない）
    if input.fs[turn] == 1 {
        'B'
    } else if state.t > 1 && state.last_dir == 'B' {
        'F'
    } else if input.fs[state.t] == 2 {
        'R'
    } else {
        'L'
    }
}

#[derive(Clone, Debug)]
struct Input {
    fs: Vec<usize>,
}

fn parse_input(f: &mut LineSource<BufReader<Stdin>>) -> Input {
    input! {
        from f,
        fs: [usize; N * N],
    }
    Input { fs }
}

#[derive(Clone, Debug)]
struct State {
    board: Vec<Vec<usize>>,
    t: usize,       // ターン数
    last_dir: char, // 最後に置かれた方向
}

impl State {
    fn new() -> Self {
        let board = vec![vec![0; N]; N]; // 0だったら空きマス
        Self {
            board,
            t: 0,
            last_dir: '.',
        }
    }
    fn place_candy(&mut self, input: &Input, pos: usize) {
        // キャンディを置く
        let mut p = 0;
        'place: for i in 0..N {
            for j in 0..N {
                if self.board[i][j] == 0 {
                    p += 1;
                    if p == pos {
                        self.board[i][j] = input.fs[self.t];
                        // self.last = (i, j);
                        break 'place;
                    }
                }
            }
        }
        self.t += 1;
    }
    fn apply_move(&mut self, dir: char) {
        // 傾ける
        match dir {
            'L' => {
                for i in 0..N {
                    let mut k = 0;
                    for j in 0..N {
                        if self.board[i][j] != 0 {
                            self.board[i][k] = self.board[i][j];
                            if k != j {
                                self.board[i][j] = 0;
                            }
                            k += 1;
                        }
                    }
                }
            }
            'R' => {
                for i in 0..N {
                    let mut k = N - 1;
                    for j in (0..N).rev() {
                        if self.board[i][j] != 0 {
                            self.board[i][k] = self.board[i][j];
                            if k != j {
                                self.board[i][j] = 0;
                            }
                            k -= 1;
                        }
                    }
                }
            }
            'F' => {
                for j in 0..N {
                    let mut k = 0;
                    for i in 0..N {
                        if self.board[i][j] != 0 {
                            self.board[k][j] = self.board[i][j];
                            if k != i {
                                self.board[i][j] = 0;
                            }
                            k += 1;
                        }
                    }
                }
            }
            'B' => {
                for j in 0..N {
                    let mut k = N - 1;
                    for i in (0..N).rev() {
                        if self.board[i][j] != 0 {
                            self.board[k][j] = self.board[i][j];
                            if k != i {
                                self.board[i][j] = 0;
                            }
                            k -= 1;
                        }
                    }
                }
            }
            _ => {
                panic!("Illegal output: {}", dir)
            }
        }
        self.last_dir = dir;
    }
}

fn compute_score(input: &Input, state: &State) -> i64 {
    let mut visited = vec![vec![false; N]; N];
    let mut num = 0;
    for i in 0..N {
        for j in 0..N {
            if !visited[i][j] && state.board[i][j] != 0 {
                visited[i][j] = true;
                let c = state.board[i][j];
                let mut size = 1;
                let mut stack = vec![(i, j)];
                while let Some((i, j)) = stack.pop() {
                    for &(di, dj) in &DIJ {
                        let i2 = i + di;
                        let j2 = j + dj;
                        if i2 < N && j2 < N && !visited[i2][j2] && state.board[i2][j2] == c {
                            visited[i2][j2] = true;
                            stack.push((i2, j2));
                            size += 1;
                        }
                    }
                }
                num += size * size;
            }
        }
    }
    let mut d = vec![0; M + 1];
    for &f in &input.fs {
        d[f] += 1;
    }
    (1e6 * num as f64 / d[1..].iter().map(|d| d * d).sum::<i32>() as f64).round() as i64
}

fn get_time() -> f64 {
    let t = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap();
    t.as_secs() as f64 + t.subsec_nanos() as f64 * 1e-9
}

struct Timer {
    start_time: f64,
}

impl Timer {
    fn new() -> Timer {
        Timer {
            start_time: get_time(),
        }
    }

    fn get_time(&self) -> f64 {
        get_time() - self.start_time
    }
}
