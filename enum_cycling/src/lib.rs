//! # Enum Cycling
//!
//! Enum Cycling is a crate that allows one to
//! more easily navigate enums in Rust.
//!
//! The full version of the README can be found on Github
//!
//! # How to include Enum Cycling
//!
//! Import enum_cycling into your project by adding this line to your
//! Cargo.toml.
//!
//! ```toml
//! [dependencies]
//! enum_cycling = "0.1.0"
//! enum_cycling_derive = "0.1.0"
//!```

/// This trait is the central piece to move up and down an `Enum`.
/// By using `#[derive(EnumCycle]` you can save yourself the hassle of
/// having to write the implementation.
///
/// # Example
///
/// ```rust
/// //Bring it into scope
/// use enum_cycling::EnumCycle;
///
/// #[derive(PartialEq, Debug, EnumCycle)]
/// enum MainMenu {
///     NewGame,
///
///     #[skip]
///     Secret,
///
///     Continue,
///     Quit,
/// }
///
/// fn main() {
///     assert_eq!(MainMenu::NewGame.down(), MainMenu::Continue);
/// }
/// ```
///
/// Additionally, you may use the 'cycle' attribute in order to keep
/// the order of your cycle independant from the enum Variant order.
/// This allows you to keep your enum variants alphabetically sorted,
/// while also using EnumCycle.
///
/// ```rust
/// use enum_cycling::EnumCycle;
///
/// #[derive(PartialEq, Debug, EnumCycle)]
/// #[cycle(NewGame, Continue, Quit)]
/// enum MainMenu {
///     Continue,
///     NewGame,
///     Quit,
///     Secret,
/// }
/// ```
///
/// The generated code should look something along the lines of:
///
/// ```nocompile
/// impl EnumCycle for MainMenu {
///     fn up(&self) -> Self {
///         match *self {
///             Self::NewGame => Self::Quit,
///             Self::Continue => Self::NewGame,
///             Self::Quit => Self::Continue,
///             _ => panic!("Unable to call 'up' on a skipped variant"),
///         }
///     }
///     fn down(&self) -> Self {
///         match *self {
///             Self::NewGame => Self::Continue,
///             Self::Continue => Self::Quit,
///             Self::Quit => Self::NewGame,
///             _ => panic!("Unable to call 'down' on a skipped variant"),
///         }
///     }
/// }
/// ```
pub trait EnumCycle {
    fn up(&self) -> Self;
    fn down(&self) -> Self;
}

#[cfg(feature = "derive")]
pub use enum_cycling_derive::EnumCycle;
