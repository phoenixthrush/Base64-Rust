const BASE64_CHARS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

fn string_to_binary(input: &str) -> Vec<String> {
    input
        .chars()
        .map(|char| {
            let binary = format!("{:08b}", char as u8);
            binary
        })
        .collect()
}

fn chunk_bits(bits: Vec<String>) -> Vec<String> {
    let bit_string = bits.concat();
    // chunk in 6 bits
    let mut chunks: Vec<String> = bit_string
        .chars()
        .collect::<Vec<char>>()
        .chunks(6)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect();

    println!("{:?}", chunks);

    // pad last chunk if not 6 bits
    if let Some(last) = chunks.last_mut() {
        if last.len() != 6 {
            last.push_str(&"0".repeat(6 - last.len()));
        }
    }

    chunks
}

fn bits_to_key(bits: Vec<String>) -> String {
    let base64_key: String = bits
        .iter()
        .map(|chunk| {
            // convert binary to decimal
            let bit_value = usize::from_str_radix(chunk, 2).expect("Invalid binary chunk");
            // map decimal to base64 key
            BASE64_CHARS
                .chars()
                .nth(bit_value)
                .expect("Invalid Base64 index")
        })
        .collect();

    // if chunks are not a multiple of four pad with '='
    let padding = (4 - (base64_key.len() % 4)) % 4;
    let padded_key = format!("{}{}", base64_key, "=".repeat(padding));

    padded_key
}

fn main() {
    let mut string = String::new();
    println!("Enter something: ");

    std::io::stdin()
        .read_line(&mut string)
        .expect("Could not read user input");

    let string_as_binary = string_to_binary(&mut string);
    println!("{:?}", string_as_binary);

    let chunked_bits = chunk_bits(string_as_binary);
    println!("{:?}", chunked_bits);

    let bits_as_key = bits_to_key(chunked_bits);
    println!("{}", bits_as_key);
}
