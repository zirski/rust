use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filepath = &args[0];
    let query = &args[1];
}
