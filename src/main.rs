// This is a comment, and is ignored by the compiler
// You can test this code by clicking the "Run" button over there ->
// or if you prefer to use your keyboard, you can use the "Ctrl + Enter" shortcut

// This code is editable, feel free to hack it!
// You can always return to the original code by clicking the "Reset" button ->

// This is the main function
fn main() {
    // Statements here are executed when the compiled binary is called

    // Print text to the console
    println!("{}", hello());
}


fn hello() -> &'static str {
    "Hello, world!"
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_greets() {
        assert_eq!("Hello, world!", hello());
    }
}
