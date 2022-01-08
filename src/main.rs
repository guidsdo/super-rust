use ferris_says;
use std::io;

fn main() {
    let stdout = io::stdout();
    let message = String::from("Hello world");
    let width = message.chars().count();

    let mut writer = io::BufWriter::new(stdout.lock());

    ferris_says::say(message.as_bytes(), width, &mut writer).unwrap_or({});
}
