use hyper::Method;

pub trait Route: Sized {
    fn check(method: &Method, components: &[&str]) -> Option<Self>;
}

pub trait FromUriComponent: Sized {
    fn read(component: &str) -> Option<Self>;
}

impl FromUriComponent for String {
    fn read(component: &str) -> Option<Self> {
        Some(component.to_owned())
    }
}

macro_rules! from_uri_for_parseables {
    ( $( $ty: ty, )+ ) => {
        $(
            impl FromUriComponent for $ty {
                fn read(component: &str) -> Option<Self> {
                    component.parse().ok()
                }
            }
        )+
    };
}

from_uri_for_parseables! {
    u8,
    u16,
    u32,
    u64,
    u128,
    i8,
    i16,
    i32,
    i64,
    i128,
    f32,
    f64,
}
