use ferris_says::say;
use std::{
    env,
    io::{stdout, BufWriter},
};

fn main() {
    let stdout = stdout();

    let message: String = env::args().skip(1).collect::<Vec<String>>().join(" ");

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), 32, &mut writer).unwrap();
}
