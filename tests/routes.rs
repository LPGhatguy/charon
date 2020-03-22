use hyper::Method;

charon::router!(Router {
    Root: GET (/),
    Plain: GET (/plain),
    WithParam: GET (/with/{ id: u32 }),
    Wildcard: POST (/users/_/ham),
});

#[test]
fn routes_match() {
    assert!(matches!(
        Router::route(&Method::GET, "/"),
        Some(Router::Root(_))
    ));

    assert!(matches!(
        Router::route(&Method::GET, "/plain"),
        Some(Router::Plain(_))
    ));

    assert!(matches!(
        Router::route(&Method::GET, "/with/56"),
        Some(Router::WithParam(WithParam { id: 56 }))
    ));

    assert!(matches!(
        Router::route(&Method::POST, "/users/foobar/ham"),
        Some(Router::Wildcard(_))
    ));
}
