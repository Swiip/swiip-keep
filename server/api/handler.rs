use http::Method;
use serde_json::json;
use swiip_keep_server::set;
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(_req: Request) -> Result<Response<Body>, Error> {
    if _req.method() == Method::GET {
        return set::handler(_req);
    }

    Ok(Response::builder()
        .status(StatusCode::METHOD_NOT_ALLOWED)
        .header("Content-Type", "application/json")
        .body(
            json!({ "message": "Method not allowed" })
                .to_string()
                .into(),
        )?)
}
