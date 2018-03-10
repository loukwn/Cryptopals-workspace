mod part_a;

fn main() {
    // 1
    let test_input = String::from("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
    let base64_string = part_a::hex_to_base64(test_input);

    println!("Result: {}", base64_string);

    // 2
    let a = String::from("1c0111001f010100061a024b53535009181c");
    let b = String::from("686974207468652062756c6c277320657965");

    let xored_string = part_a::fixed_xor(a,b);

    println!("Result2: {}", xored_string);

    // 3
    let a = String::from("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");
    let b = String::from("ab");

    let xored_string = part_a::fixed_xor(a,b);
    println!("Result2: {}", xored_string);
}
