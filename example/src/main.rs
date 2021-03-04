use enum_cycling::EnumCycle;

#[derive(PartialEq, EnumCycle)]
enum Alphabet {
    A,
    B,
    C,
}

fn main() {
    assert!(Alphabet::A.up() == Alphabet::B);
    assert!(Alphabet::C.up() == Alphabet::A);
    assert!(Alphabet::C.down() == Alphabet::B);
    assert!(Alphabet::C.down() == Alphabet::A.up());
    assert!(Alphabet::A.up().up().up() == Alphabet::A.down().up());
}
