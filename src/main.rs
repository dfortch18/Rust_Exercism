mod exercism;

fn print_separator() {
    println!("-----------------------------------------");
}

fn main() {
    print_separator();

    println!("{}", exercism::hello::hello());

    print_separator();

    let reverse_string_inputs = vec!["robot", "Ramen", "I'm hungry!", "racecar", "drawer", "子猫", "Würstchenstand", "ผู้เขียนโปรแกรม"];

    for input in reverse_string_inputs {
        println!("Reverse of ({}): {}", input, exercism::reverse_string::reverse(input));
    }

    print_separator();
}
