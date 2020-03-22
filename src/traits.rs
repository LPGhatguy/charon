use hyper::Method;

pub trait Route: Sized {
    fn check(method: &Method, components: &[&str]) -> Option<Self>;
}

pub trait FromUriComponent: Sized {
    fn read(component: &str) -> Option<Self>;
}

impl FromUriComponent for u32 {
    fn read(component: &str) -> Option<Self> {
        component.parse().ok()
    }
}
