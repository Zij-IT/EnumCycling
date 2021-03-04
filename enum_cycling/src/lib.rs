pub use enum_cycling_derive::*;

pub trait EnumCycle {
    fn up(&self) -> Self;
    fn down(&self) -> Self;
}
