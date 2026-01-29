---
url: https://docs.aws.amazon.com/lambda/latest/dg/lambda-managed-instances-nodejs-runtime.html
title: Node.js runtime for Lambda Managed Instances
word_count: 1253
filtered: true
elements_removed: 0
density_score: 0.88
---

Node.js runtime for Lambda Managed Instances - AWS Lambda
Node.js runtime for Lambda Managed Instances - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#lambda-managed-instances-nodejs-runtime)
[Maximum concurrency](#lambda-managed-instances-nodejs-max-concurrency)[Building functions for multi-concurrency](#lambda-managed-instances-nodejs-building)[Node.js 22 callback-based handlers](#lambda-managed-instances-nodejs-callback-handlers)[Shared /tmp directory](#lambda-managed-instances-nodejs-shared-tmp)[Logging](#lambda-managed-instances-nodejs-logging)[Request context](#lambda-managed-instances-nodejs-request-context)[Initialization and shutdown](#lambda-managed-instances-nodejs-init-shutdown)[Dependency versions](#lambda-managed-instances-nodejs-dependencies)[Powertools for AWS Lambda (TypeScript)](#lambda-managed-instances-nodejs-powertools)[Next steps](#lambda-managed-instances-nodejs-next-steps)
# Node.js runtime for Lambda Managed Instances
For Node.js runtimes, Lambda Managed Instances uses worker threads with `async`/`await`-based execution to handle concurrent requests. Function initialization occurs once per worker thread. Concurrent invocations are handled across two dimensions: worker threads provide parallelism across vCPUs, and asynchronous execution provides concurrency within each thread. Each concurrent request handled by the same worker thread shares the same handler object and global state, requiring safe handling under multiple concurrent requests.
## Maximum concurrency
The maximum number of concurrent requests which Lambda sends to each execution environment is controlled by the `PerExecutionEnvironmentMaxConcurrency` setting in the function configuration. This is an optional setting, and the default value varies depending on the runtime. For Node.js runtimes, the default is 64 concurrent requests per vCPU, or you can configure your own value. Lambda automatically adjusts the number of concurrent requests up to the configured maximum based on the capacity of each execution environment to absorb those requests.
For Node.js, the number of concurrent requests that each execution environment can process is determined by the number of worker threads and the capacity of each worker thread to process concurrent requests asynchronously. The default number of worker threads is determined by the number of vCPUs available, or you can configure the number of worker threads by setting the `AWS\_LAMBDA\_NODEJS\_WORKER\_COUNT` environment variable. We recommend using async function handlers since this allows processing multiple requests per worker thread. If your function handler is synchronous, each worker thread can only process a single request at a time.
## Building functions for multi-concurrency
With an async function handler, each runtime worker processes multiple requests concurrently. Global objects will be shared across multiple concurrent requests. For mutable objects, avoid using global state or use `AsyncLocalStorage`.
AWS SDK clients are async safe and do not require special handling.
**Example: Global state**
The following code uses a global object which is mutated inside the function handler. This is not async-safe.
```
`let state = {
currentUser: null,
requestData: null
};
export const handler = async (event, context) =&gt; {
state.currentUser = event.userId;
state.requestData = event.data;
await processData(state.requestData);
// state.currentUser might now belong to a different request
return { user: state.currentUser };
};`
```
Initialising the `state` object inside the function handler avoids shared global state.
```
`export const handler = async (event, context) =&gt; {
let state = {
currentUser: event.userId,
requestData: event.data
};
await processData(state.requestData);
return { user: state.currentUser };
};`
```
**Example: Database connections**
The following code uses a shared client object which is shared between multiple invocations. Depending on the connection library used, this may not be concurrency safe.
```
`const { Client } = require('pg');
// Single connection created at init time
const client = new Client({
host: process.env.DB\_HOST,
database: process.env.DB\_NAME,
user: process.env.DB\_USER,
password: process.env.DB\_PASSWORD
});
// Connect once during cold start
client.connect();
exports.handler = async (event) =&gt;&gt; {
// Multiple parallel invocations share this single connection = BAD
// With multi-concurrent Lambda, queries will collide
const result = await client.query('SELECT \* FROM users WHERE id = $1', [event.userId]);
return {
statusCode: 200,
body: JSON.stringify(result.rows[0])
};
};`
```
A concurrency-safe approach is to use a connection pool. The pool uses a separate connection for each concurrent database query.
```
`const { Pool } = require('pg');
// Connection pool created at init time
const pool = new Pool({
host: process.env.DB\_HOST,
database: process.env.DB\_NAME,
user: process.env.DB\_USER,
password: process.env.DB\_PASSWORD,
max: 20, // Max connections in pool
idleTimeoutMillis: 30000,
connectionTimeoutMillis: 2000
});
exports.handler = async (event) =&gt;&gt; {
// Pool gives each parallel invocation its own connection
const result = await pool.query('SELECT \* FROM users WHERE id = $1', [event.userId]);
return {
statusCode: 200,
body: JSON.stringify(result.rows[0])
};
};`
```
## Node.js 22 callback-based handlers
When using Node.js 22, you cannot use a callback-based function handler with Lambda Managed Instances. Callback-based handlers are only supported for Lambda (default) functions. For Node.js 24 and later runtimes, callback-based function handlers are deprecated for both Lambda (default) and Lambda Managed Instances.
Instead, use an `async` function handler when using Lambda Managed Instances. For more information, see [Define Lambda function handler in Node.js](https://docs.aws.amazon.com/lambda/latest/dg/nodejs-handler.html).
## Shared /tmp directory
The `/tmp` directory is shared across all concurrent requests in the execution environment. Concurrent writes to the same file can cause data corruption, for example if another process overwrites the file. To address this, either implement file locking for shared files or use unique file names per request to avoid conflicts. Remember to clean up unneeded files to avoid exhausting the available space.
## Logging
Log interleaving (log entries from different requests being interleaved in logs) is normal in multi-concurrent systems. Functions using Lambda Managed Instances always use the structured JSON log format introduced with [advanced logging controls](./monitoring-logs.html#monitoring-cloudwatchlogs-advanced). This format includes the `requestId`, allowing log entries to be correlated to a single request. When you use the `console` logger, the `requestId` is automatically included in each log entry. For further information, see [Using Lambda advanced logging controls with Node.js](./nodejs-logging.html#node-js-logging-advanced).
Popular third-party logging libraries, such as [Winston](https://github.com/winstonjs/winston), typically include support for using console for log output.
## Request context
Using `context.awsRequestId` provides async-safe access to the request ID for the current request.
Use `context.xRayTraceId` to access the X-Ray trace ID. This provides concurrency-safe access to the trace ID for the current request. Lambda does not support the `\_X\_AMZN\_TRACE\_ID` environment variable with Lambda Managed Instances. The X-Ray trace ID is propagated automatically when using the AWS SDK.
Use `context.getRemainingTimeInMillis()` to detect timeouts. See [Error handling and recovery](./lambda-managed-instances-execution-environment.html#lambda-managed-instances-error-handling) for more information.
## Initialization and shutdown
Function initialization occurs once per worker thread. You may see repeat log entries if your function emits logs during initialization.
For Lambda functions with extensions, the execution environment emits a SIGTERM signal during shut down. This signal is used by extensions to trigger clean up tasks, such as flushing buffers. Lambda (default) functions with extensions can also subscribe to the SIGTERM signal using `process.on()`. This is not supported for functions using Lambda Managed Instances since `process.on()` cannot be used with worker threads. To learn more about the execution environment lifecycle, see [Understanding the Lambda execution environment lifecycle](./lambda-runtime-environment.html).
## Dependency versions
Lambda Managed Instances requires the following minimum package versions:
* AWS SDK for JavaScript v3: version 3.933.0 or later
* AWS X-Ray SDK for Node.js: version 3.12.0 or later
* AWS Distro for OpenTelemetry - Instrumentation for JavaScript: version 0.8.0 or later
* Powertools for AWS Lambda (TypeScript): version 2.29.0 or later
## Powertools for AWS Lambda (TypeScript)
Powertools for AWS Lambda (TypeScript) is compatible with Lambda Managed Instances and provides utilities for logging, tracing, metrics, and more. For more information, see [Powertools for AWS Lambda (TypeScript)](https://github.com/aws-powertools/powertools-lambda-typescript).
## Next steps
* Review [Java runtime for Lambda Managed Instances](./lambda-managed-instances-java-runtime.html)
* Review [Python runtime for Lambda Managed Instances](./lambda-managed-instances-python-runtime.html)
* Review [.NET runtime for Lambda Managed Instances](./lambda-managed-instances-dotnet-runtime.html)
* Learn about [scaling Lambda Managed Instances](./lambda-managed-instances-scaling.html)
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Java runtime
Python runtime
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.