pub fn run(input: &str) -> (String, String) {
    let input = parse(input);
    (part1(&input).to_string(), part2(&input).to_string())
}

pub fn parse(input: &str) -> String {
    input.to_string()
}

pub fn part1(input: &str) -> u64 {
    let mut number = 1;
    loop {
        let data = format!("{}{}", input, number);
        let digest = md5::compute(data.as_bytes());
        // Check for 5 leading zeros in hex (20 bits):
        // - digest.0[0] checks first byte (8 bits / 2 hex digits)
        // - digest.0[1] checks second byte (8 bits / 2 hex digits)
        // - digest.0[2] & 0xf0 checks first 4 bits of third byte (1 hex digit)
        if (digest.0[0] | digest.0[1] | (digest.0[2] & 0xf0)) == 0 {
            return number;
        }
        number += 1;
    }
}

pub fn part2(input: &str) -> usize {
    let mut number = 1;
    loop {
        let data = format!("{}{}", input, number);
        let digest = md5::compute(data.as_bytes());
        // Check for 6 leading zeros in hex (24 bits):
        // - digest.0[0] checks first byte (8 bits / 2 hex digits)
        // - digest.0[1] checks second byte (8 bits / 2 hex digits)
        // - digest.0[2] checks third byte (8 bits / 2 hex digits)
        if (digest.0[0] | digest.0[1] | digest.0[2]) == 0 {
            return number;
        }
        number += 1;
    }
}
