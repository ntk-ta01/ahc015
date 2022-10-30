use proconio::{input, source::line::LineSource};
use std::io::{self, BufReader, Stdin};

// const TIMELIMIT: f64 = 1.95;

const DIJ: [(usize, usize); 4] = [(1, 0), (0, 1), (!0, 0), (0, !0)];
const DIR: [char; 4] = ['B', 'R', 'F', 'L']; // 下、右、上、左

const N: usize = 10;
const M: usize = 3;

type Output = Vec<char>;

fn main() {
    let mut source = LineSource::new(BufReader::new(io::stdin()));
    let input = parse_input(&mut source);
    let mut output: Output = vec![];
    // let mut rng = rand_chacha::ChaCha20Rng::seed_from_u64(159640460);
    let mut state = State::new();
    for _ in 0..N * N - 1 {
        input! {
            from &mut source,
            pos: usize,
        }
        state.place_candy(&input, pos);
        // 次に来るやつを見て決める（一番最初に来た奴は見ない）
        let dir = if input.fs[state.t] == 1 {
            println!("B");
            'B'
        } else if state.t > 1 && output[state.t - 2] == 'B' {
            println!("F");
            'F'
        } else if input.fs[state.t] == 2 {
            println!("R");
            'R'
        } else {
            println!("L");
            'L'
        };
        state.apply_move(dir);
        output.push(dir);
    }
    // eprintln!("{}", output.len());
    // eprintln!("{}", compute_score(&input, &state));
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
    // ps: Vec<usize>, // t回目でキャンディがどこに来るか
    board: Vec<Vec<usize>>,
    t: usize,             // ターン数
    last: (usize, usize), // 最後に置かれた場所
}

impl State {
    fn new() -> Self {
        let board = vec![vec![0; N]; N]; // 0だったら空きマス
        let last = (!0, !0);
        // let last = ((input.ps[0] - 1) / N, (input.ps[0] - 1) % N);
        // board[last.0][last.1] = input.fs[0];
        Self {
            // ps: input.ps.clone(),
            board,
            t: 0,
            last,
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
                        self.last = (i, j);
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
        // self.t += 1;
        // let mut p = 0;
        // for i in 0..N {
        //     for j in 0..N {
        //         if self.board[i][j] == 0 {
        //             p += 1;
        // /tools/libs.rsではここで置く処理をしていた
        // if p == self.ps[self.t] {
        //     self.board[i][j] = input.fs[self.t];
        //     self.last = (i, j);
        // }
        //     }
        // }
        // }
    }
}

fn compute_score(input: &Input, state: &State) -> i64 {
    // let mut state = State::new();
    // for t in 0..out.len().min(N * N - 1) {
    //     if let Err(err) = state.apply_move(out[t]) {
    //         return (0, format!("{} (turn: {})", err, t), state);
    //     }
    // }
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

// fn get_time() -> f64 {
//     let t = std::time::SystemTime::now()
//         .duration_since(std::time::UNIX_EPOCH)
//         .unwrap();
//     t.as_secs() as f64 + t.subsec_nanos() as f64 * 1e-9
// }

// struct Timer {
//     start_time: f64,
// }

// impl Timer {
//     fn new() -> Timer {
//         Timer {
//             start_time: get_time(),
//         }
//     }

//     fn get_time(&self) -> f64 {
//         get_time() - self.start_time
//     }
// }
