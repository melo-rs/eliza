pub struct Response {}

pub trait IntoResponse {
    fn into_response(self) -> Response;
}

impl IntoResponse for &'static str {
    fn into_response(self) -> Response {
        Response {}
    }
}
