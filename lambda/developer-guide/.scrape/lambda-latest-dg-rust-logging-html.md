---
url: https://docs.aws.amazon.com/lambda/latest/dg/rust-logging.html
title: Log and monitor Rust Lambda functions
word_count: 639
filtered: true
elements_removed: 0
density_score: 0.87
---

Log and monitor Rust Lambda functions - AWS Lambda
Log and monitor Rust Lambda functions - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#rust-logging)
[Creating a function that writes logs](#rust-logging-function)[Implementing advanced logging with the Tracing crate](#rust-logging-tracing)
# Log and monitor Rust Lambda functions
AWS Lambda automatically monitors Lambda functions on your behalf and sends logs to Amazon CloudWatch. Your Lambda function comes with a CloudWatch Logs log group and a log stream for each instance of your function. The Lambda runtime environment sends details about each invocation to the log stream, and relays logs and other output from your function's code. For more information, see [Sending Lambda function logs to CloudWatch Logs](./monitoring-cloudwatchlogs.html). For information about configuring log formats, see [Configuring JSON and plain text log formats](./monitoring-cloudwatchlogs-logformat.html). This page describes how to produce log output from your Lambda function's code.
## Creating a function that writes logs
To output logs from your function code, you can use any logging function that writes
to `stdout` or `stderr`, such as
the `println!` macro. The following example uses `println!` to
print a message when the function handler starts and before it finishes.
```
`use lambda\_runtime::{service\_fn, LambdaEvent, Error};
use serde\_json::{json, Value};
async fn handler(event: LambdaEvent&lt;Value&gt;) -&gt; Result&lt;Value, Error&gt; {
println!("Rust function invoked");
let payload = event.payload;
let first\_name = payload["firstName"].as\_str().unwrap\_or("world");
println!("Rust function responds to {}", &amp;&amp;first\_name);
Ok(json!({ "message": format!("Hello, {first\_name}!") }))
}
#[tokio::main]
async fn main() -&gt;&gt; Result&lt;&lt;(), Error&gt;&gt; {
lambda\_runtime::run(service\_fn(handler)).await
}
`
```
## Implementing advanced logging with the Tracing crate
[Tracing](https://crates.io/crates/tracing) is a framework for
instrumenting Rust programs to collect structured, event-based diagnostic information. This framework provides utilities to customize logging output levels and formats, like creating
structured JSON log messages. To use this framework, you must initialize
a `subscriber` before implementing the function handler. Then, you can use
tracing macros like `debug`, `info`, and `error`, to specify the level of logging that you want for each scenario.
###### Example — Using the Tracing crate
Note the following:
* `tracing\_subscriber::fmt().json()`: When this option is included, logs are
formatted in JSON. To use this option, you must include the `json` feature
in the `tracing-subscriber` dependency (for
example,`tracing-subscriber = { version = "0.3.11", features = ["json"] }`).
* `#[tracing::instrument(skip(event), fields(req\_id = %event.context.request\_id))]`:
This annotation generates a span every time the handler is invoked. The span adds the
request ID to each log line.
* `{ %first\_name }`: This construct adds the `first\_name` field to the log line where it's used. The value for this
field corresponds to the variable with the same name.
```
`use lambda\_runtime::{service\_fn, Error, LambdaEvent};
use serde\_json::{json, Value};
#[tracing::instrument(skip(event), fields(req\_id = %event.context.request\_id))]
async fn handler(event: LambdaEvent&lt;&lt;Value&gt;&gt;) -&gt;&gt; Result&lt;&lt;Value, Error&gt;&gt; {
tracing::info!("Rust function invoked");
let payload = event.payload;
let first\_name = payload["firstName"].as\_str().unwrap\_or("world");
tracing::info!({ %first\_name }, "Rust function responds to event");
Ok(json!({ "message": format!("Hello, {first\_name}!") }))
}
#[tokio::main]
async fn main() -&gt;&gt; Result&lt;&lt;(), Error&gt;&gt; {
tracing\_subscriber::fmt().json()
.with\_max\_level(tracing::Level::INFO)
// this needs to be set to remove duplicated information in the log.
.with\_current\_span(false)
// this needs to be set to false, otherwise ANSI color codes will
// show up in a confusing manner in CloudWatch logs.
.with\_ansi(false)
// disabling time is handy because CloudWatch will add the ingestion time.
.without\_time()
// remove the name of the function from every log entry
.with\_target(false)
.init();
lambda\_runtime::run(service\_fn(handler)).await
}
`
```
When this Rust function is invoked, it prints two log lines similar to the following:
```
`{"level":"INFO","fields":{"message":"Rust function invoked"},"spans":[{"req\_id":"45daaaa7-1a72-470c-9a62-e79860044bb5","name":"handler"}]}
{"level":"INFO","fields":{"message":"Rust function responds to event","first\_name":"David"},"spans":[{"req\_id":"45daaaa7-1a72-470c-9a62-e79860044bb5","name":"handler"}]}`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Layers
Best practices
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.