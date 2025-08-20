pub trait QMarker {
    fn new(value: f64) -> Self;
    fn value(&self) -> f64;
}

pub trait Foo: Into<Self::Q> + From<Self::Q> {
    type Q: QMarker;
    const FACTOR: f64;
}

pub struct Baz<F: Foo>(pub f64, std::marker::PhantomData<F>);

impl<F: Foo> Baz<F> {
    pub fn new(value: f64) -> Self {
        Self(value, std::marker::PhantomData)
    }
}

// impl<F: Foo> From<F::Q> for Baz<F> {
//     fn from(unit: F::Q) -> Self {
//         Self(unit.value() * F::FACTOR, std::marker::PhantomData)
//     }
// }

// -----------------------------------

struct QTest(pub f64);
impl QMarker for QTest {
    fn new(value: f64) -> Self {
        Self(value)
    }
    fn value(&self) -> f64 {
        self.0
    }
}

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

type BazTest = Baz<FooTest>;

fn test() {
    let x1 = FooTest(42.0);
    let y1: QTest = x1.into();

    let a1 = QTest::new(100.0);
    let b1: FooTest = a1.into();

    // let x2 = BazTest::new(42.0);
    // let y2: QTest = x2.into();

    // let a2 = QTest::new(100.0);
    // let b2: BazTest = a2.into();
}
