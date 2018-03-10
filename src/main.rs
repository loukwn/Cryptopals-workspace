mod partA;

fn main() {
    let testInput = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d".to_string();
    let base64String = partA::hex_to_base64(testInput);

    println!("Result: {}", base64String);
}
