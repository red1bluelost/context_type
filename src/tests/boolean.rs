#![cfg_attr(any(test, doctest), allow(dead_code))]

/// ```compile_fail
/// context_type::yes_no! {
///     enum Hello;
/// }
///
/// ```
#[cfg(doctest)]
pub struct FailWithoutFullPathYesNo;

/// ```rust
/// context_type::boolean::yes_no! {
///     enum Hello;
/// }
///
/// ```
#[cfg(doctest)]
pub struct AllowFullPathYesNo;

/// ```compile_fail
/// context_type::boolean::yes_no! {
///     enum Hello {
///         default = 3
///     }
/// }
///
/// ```
#[cfg(doctest)]
pub struct FailWithInvalidDefaultWrongType;
