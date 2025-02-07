// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.

// I AM NOT DONE

fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data);

    string_uppercase(data);
}


fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}


fn string_uppercase(mut data: String) {
    data = data.to_uppercase();
    println!("{}", data);
}