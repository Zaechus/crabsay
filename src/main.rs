use ferris_says::say;
use std::{env, io::{stdout, BufWriter}};

fn main() {
    let stdout = stdout();
    let message: String = env::args().skip(1).collect::<Vec<String>>().join(" ");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}
