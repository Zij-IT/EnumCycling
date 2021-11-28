#[cfg(test)]
mod tests {
    use enum_cycling::EnumCycle;

    #[derive(Debug, PartialEq, EnumCycle)]
    enum Alphabet {
        A,
        B,
        C,
        D,
    }

    #[test]
    fn simple_up_test() {
        assert_eq!(Alphabet::B.up(), Alphabet::A);
    }

    #[test]
    fn simple_down_test() {
        assert_eq!(Alphabet::A.down(), Alphabet::B);
    }

    #[test]
    fn chaining_test() {
        assert_eq!(Alphabet::D.up().up(), Alphabet::B);
        assert_eq!(Alphabet::D.up().down(), Alphabet::D);
    }

    #[derive(Debug, PartialEq, EnumCycle)]
    enum SkipAlphabet {
        #[skip]
        A,
        B,
        C,
        D,
    }

    #[test]
    fn skip_up_test() {
        assert_eq!(SkipAlphabet::B.up(), SkipAlphabet::D);
    }

    #[test]
    fn skip_down_test() {
        assert_eq!(SkipAlphabet::B.down(), SkipAlphabet::C);
    }

    #[test]
    #[should_panic]
    fn calling_on_skipped() {
        SkipAlphabet::A.up();
    }

    #[derive(Debug, PartialEq, EnumCycle)]
    enum VariantTypes {
        Unit,
        Unnamed(i32),
        Named { field: String },
    }

    #[test]
    fn simple_variant_test() {
        assert_eq!(
            VariantTypes::Unit.up(),
            VariantTypes::Named {
                field: String::default()
            }
        );
        assert_eq!(
            VariantTypes::Unit.down(),
            VariantTypes::Unnamed(i32::default())
        );
        assert_eq!(VariantTypes::Unnamed(0).up(), VariantTypes::Unit);
    }

    #[derive(Debug, PartialEq, EnumCycle)]
    enum ComplexVT {
        Unit,
        Unnamed(i32, f32, usize, String),
        Named { f1: String, f2: f32, f3: i32 },
    }

    #[test]
    fn complex_variant_test() {
        assert_eq!(
            ComplexVT::Unit.up(),
            ComplexVT::Named {
                f1: String::default(),
                f2: f32::default(),
                f3: i32::default()
            }
        );
    }

    #[test]
    #[should_panic(expected = "Unable to call \"up\" fn on a skipped variant")]
    fn bad_up_call() {
        SkipAlphabet::A.up();
    }

    #[test]
    #[should_panic(expected = "Unable to call \"down\" fn on a skipped variant")]
    fn bad_down_call() {
        SkipAlphabet::A.down();
    }
}
