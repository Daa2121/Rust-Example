use std::io::Write;
mod eve;

fn main() {
    // Purpose:    Driver for DH.
    // Parameters: None
    // User Input: If no args, input dec numbers
    // Prints:     If no args, then print result
    // Returns:    Nothing
    // Modifies:   Nothing
    // Calls:      ?
    // Tests:      arg_tests/ and stdio_tests/
    // Status:     Student does this
    // Check for command-line arguments
    // Check if command-line arguments are provided
    // Get command-line arguments
    // Get command-line arguments
    let args: Vec<String> = std::env::args().collect();

    if args.len() >= 3 {
        let input_file = &args[1];
        let output_file = &args[2];

        let input_data = std::fs::read_to_string(input_file).expect("Unable to open input file");
        let lines: Vec<&str> = input_data.split(" ").collect();

        let alice_broadcasts: u64 = lines[0].trim().parse().expect("Invalid Alice broadcast");
        let bob_broadcasts: u64 = lines[1].trim().parse().expect("Invalid Bob broadcast");
        let public_base: u64 = lines[2].trim().parse().expect("Invalid public base");
        let public_modulus: u64 = lines[3].trim().parse().expect("Invalid public modulus");

        let result = eve::big_eve(
            alice_broadcasts,
            bob_broadcasts,
            public_base,
            public_modulus,
        );

        let mut output = std::fs::File::create(output_file).expect("Unable to create output file");
        write!(output, "{} {} {}", result[0], result[1], result[2])
            .expect("Unable to write to output file");
    } else {
        let mut input = String::new();

        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let alice_broadcasts: u64 = input.trim().parse().expect("Invalid input");
        input.clear();

        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let bob_broadcasts: u64 = input.trim().parse().expect("Invalid input");
        input.clear();

        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let public_base: u64 = input.trim().parse().expect("Invalid input");
        input.clear();

        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let public_modulus: u64 = input.trim().parse().expect("Invalid input");

        let result = eve::big_eve(
            alice_broadcasts,
            bob_broadcasts,
            public_base,
            public_modulus,
        );

        println!("{} {} {}", result[0], result[1], result[2]);
    }
}
