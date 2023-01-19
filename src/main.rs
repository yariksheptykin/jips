extern crate jsonpath_lib as jsonpath;
#[macro_use]
extern crate serde_json;

fn main() {
    println!("ok");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_can_jsonpath() {
        let json_obj = json!({
            "store": {
                "book": [
                    {
                        "category": "reference",
                        "author": "Nigel Rees",
                        "title": "Sayings of the Century",
                        "price": 8.95
                    },
                    {
                        "category": "fiction",
                        "author": "Evelyn Waugh",
                        "title": "Sword of Honour",
                        "price": 12.99
                    },
                    {
                        "category": "fiction",
                        "author": "Herman Melville",
                        "title": "Moby Dick",
                        "isbn": "0-553-21311-3",
                        "price": 8.99
                    },
                    {
                        "category": "fiction",
                        "author": "J. R. R. Tolkien",
                        "title": "The Lord of the Rings",
                        "isbn": "0-395-19395-8",
                        "price": 22.99
                    }
                ],
                "bicycle": {
                    "color": "red",
                    "price": 19.95
                }
            },
            "expensive": 10
        });

        let mut selector = jsonpath::selector(&json_obj);

        assert_eq!(
            selector("$.store.book[*].author").unwrap(),
            vec![
                "Nigel Rees",
                "Evelyn Waugh",
                "Herman Melville",
                "J. R. R. Tolkien"
            ]
        );
    }
}
