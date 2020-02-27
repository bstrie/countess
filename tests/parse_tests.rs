use countess::valid;

#[test]
fn parse_simple() {
    #[valid(0..10)]
    struct Foo(i32);
}

#[test]
fn parse_multiple_ranges() {
    #[valid(-20..=200, 30..300, ..40, 50..)]
    struct Foo(i32);
}
