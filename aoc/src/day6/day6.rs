use std::collections::HashSet;

fn find_signal(input: &str, offset: usize) -> usize {
    return offset
        + input
            .as_bytes()
            .windows(offset)
            .position(|window| HashSet::<u8>::from_iter(window.iter().copied()).len() == offset)
            .unwrap();
}
fn main() {
    let file = include_str!("signal.txt");
    let signal_start = find_signal(file, 4);
    let message_start = find_signal(file, 14);
    println! {"Signal starts at {}, and message starts at {}", signal_start, message_start};
}
