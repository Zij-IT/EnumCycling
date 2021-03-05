use enum_cycling::EnumCycle;

#[derive(Debug, PartialEq, EnumCycle)]
enum Alphabet {
    #[skip]
    A(i32, i32),
    B,
    C {
        c: i32,
    },
    D,
}

fn main() {
    assert_ne!(
        Alphabet::A(i32::default(), i32::default()),
        Alphabet::B.up()
    );
    assert_eq!(Alphabet::B.up(), Alphabet::C { c: 1 }.down());
    assert_eq!(Alphabet::D.up(), Alphabet::B.down());
}
