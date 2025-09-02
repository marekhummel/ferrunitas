pub(crate) mod dimension;
pub(crate) mod measure;
pub(crate) mod prefix;
pub(crate) mod quantity;
pub(crate) mod unit;

/// Public trait in private module to prevent external implementations
mod sealed {
    pub trait Sealed {}
}
