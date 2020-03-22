use hyper::{header, Body, Error, Request, Response, StatusCode};

struct MyCoolType(String);

impl charon::FromUriComponent for MyCoolType {
    fn read(component: &str) -> Option<Self> {
        Some(Self(component.to_string()))
    }
}

charon::router!(Router {
    ApiPlain: GET (/api/hello),
    ApiRead: GET (/api/read/{ id: MyCoolType }),
    ApiWrite: POST (/api/write),
    ApiSubscribe: GET (/api/subscribe/{ cursor: u32 }),
    ApiWildcard: GET (/api/foo/ */bar),
});

async fn handle_request(request: Request<Body>) -> Result<Response<Body>, Error> {
    let method = request.method();
    let path = request.uri().path();

    match Router::route(method, path) {
        Some(Router::ApiPlain(args)) => unimplemented!(),
        Some(Router::ApiRead(args)) => unimplemented!(),
        Some(Router::ApiWrite(args)) => unimplemented!(),
        Some(Router::ApiSubscribe(args)) => unimplemented!(),
        Some(Router::ApiWildcard(args)) => unimplemented!(),

        None => Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(r#"{ "message": "route not found" }"#))
            .unwrap()),
    }
}

fn main() {
    // do nothing, this code compiling means enough
}
