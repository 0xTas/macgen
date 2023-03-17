use rand::{seq::SliceRandom, thread_rng, Rng};

fn main() {
    let oui_prefixes: Vec<&str> = vec![
        "00:0E:F6", "00:08:28", "84:7A:88", "D8:A2:5E", "00:30:0B", "00:26:0C", "A4:C5:4E",
        "94:86:CD", "E0:35:60", "00:19:BC", "70:01:36", "FC:1F:C0", "00:E0:DE", "00:07:19",
        "00:1B:AF", "00:24:27", "28:4F:CE", "00:22:A0", "74:40:BB", "28:E7:94", "C4:93:00",
        "30:A2:20", "00:17:88", "02:5B:76", "2C:F4:32", "40:16:7E", "40:B0:76", "5C:A6:E6",
        "60:E3:27", "68:9A:87", "62:5C:65", "68:9A:87", "A0:8C:FD", "B8:BB:AF", "F0:F2:74",
        "F0:35:75", "F2:34:11", "CC:40:D0", "20:4E:7F", "AC:37:43", "04:0E:C2", "18:2B:05",
        "00:04:D2", "24:B6:FD", "F0:46:3B", "00:50:FF", "00:0B:2A", "00:1F:ED", "00:22:FB",
        "CC:3F:EA", "00:02:BF",
    ];

    let iters = if std::env::args().any(|arg| arg.parse::<u32>().is_ok()) {
        let arg_num = std::env::args()
            .find(|arg| arg.parse::<u32>().is_ok())
            .expect("Pre-checked for numeric arg.")
            .parse::<u32>()
            .expect("Already verified as parsable arg.");

        if arg_num == 0 {
            1
        } else {
            arg_num
        }
    } else {
        1
    };

    if std::env::args().any(|arg| arg.to_lowercase() == "-h" || arg.to_lowercase() == "--help") {
        println!("Usage: ./macgen [-n (won't append newline)] [num (e.g 5)]");
    } else {
        for _ in 0..iters {
            let mut rng = thread_rng();
            let fake_addr: u64 = thread_rng().gen_range(0x100000..=0xffffff);
            let fake_addr_str = format!("{:2X}", fake_addr);

            let substrings = fake_addr_str
                .as_bytes()
                .chunks(2)
                .map(std::str::from_utf8)
                .collect::<Result<Vec<&str>, _>>()
                .expect("Expected valid chunks!");

            let formatted_addr = substrings.join(":");

            if std::env::args().any(|arg| arg == "-n") {
                print!(
                    "{}:{} ",
                    oui_prefixes
                        .choose(&mut rng)
                        .expect("Vec should not be empty."),
                    formatted_addr
                );
            } else {
                println!(
                    "{}:{}",
                    oui_prefixes
                        .choose(&mut rng)
                        .expect("Vec should not be empty."),
                    formatted_addr
                );
            };
        }
    };
}
