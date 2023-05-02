use lambda_http::{run, service_fn, Body, Error, Request, Response};
use reqwest::Client;
mod session_scrapper;
mod session_types;

use crate::session_scrapper::get_new_session;
/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
async fn function_handler(_: Request, client: &Client) -> Result<Response<Body>, Error> {
    // Extract some useful information from the request
    let session = get_new_session(&client)
        .await
        .map_err(|e| format!("Failed to get session: {}", e.to_string()))?;
    // Return something that implements IntoResponse.
    // It will be serialized to the right response event automatically by the runtime
    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(serde_json::to_string(&session).unwrap().into())
        .map_err(Box::new)?;
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();
    let reqwest_client = reqwest::Client::new();
    run(service_fn(|req| function_handler(req, &reqwest_client))).await
}
