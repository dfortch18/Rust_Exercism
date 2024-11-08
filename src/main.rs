mod exercism;

fn print_separator() {
    println!("-----------------------------------------");
}

fn main() {
    print_separator();

    println!("{}", exercism::hello::hello());

    print_separator();
}
