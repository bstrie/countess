use countess::range;

#[range(99..999)]
struct Foo;

#[test]
fn works() {
    assert_eq!(99, Foo::LOWER);
}
