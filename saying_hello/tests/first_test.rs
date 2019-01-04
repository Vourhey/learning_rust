extern crate saying_hello;

#[test]
fn name_vadim() {
    assert_eq!("Hello, Vadim, nice to meet you!", saying_hello::return_greetings("Vadim"));
}

#[test]
fn name_123() {
    assert_eq!("Hello, 123, nice to meet you!", saying_hello::return_greetings("123"));
}

