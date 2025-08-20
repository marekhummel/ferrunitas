pub trait QMarker {
    fn value(&self) -> f64;
}

pub trait Foo: Into<Self::Q> + From<Self::Q> {
    type Q: QMarker;
    const FACTOR: f64;
}

#[derive(Debug, Clone, Copy)]
struct QTest(pub f64);
impl QMarker for QTest {
    fn value(&self) -> f64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy)]
struct FooTest(f64);

impl From<FooTest> for QTest {
    fn from(foo: FooTest) -> QTest {
        QTest(foo.0 * FooTest::FACTOR)
    }
}

impl From<QTest> for FooTest {
    fn from(q: QTest) -> Self {
        FooTest(q.value() / FooTest::FACTOR)
    }
}

impl Foo for FooTest {
    type Q = QTest;
    const FACTOR: f64 = 2.0;
}

fn test1() {
    let x1 = FooTest(42.0);
    let y1: QTest = x1.into();
    let z1 = QTest::from(x1);

    let a1 = QTest(100.0);
    let b1: FooTest = a1.into();
    let c1 = FooTest::from(a1);

    println!(
        "x1: {:?}, y1: {:?}, z1: {:?}\na1: {:?}, b1: {:?}, c1: {:?}",
        x1, y1, z1, a1, b1, c1
    );
}

// -----------------------------------

trait Pref {
    const FACTOR: f64;
}
pub struct Forty;
impl Pref for Forty {
    const FACTOR: f64 = 40.0;
}

pub struct Bar<P: Pref, F: Foo>(pub f64, std::marker::PhantomData<(P, F)>);

impl<P: Pref, F: Foo> Bar<P, F> {
    pub fn new(value: f64) -> Self {
        Self(value, std::marker::PhantomData)
    }
}

type BarTest = Bar<Forty, FooTest>;

// impl<P: Pref, F: Foo> From<F::Q> for Bar<P, F> {
//     fn from(unit: F::Q) -> Self {
//         Self(
//             unit.value() * P::FACTOR * F::FACTOR,
//             std::marker::PhantomData,
//         )
//     }
// }

fn test2() {
    // let x1 = BarTest::new(42.0);
    // let y1: QTest = x1.into();
    // let z1 = QTest::from(x1);

    // let a1 = QTest::new(100.0);
    // let b1: FooTest = a1.into();
    // let c1 = FooTest::from(a1);

    // println!(
    //     "x1: {:?}, y1: {:?}, z1: {:?}\na1: {:?}, b1: {:?}, c1: {:?}",
    //     x1, y1, z1, a1, b1, c1
    // );
}

fn main() {
    test1();
    test2(); // Uncomment to test Bar with Foo
}
