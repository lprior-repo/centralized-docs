---
url: https://docs.aws.amazon.com/lambda/latest/dg/rust-http-events.html
title: Processing HTTP events with Rust
word_count: 455
filtered: true
elements_removed: 0
density_score: 0.88
---

Processing HTTP events with Rust - AWS Lambda
Processing HTTP events with Rust - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#rust-http-events)
# Processing HTTP events with Rust
Amazon API Gateway APIs, Application Load Balancers, and [Lambda function URLs](./urls-configuration.html) can send HTTP events to Lambda. You can use
the [aws\_lambda\_events](https://crates.io/crates/aws_lambda_events) crate
from crates.io to process events from these sources.
###### Example — Handle API Gateway proxy request
Note the following:
* `use aws\_lambda\_events::apigw::{ApiGatewayProxyRequest, ApiGatewayProxyResponse}`:
The [aws\_lambda\_events](https://crates.io/crates/aws-lambda-events) crate
includes many Lambda events. To reduce compilation time, use feature flags to activate the
events you need. Example:
`aws\_lambda\_events = { version = "0.8.3", default-features = false, features = ["apigw"] }`.
* `use http::HeaderMap`: This import requires you to add the [http](https://crates.io/crates/http) crate to your dependencies.
```
`use aws\_lambda\_events::apigw::{ApiGatewayProxyRequest, ApiGatewayProxyResponse};
use http::HeaderMap;
use lambda\_runtime::{service\_fn, Error, LambdaEvent};
async fn handler(
\_event: LambdaEvent&lt;&lt;ApiGatewayProxyRequest&gt;&gt;,
) -&gt;&gt; Result&lt;&lt;ApiGatewayProxyResponse, Error&gt;&gt; {
let mut headers = HeaderMap::new();
headers.insert("content-type", "text/html".parse().unwrap());
let resp = ApiGatewayProxyResponse {
status\_code: 200,
multi\_value\_headers: headers.clone(),
is\_base64\_encoded: false,
body: Some("Hello AWS Lambda HTTP request".into()),
headers,
};
Ok(resp)
}
#[tokio::main]
async fn main() -&gt;&gt; Result&lt;&lt;(), Error&gt;&gt; {
lambda\_runtime::run(service\_fn(handler)).await
}`
```
The [Rust runtime client for Lambda](https://github.com/aws/aws-lambda-rust-runtime) also provides an abstraction over these event types that
allows you to work with native HTTP types, regardless of which service sends the events. The
following code is equivalent to the previous example, and it works out of the box with Lambda
function URLs, Application Load Balancers, and API Gateway.
###### Note
The [lambda\_http](https://crates.io/crates/lambda_http) crate uses the [lambda\_runtime](https://crates.io/crates/lambda_runtime) crate
underneath. You don't have to import `lambda\_runtime` separately.
###### Example — Handle HTTP requests
```
`use lambda\_http::{service\_fn, Error, IntoResponse, Request, RequestExt, Response};
async fn handler(event: Request) -&gt;&gt; Result&lt;&lt;impl IntoResponse, Error&gt;&gt; {
let resp = Response::builder()
.status(200)
.header("content-type", "text/html")
.body("Hello AWS Lambda HTTP request")
.map\_err(Box::new)?;
Ok(resp)
}
#[tokio::main]
async fn main() -&gt;&gt; Result&lt;&lt;(), Error&gt;&gt; {
lambda\_http::run(service\_fn(handler)).await
}
`
```
For another example of how to use `lambda\_http`, see
the [http-axum code sample](https://github.com/aws/aws-lambda-rust-runtime/blob/main/examples/http-axum/src/main.rs) on the AWS Labs GitHub repository.
###### Sample HTTP Lambda events for Rust
* [Lambda HTTP events](https://github.com/aws/aws-lambda-rust-runtime/tree/main/examples/http-basic-lambda): A Rust function that handles HTTP
events.
* [Lambda
HTTP events with CORS headers](https://github.com/aws/aws-lambda-rust-runtime/blob/main/examples/http-cors): A Rust function that uses Tower to inject CORS
headers.
* [Lambda
HTTP events with shared resources](https://github.com/aws/aws-lambda-rust-runtime/tree/main/examples/basic-shared-resource): A Rust function that uses shared resources
initialized before the function handler is created.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Context
Deploy .zip file archives
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.