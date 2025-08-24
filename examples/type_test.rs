// macro_rules! __typenum_from_lit {
//     (0) => {
//         typenum::Z0
//     };
//     (1) => {
//         typenum::P1
//     };
//     (-1) => {
//         typenum::N1
//     };
//     ($other:literal) => {
//         compile_error!("typenum_from_lit!: unsupported exponent literal")
//     };
// }

// macro_rules! make_vec {
//     ($exp:literal) => {
//         Vec<__typenum_from_lit!($exp)>
//     };
// }

// fn main() {
//     type Foo = __typenum_from_lit!(1);
//     type Test = make_vec!(1);
// }

macro_rules! __typenum_from_lit {
    (0) => {
       Vec<typenum::Z0>
    };
    (1) => {
        Vec<typenum::P1>
    };
    (-1) => {
        Vec<typenum::N1>
    };
    ($other:literal) => {
        compile_error!("typenum_from_lit!: unsupported exponent literal")
    };
}

type Foo = __typenum_from_lit!(1); // expands to typenum::P1 (a type)
type Test = __typenum_from_lit!(1); // expands to Vec<typenum::P1>

fn test(list: Vec<typenum::P1>) {
    println!("List length: {}", list.len());
}

fn main() {
    let x: Test = Vec::new(); // Vec<P1>
    let y: Foo = Vec::new(); // also Vec<P1>
    println!("{}", std::any::type_name::<Test>());
    println!("{}", std::any::type_name::<Foo>());
    test(x);
    test(y);
}
