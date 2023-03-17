use rand::{thread_rng, Rng};

fn main() {
    let fake_addr: u64 = thread_rng().gen_range(0x100000000000..=0xffffffffffff);
    let fake_addr_str = format!("{:2X}", fake_addr);

    let substrings = fake_addr_str.as_bytes()
        .chunks(2)
        .map(std::str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .expect("Expected valid chunks!");

    let formatted_addr = substrings.join(":");
    print!("{}", formatted_addr);
}
