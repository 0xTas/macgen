use rand::{thread_rng, Rng};

fn main() {
    let fake_addr: u64 = thread_rng().gen_range(0x100000000000..=0xffffffffffff);
    let fake_addr_str = format!("{:2X}", fake_addr);

    let substrings = fake_addr_str
        .as_bytes()
        .chunks(2)
        .map(std::str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .expect("Expected valid chunks!");

    let formatted_addr = substrings.join(":");

    let mut args = std::env::args();

    if args.any(|arg| arg == "-n") {
        println!("{}", formatted_addr);
    } else if args.any(|arg| arg.to_lowercase() == "-h" || arg.to_lowercase() == "--help") {
        println!("Usage: ./macgen [-n (appends newline)]");
    } else {
        print!("{}", formatted_addr);
    };
}
