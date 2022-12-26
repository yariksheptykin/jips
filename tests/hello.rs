

// Import the `hello` function.
use crate::hello;

#[test]
fn test_hello() {
    assert_eq!(hello(), "Hello, world!");
}
