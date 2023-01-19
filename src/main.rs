extern crate jsonpath_lib as jsonpath;
extern crate alloc;
#[macro_use]
extern crate serde_json;

fn main() {
    println!("ok");
}

fn getAuthors() -> alloc::string::String {
    let ret = jsonpath::select_as_str(r#"
    {
        "school": {
            "friends": [
                    {"name": "친구1", "age": 20},
                    {"name": "친구2", "age": 20}
                ]
        },
        "friends": [
            {"name": "친구3", "age": 30},
            {"name": "친구4"}
        ]
    }
    "#, "$..friends[0]").unwrap();

    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_can_jsonpath() {
        assert_eq!(getAuthors(), r#"[{"name":"친구3","age":30},{"name":"친구1","age":20}]"#);
    }
}
