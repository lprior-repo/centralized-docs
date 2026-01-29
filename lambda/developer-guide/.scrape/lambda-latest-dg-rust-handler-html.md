---
url: https://docs.aws.amazon.com/lambda/latest/dg/rust-handler.html
title: Define Lambda function handlers in Rust
word_count: 2683
filtered: true
elements_removed: 0
density_score: 0.84
---

Define Lambda function handlers in Rust - AWS Lambda
Define Lambda function handlers in Rust - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#rust-handler)
[Setting up your Rust handler project](#rust-handler-setup)[Example Rust Lambda function code](#rust-example-code)[Valid class definitions for Rust handlers](#rust-handler-signatures)[Handler naming conventions](#rust-example-naming)[Defining and accessing the input event object](#rust-handler-input)[Accessing and using the Lambda context object](#rust-example-context)[Using the AWS SDK for Rust in your handler](#rust-example-sdk-usage)[Accessing environment variables](#rust-example-envvars)[Using shared state](#rust-shared-state)[Code best practices for Rust Lambda functions](#rust-best-practices)
# Define Lambda function handlers in Rust
The Lambda function *handler* is the method in your function code that processes events. When your function is
invoked, Lambda runs the handler method. Your function runs until the handler returns a response, exits, or times out.
This page describes how to work with Lambda function handlers in Rust, including
project initialization, naming conventions, and best practices. This page also includes
an example of a Rust Lambda function that takes in information about an order, produces
a text file receipt, and puts this file in an Amazon Simple Storage Service (S3) bucket. For more information
about how to deploy your function after writing it, see [Deploy Rust Lambda functions with .zip file archives](./rust-package.html).
###### Topics
* [Setting up your Rust handler project](#rust-handler-setup)
* [Example Rust Lambda function code](#rust-example-code)
* [Valid class definitions for Rust handlers](#rust-handler-signatures)
* [Handler naming conventions](#rust-example-naming)
* [Defining and accessing the input event object](#rust-handler-input)
* [Accessing and using the Lambda context object](#rust-example-context)
* [Using the AWS SDK for Rust in your handler](#rust-example-sdk-usage)
* [Accessing environment variables](#rust-example-envvars)
* [Using shared state](#rust-shared-state)
* [Code best practices for Rust Lambda functions](#rust-best-practices)
## Setting up your Rust handler project
When working with Lambda functions in Rust, the process involves writing your
code, compiling it, and deploying the compiled artifacts to Lambda. The simplest way
to set up a Lambda handler project in Rust is to use the
[AWS Lambda Runtime for Rust](https://github.com/aws/aws-lambda-rust-runtime).
Despite its name, the AWS Lambda Runtime for Rust is not a managed runtime in the same sense as
it is in Lambda for Python, Java, or Node.js. Instead, the AWS Lambda Runtime for Rust is a crate
(`lambda\_runtime`) that supports writing Lambda functions in Rust and interfacing
with AWS Lambda's execution environment.
Use the following command to install [Cargo Lambda](https://www.cargo-lambda.info/guide/what-is-cargo-lambda.html), a third-party open-source extension to the Cargo command-line tool that simplifies building and deploying Rust Lambda functions:
```
`cargo install cargo-lambda`
```
After you successfully install `cargo-lambda`, use the following command
to initialize a new Rust Lambda function handler project:
```
`cargo lambda new example-rust`
```
When you run this command, the command line interface (CLI) asks you a couple of
questions about your Lambda function:
* **HTTP function** – If you intend to invoke
your function via [API Gateway](./services-apigateway.html) or a
[function URL](./urls-configuration.html), answer
**Yes**. Otherwise, answer **No**.
In the example code on this page, we invoke our function with a custom JSON event,
so we answer **No**.
* **Event type** – If you intend to use a
predefined event shape to invoke your function, select the correct expected event type.
Otherwise, leave this option blank. In the example code on this page, we invoke our
function with a custom JSON event, so we leave this option blank.
After the command runs successfully, enter the main directory of your project:
```
`cd example-rust`
```
This command generates a `generic\_handler.rs` file and a `main.rs`
file in the `src` directory. The `generic\_handler.rs` can be used to
customize a generic event handler. The `main.rs` file contains your main
application logic. The `Cargo.toml` file contains metadata about your package
and lists its external dependencies.
## Example Rust Lambda function code
The following example Rust Lambda function code takes in information about an order,
produces a text file receipt, and puts this file in an Amazon S3 bucket.
###### Example `main.rs` Lambda function
```
`use aws\_sdk\_s3::{Client, primitives::ByteStream};
use lambda\_runtime::{run, service\_fn, Error, LambdaEvent};
use serde::{Deserialize, Serialize};
use serde\_json::Value;
use std::env;
#[derive(Deserialize, Serialize)]
struct Order {
order\_id: String,
amount: f64,
item: String,
}
async fn function\_handler(event: LambdaEvent&lt;&lt;Value&gt;&gt;) -&gt;&gt; Result&lt;&lt;String, Error&gt;&gt; {
let payload = event.payload;
// Deserialize the incoming event into Order struct
let order: Order = serde\_json::from\_value(payload)?;
let bucket\_name = env::var("RECEIPT\_BUCKET")
.map\_err(|\_| "RECEIPT\_BUCKET environment variable is not set")?;
let receipt\_content = format!(
"OrderID: {}\\nAmount: ${:.2}\\nItem: {}",
order.order\_id, order.amount, order.item
);
let key = format!("receipts/{}.txt", order.order\_id);
let config = aws\_config::load\_defaults(aws\_config::BehaviorVersion::latest()).await;
let s3\_client = Client::new(&amp;&amp;config);
upload\_receipt\_to\_s3(&amp;&amp;s3\_client, &amp;&amp;bucket\_name, &amp;&amp;key, &amp;&amp;receipt\_content).await?;
Ok("Success".to\_string())
}
async fn upload\_receipt\_to\_s3(
client: &amp;&amp;Client,
bucket\_name: &amp;&amp;str,
key: &amp;&amp;str,
content: &amp;&amp;str,
) -&gt;&gt; Result&lt;&lt;(), Error&gt;&gt; {
client
.put\_object()
.bucket(bucket\_name)
.key(key)
.body(ByteStream::from(content.as\_bytes().to\_vec())) // Fixed conversion
.content\_type("text/plain")
.send()
.await?;
Ok(())
}
#[tokio::main]
async fn main() -&gt;&gt; Result&lt;&lt;(), Error&gt;&gt; {
run(service\_fn(function\_handler)).await
}
`
```
This `main.rs` file contains the following sections of code:
* `use` statements: Use these to import Rust crates and methods that your Lambda
function requires.
* `#[derive(Deserialize, Serialize)]`: Define the shape of the expected input
event in this Rust struct.
* `async fn function\_handler(event: LambdaEvent&lt;&lt;Value&gt;&gt;) -&gt;&gt; Result&lt;&lt;String, Error&gt;&gt;`:
This is the **main handler method**, which contains your main
application logic.
* `async fn upload\_receipt\_to\_s3 (...)`: This is a helper method that's
referenced by the main `function\_handler` method.
* `#[tokio::main]`: This is a macro that marks the entry point of a Rust program.
It also sets up a [Tokio runtime](https://docs.rs/tokio/latest/tokio/runtime/index.html),
which allows your `main()` method to use `async`/`await` and run
asynchronously.
* `async fn main() -&gt; Result&lt;(), Error&gt;`: The `main()`
function is the entry point of your code. Within it, we specify `function\_handler`
as the main handler method.
The following `Cargo.toml` file accompanies this function.
```
`[package]
name = "example-rust"
version = "0.1.0"
edition = "2024"
[dependencies]
aws-config = "1.5.18"
aws-sdk-s3 = "1.78.0"
lambda\_runtime = "0.13.0"
serde = { version = "1", features = ["derive"] }
serde\_json = "1"
tokio = { version = "1", features = ["full"] }`
```
For this function to work properly, its [
execution role](./lambda-intro-execution-role.html) must allow the `s3:PutObject` action. Also, ensure that you
define the `RECEIPT\_BUCKET` environment variable. After a successful invocation,
the Amazon S3 bucket should contain a receipt file.
## Valid class definitions for Rust handlers
In most cases, Lambda handler signatures that you define in Rust will have the following
format:
```
`async fn function\_handler(event: LambdaEvent&lt;&lt;T&gt;&gt;) -&gt;&gt; Result&lt;&lt;U, Error&gt;&gt;`
```
For this handler:
* The name of this handler is `function\_handler`.
* The singular input to the handler is event, and is of type `LambdaEvent&lt;T&gt;`.
* `LambdaEvent` is a wrapper that comes from the `lambda\_runtime`
crate. Using this wrapper gives you access to the context object, which includes
Lambda-specific metadata such as the request ID of the invocation.
* `T` is the deserialized event type. For example, this can be
`serde\_json::Value`, which allows the handler to take in any generic JSON input.
Alternatively, this can be a type like `ApiGatewayProxyRequest` if your function
expects a specific, pre-defined input type.
* The return type of the handler is `Result&lt;U, Error&gt;`.
* `U` is the deserialized output type. `U` must implement the
`serde::Serialize` trait so Lambda can convert the return value to JSON. For
example, `U` can be a simple type like `String`,
`serde\_json::Value`, or a custom struct as long as it implements
`Serialize`. When your code reaches an Ok(U) statement, this indicates
successful execution, and your function returns a value of type `U`.
* When your code encounters an error (i.e. `Err(Error)`), your function
logs the error in Amazon CloudWatch and returns an error response of type `Error`.
In our example, the handler signature looks like the following:
```
`async fn function\_handler(event: LambdaEvent&lt;&lt;Value&gt;&gt;) -&gt;&gt; Result&lt;&lt;String, Error&gt;&gt;`
```
Other valid handler signatures can feature the following:
* Omitting the `LambdaEvent` wrapper – If you omit `LambdaEvent`,
you lose access to the Lambda context object within your function. The following is an
example of this type of signature:
```
`async fn handler(event: serde\_json::Value) -&gt;&gt; Result&lt;&lt;String, Error&gt;&gt;`
```
* Using the unit type as an input – For Rust, you can use the unit type to
represent an empty input. This is commonly used for functions with periodic, scheduled
invocations. The following is an example of this type of signature:
```
`async fn handler(\_: ()) -&gt;&gt; Result&lt;&lt;Value, Error&gt;&gt;`
```
## Handler naming conventions
Lambda handlers in Rust don’t have strict naming restrictions. Although you can use any name
for your handler, function names in Rust are generally in `snake\_case`.
For smaller applications, such as in this example, you can use a single `main.rs`
file to contain all of your code. For larger projects, `main.rs` should contain the
entry point to your function, but you can have additional files for that separate your code into
logical modules. For example, you might have the following file structure:
```
`/example-rust
│── src/
│ ├── main.rs # Entry point
│ ├── handler.rs # Contains main handler
│ ├── services.rs # [Optional] Back-end service calls
│ ├── models.rs # [Optional] Data models
│── Cargo.toml`
```
## Defining and accessing the input event object
JSON is the most common and standard input format for Lambda functions. In this example,
the function expects an input similar to the following:
```
`{
"order\_id": "12345",
"amount": 199.99,
"item": "Wireless Headphones"
}`
```
In Rust, you can define the shape of the expected input event in a struct. In this example,
we define the following struct to represent an `Order`:
```
`#[derive(Deserialize, Serialize)]
struct Order {
order\_id: String,
amount: f64,
item: String,
}`
```
This struct matches the expected input shape. In this example, the
`#[derive(Deserialize, Serialize)]` macro automatically generates code for
serialization and deserialization. This means that we can deserialize the generic input JSON type
into our struct using the `serde\_json::from\_value()` method. This is illustrated in
the first few lines of the handler:
```
`async fn function\_handler(event: LambdaEvent&lt;&lt;Value&gt;&gt;) -&gt;&gt; Result&lt;&lt;String, Error&gt;&gt; {
let payload = event.payload;
// Deserialize the incoming event into Order struct
let order: Order = serde\_json::from\_value(payload)?;
...
}`
```
You can then access the fields of the object. For example, `order.order\_id` retrieves
the value of `order\_id` from the original input.
### Pre-defined input event types
There are many pre-defined input event types available in the `aws\_lambda\_events`
crate. For example, if you intend to invoke your function with API Gateway, including the
following import:
```
`use aws\_lambda\_events::event::apigw::ApiGatewayProxyRequest;`
```
Then, make sure your main handler uses the following signature:
```
`async fn handler(event: LambdaEvent&lt;ApiGatewayProxyRequest&gt;) -&gt; Result&lt;String, Error&gt; {
let body = event.payload.body.unwrap\_or\_default();
...
}`
```
Refer to the [aws\_lambda\_events crate](https://crates.io/crates/aws_lambda_events)
for more information about other pre-defined input event types.
## Accessing and using the Lambda context object
The Lambda [context object](./rust-context.html) contains information about the
invocation, function, and execution environment. In Rust, the `LambdaEvent` wrapper
includes the context object. For example, you can use the context object to retrieve the request ID
of the current invocation with the following code:
```
`async fn function\_handler(event: LambdaEvent&lt;&lt;Value&gt;&gt;) -&gt;&gt; Result&lt;&lt;String, Error&gt;&gt; {
let request\_id = event.context.request\_id;
...
}`
```
For more information about the context object, see [Using the Lambda context object to retrieve Rust function information](./rust-context.html).
## Using the AWS SDK for Rust in your handler
Often, you’ll use Lambda functions to interact with or make updates to other AWS resources.
The simplest way to interface with these resources is to use the
[AWS SDK for Rust](https://docs.aws.amazon.com/sdk-for-rust/latest/dg/welcome.html).
To add SDK dependencies to your function, add them in your `Cargo.toml` file. We
recommend only adding the libraries that you need for your function. In the example code earlier,
we used the `aws\_sdk\_s3::Client`. In the `Cargo.toml` file, you can add
this dependency by adding the following line under the `[dependencies]` section:
```
`aws-sdk-s3 = "1.78.0"`
```
###### Note
This may not be the most recent version. Choose the appropriate version for your application.
The, import the dependencies directly in your code:
```
`use aws\_sdk\_s3::{Client, primitives::ByteStream};`
```
The example code then initializes an Amazon S3 client as follows:
```
`let config = aws\_config::load\_defaults(aws\_config::BehaviorVersion::latest()).await;
let s3\_client = Client::new(&amp;&amp;config);`
```
After you initialize your SDK client, you can then use it to interact with other AWS services.
The example code calls the Amazon S3 `PutObject` API in the `upload\_receipt\_to\_s3`
helper function.
## Accessing environment variables
In your handler code, you can reference any [environment
variables](./configuration-envvars.html) by using the `env::var` method. In this example, we reference the
defined `RECEIPT\_BUCKET` environment variable using the following line of code:
```
`let bucket\_name = env::var("RECEIPT\_BUCKET")
.map\_err(|\_| "RECEIPT\_BUCKET environment variable is not set")?;`
```
## Using shared state
You can declare shared variables that are independent of your Lambda function's handler code.
These variables can help you load state information during
the [Init phase](./lambda-runtime-environment.html#runtimes-lifecycle-ib), before your function
receives any events. For example, you can modify the code on this page to use shared state when
initializing the Amazon S3 client by updating the `main` function and handler signature:
```
`async fn function\_handler(client: &amp;&amp;Client, event: LambdaEvent&lt;&lt;Value&gt;&gt;) -&gt;&gt; Result&lt;&lt;String, Error&gt;&gt; {
...
upload\_receipt\_to\_s3(client, &amp;&amp;bucket\_name, &amp;&amp;key, &amp;&amp;receipt\_content).await?;
...
}
...
#[tokio::main]
async fn main() -&gt;&gt; Result&lt;&lt;(), Error&gt;&gt; {
let shared\_config = aws\_config::from\_env().load().await;
let client = Client::new(&amp;&amp;shared\_config);
let shared\_client = &amp;&amp;client;
lambda\_runtime::run(service\_fn(move |event: LambdaEvent&lt;&lt;Request&gt;&gt;| async move {
handler(&amp;&amp;shared\_client, event).await
}))
.await`
```
## Code best practices for Rust Lambda functions
Adhere to the guidelines in the following list to use best coding practices when building your Lambda functions:
* **Separate the Lambda handler from your core logic.** This allows you to make
a more unit-testable function.
* **Minimize the complexity of your dependencies.** Prefer simpler frameworks
that load quickly on [execution environment](./lambda-runtime-environment.html) startup.
* **Minimize your deployment package size to its runtime necessities. ** This
will reduce the amount of time that it takes for your deployment package to be downloaded and unpacked ahead
of invocation.
**Take advantage of execution environment reuse to improve the performance of your
function.** Initialize SDK clients and database connections outside of the function handler, and
cache static assets locally in the `/tmp` directory. Subsequent invocations processed by
the same instance of your function can reuse these resources. This saves cost by reducing function run time.
To avoid potential data leaks across invocations, don’t use the execution environment to store user data,
events, or other information with security implications. If your function relies on a mutable state that can’t
be stored in memory within the handler, consider creating a separate function or separate versions of a
function for each user.
**Use a keep-alive directive to maintain persistent connections.** Lambda purges idle connections over time.
Attempting to reuse an idle connection when invoking a function will result in a connection error. To maintain your persistent connection, use the
keep-alive directive associated with your runtime.
For an example, see [Reusing Connections with Keep-Alive in Node.js](https://docs.aws.amazon.com/sdk-for-javascript/v3/developer-guide/node-reusing-connections.html).
**Use [environment variables](./configuration-envvars.html) to pass operational parameters to your function.** For example, if
you are writing to an Amazon S3 bucket, instead of hard-coding the bucket name you are writing to, configure the
bucket name as an environment variable.
**Avoid using recursive invocations** in your Lambda function, where the function
invokes itself or initiates a process that may invoke the function again. This could lead to unintended volume of
function invocations and escalated costs. If you see an unintended volume of invocations, set the function reserved concurrency
to `0` immediately to throttle all invocations to the function, while you update the
code.
**Do not use non-documented, non-public APIs** in your Lambda function code.
For AWS Lambda managed runtimes, Lambda periodically applies security and functional updates to Lambda's internal APIs.
These internal API updates may be backwards-incompatible, leading to unintended consequences such as invocation
failures if your function has a dependency on these non-public APIs. See
[the API reference](https://docs.aws.amazon.com/lambda/latest/api/welcome.html) for a list of
publicly available APIs.
**Write idempotent code.** Writing idempotent code for your functions ensures that
duplicate events are handled the same way. Your code should properly validate events and gracefully handle
duplicate events. For more information, see
[How do I make
my Lambda function idempotent?](https://aws.amazon.com/premiumsupport/knowledge-center/lambda-function-idempotent/).
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Building with Rust
Context
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.