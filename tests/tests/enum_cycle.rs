use enum_cycling::EnumCycle;

#[derive(Debug, PartialEq, EnumCycle)]
enum Alphabet {
    A,
    B,
    C,
    D,

    #[skip]
    E,
}

#[test]
fn simple() {
    assert_eq!(Alphabet::A.down(), Alphabet::B);
    assert_eq!(Alphabet::B.up(), Alphabet::A);
}

#[test]
fn chaining() {
    assert_eq!(Alphabet::D.up().up(), Alphabet::B);
    assert_eq!(Alphabet::D.up().down(), Alphabet::D);
}

#[test]
fn simple_with_skip() {
    assert_eq!(Alphabet::A.up(), Alphabet::D);
    assert_eq!(Alphabet::D.down(), Alphabet::A);
}

#[test]
fn chaining_with_skip() {
    assert_eq!(Alphabet::B.up().up(), Alphabet::D);
    assert_eq!(Alphabet::D.down().down(), Alphabet::B);
}

#[test]
#[should_panic(expected = "Unable to call \"up\" fn on a skipped variant")]
fn up_on_skip_variant() {
    Alphabet::E.up();
}

#[test]
#[should_panic(expected = "Unable to call \"down\" fn on a skipped variant")]
fn down_on_skip_variant() {
    Alphabet::E.down();
}

#[derive(Debug, PartialEq, EnumCycle)]
enum EnumVariantStruct {
    Unit,
    Unnamed(i32, f32, usize, String),
    Named { f1: String, f2: f32, f3: i32 },
}

#[test]
fn struct_variants() {
    assert_eq!(
        EnumVariantStruct::Unit.up(),
        EnumVariantStruct::Named {
            f1: String::default(),
            f2: f32::default(),
            f3: i32::default()
        }
    );

    assert_eq!(
        EnumVariantStruct::Unit.down(),
        EnumVariantStruct::Unnamed(
            i32::default(),
            f32::default(),
            usize::default(),
            String::default()
        )
    );
}

#[derive(PartialEq, EnumCycle)]
#[cycle(A,B,C,D)]
enum Cyclebet {
    C,
    A,
    D,
    B,

    #[allow(dead_code)]
    E,
}

#[test]
fn simple_cycle() {
    assert_eq!(Alphabet::A.down(), Alphabet::B);
    assert_eq!(Alphabet::B.up(), Alphabet::A);
}

#[test]
fn chaining_cycle() {
    assert_eq!(Alphabet::D.up().up(), Alphabet::B);
    assert_eq!(Alphabet::D.up().down(), Alphabet::D);
}

#[test]
fn simple_cycle_with_skip() {
    assert_eq!(Alphabet::A.up(), Alphabet::D);
    assert_eq!(Alphabet::D.down(), Alphabet::A);
}

#[test]
fn chaining_cycle_with_skip() {
    assert_eq!(Alphabet::B.up().up(), Alphabet::D);
    assert_eq!(Alphabet::D.down().down(), Alphabet::B);
}

#[test]
#[should_panic(expected = "Unable to call \"up\" fn on a skipped variant")]
fn up_on_skip_cycle_variant() {
    Alphabet::E.up();
}

#[test]
#[should_panic(expected = "Unable to call \"down\" fn on a skipped variant")]
fn down_on_skip_cycle_variant() {
    Alphabet::E.down();
}
