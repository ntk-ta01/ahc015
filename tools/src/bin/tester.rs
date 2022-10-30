use std::{
    fs,
    io::{Read, Write},
    path::PathBuf,
    process::Stdio,
    thread,
};

fn exec(file_path: PathBuf, out_flag: bool) -> i64 {
    let mut file = fs::File::open(&file_path).unwrap();
    let mut buf = vec![];
    file.read_to_end(&mut buf).unwrap_or_else(|e| {
        eprintln!("failed to read {:?}", file);
        eprintln!("{}", e);
        std::process::exit(1)
    });
    let command = "cargo";
    let p = std::process::Command::new(command)
        .arg("run")
        .arg("--release")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap_or_else(|e| {
            eprintln!("failed to execute the command");
            eprintln!("{}", e);
            std::process::exit(1)
        });
    let mut stdin = p.stdin.as_ref().unwrap();
    stdin.write_all(&buf).unwrap();
    let output = p.wait_with_output().unwrap();
    if out_flag {
        let file_path = format!(
            "./tools/out/{}",
            file_path.file_name().unwrap().to_string_lossy()
        );
        let mut file = fs::File::create(&file_path).unwrap();
        file.write_all(&output.stdout).unwrap();
    }
    let b = String::from_utf8(output.stderr).unwrap();
    let s = b.split('\n').collect::<Vec<_>>();
    let score = s[s.len() - 2].split(':').collect::<Vec<_>>();
    // if print_flag {
    //     println!(
    //         "{}|{}:{}",
    //         file_path.display(),
    //         score[0],
    //         score[1].parse::<i64>().unwrap()
    //     );
    // }
    if let Ok(score) = score[0].parse::<i64>() {
        score
    } else {
        eprintln!(
            "failed ./tools/out/{}",
            file_path.file_name().unwrap().to_string_lossy()
        );
        0
    }
}

fn main() {
    let out_flag = std::env::args().into_iter().any(|s| &s == "out");
    let files = fs::read_dir("./tools/in/").unwrap();
    let mut handles = vec![];
    for file in files {
        let file = file.unwrap();
        let file_path = file.path();
        let handle = thread::spawn(move || exec(file_path, out_flag));
        handles.push(handle);
    }
    let mut total_score = 0;
    let case_num = handles.len();
    for handle in handles {
        let score = handle.join().unwrap();
        total_score += score;
    }
    const PRETESTNUM: i64 = 200;
    println!("local_score:{}", total_score * PRETESTNUM / case_num as i64);
}
