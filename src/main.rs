use std::env;

fn convert_string(input: &str) -> Vec<bool> {
    input.chars().map(|c| c == '1').collect()
}

fn sum_bit(suuji: &[bool]) -> i64 {
    let mut result = 0;
    for (i, &bit) in suuji.iter().enumerate() {
        if suuji[i] {
            result += 2_i32.pow(i as u32);
        }
    }
    result as i64
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: bin2dec string_of_0_and_1");
        std::process::exit(1);
    }
    let query = &args[1];
    if query.len() > 64 {
        println!("Number exceeds 64 bit. Aborting.");
        std::process::exit(1);
    }

    let values = convert_string(query);
    println!("{}", sum_bit(&values));
}
