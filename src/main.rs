// This is a comment, and is ignored by the compiler
// You can test this code by clicking the "Run" button over there ->
// or if you prefer to use your keyboard, you can use the "Ctrl + Enter" shortcut

// This code is editable, feel free to hack it!
// You can always return to the original code by clicking the "Reset" button ->

extern crate jsonpath_lib as jsonpath;
extern crate serde_json;

use jsonpath::Selector;
use serde_json::Value;

// This is the main function
fn main() {
    // Statements here are executed when the compiled binary is called

    // Print text to the console
    println!("ok");
}

fn json_path_value(json: &str, json_path: &str) -> Option<Value> {
    let json: Value = serde_json::from_str(json).unwrap();
    let selector = Selector::new(json_path).unwrap();
    selector.find(&json).next().cloned()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_can_jsonpath() {
        let json = r#"{ "name": { "first": "John", "last": "Doe" }, "age": 30 }"#;
        let json_path = "$.name.first";
        let value = json_path_value(json, json_path);
        assert_eq!(value, Some(json!("John")));
    }
}
