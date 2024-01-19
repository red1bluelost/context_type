#![no_std]
pub mod boolean;

pub mod __internal {
    pub use paste as __paste;
}

#[cfg(any(test, doctest))]
mod tests;
