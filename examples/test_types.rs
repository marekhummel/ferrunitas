use std::ops::Add;

struct UnitMarker;
struct ScalarMarker;

trait ToQ<T> {
    fn into_q(self) -> i32;
}

#[derive(Debug, Clone, Copy)]
struct Foo(i32); // pretend unit

impl ToQ<UnitMarker> for Foo {
    fn into_q(self) -> i32 {
        self.0 * 6
    }
}

impl ToQ<ScalarMarker> for i32 {
    fn into_q(self) -> i32 {
        self
    }
}

impl<U: ToQ<UnitMarker>> Add<U> for Foo {
    type Output = i32;
    fn add(self, rhs: U) -> Self::Output {
        self.0 + rhs.into_q()
    }
}

impl<U: ToQ<ScalarMarker>> Add<U> for Foo {
    type Output = i32;
    fn add(self, rhs: U) -> Self::Output {
        self.0 + rhs.into_q()
    }
}

fn main() {
    let q = Foo(10);
    let r1 = q + Foo(5);
    let r2 = q + 5;
    println!("{} {}", r1, r2);
}
