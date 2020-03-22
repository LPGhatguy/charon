#[macro_use]
mod macros;

mod traits;

pub use traits::{FromUriComponent, Route};

#[doc(hidden)]
pub mod internals {
    use crate::FromUriComponent;

    pub fn chomp_ty<'a, T: FromUriComponent>(
        components: &'a [&'a str],
    ) -> Option<(&'a [&'a str], T)> {
        components
            .get(0)
            .and_then(|segment| T::read(segment))
            .map(|value| (&components[1..], value))
    }

    pub fn chomp_exact<'a>(
        components: &'a [&'a str],
        expected_segment: &str,
    ) -> Option<&'a [&'a str]> {
        components.get(0).and_then(|segment| {
            if *segment == expected_segment {
                Some(&components[1..])
            } else {
                None
            }
        })
    }
}
