use std::ops::Add;

#[derive(Debug, PartialEq)]
pub struct Foo(i32);

impl Add for Foo {
    type Output = Foo;
    fn add(self, other: Foo) -> Foo {
        Foo(self.0 + other.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn add() {
        let bar = Foo(1);
        let qux = Foo(2);
        assert_eq!(bar + qux, Foo(3));
    }
}
