use dotenv_codegen::dotenv;
use lambda_runtime::{service_fn, tracing, Error, LambdaEvent};
use serde::{Deserialize, Serialize};
use spam::Daybreak;
use tracing::info;

/// This is a made-up example. Requests come into the runtime as unicode
/// strings in json format, which can map to any structure that implements `serde::Deserialize`
/// The runtime pays no attention to the contents of the request payload.
#[derive(Deserialize)]
struct Request {
    time: String,
}

/// This is a made-up example of what a response structure may look like.
/// There is no restriction on what it can be. The runtime requires responses
/// to be serialized into json. The runtime pays no attention
/// to the contents of the response payload.
#[derive(Serialize)]
struct Response {
    req_id: String,
    msg: String,
}

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
/// - https://github.com/aws-samples/serverless-rust-demo/
async fn function_handler(db: &Daybreak, event: LambdaEvent<Request>) -> Result<Response, Error> {
    // Extract some useful info from the request
    let time = event.payload.time;

    let msg = if db.checkdate(&time[..10]) {
        db.burnice().await;
        "burnin day"
    } else {
        "No burnin :("
    };
    info!(msg);
    // Prepare the response
    let resp = Response {
        req_id: event.context.request_id,
        msg: msg.to_owned(),
    };

    // Return `Response` (it will be serialized to JSON automatically by the runtime)
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();
    let start_date = dotenv!("DATE");
    let msg = dotenv!("MSG");
    let confstr = include_str!("artifacts/config.txt");
    let token = dotenv!("GTOKEN");
    let db = Daybreak::new(start_date, msg, confstr, token);
    info!("Finished creating Daybreak");
    //println!("{}", db.checkdate("2024.10.28"));
    let shared = &db;
    lambda_runtime::run(service_fn(|event: LambdaEvent<Request>| async move {
        function_handler(&shared, event).await
    }))
    .await
}
