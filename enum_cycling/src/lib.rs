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
/// It may auto generated for you using the `#[derive(EnumCycle]`
/// feature.
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
///     Continue,
///     Quit,
/// }
///
/// fn main() {
///     assert_eq!(MainMenu::NewGame.down(), MainMenu::Continue);
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
///         }
///     }
///     fn down(&self) -> Self {
///         match *self {
///             Self::NewGame => Self::Continue,
///             Self::Continue => Self::Quit,
///             Self::Quit => Self::NewGame,
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
