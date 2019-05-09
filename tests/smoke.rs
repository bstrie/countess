use countess::range;

#[range(-20..=200, 30..300, ..40, 50..)]
struct Foo(i32);

#[test]
fn works() {
    assert_eq!(-20, Foo::LOWER);
}
