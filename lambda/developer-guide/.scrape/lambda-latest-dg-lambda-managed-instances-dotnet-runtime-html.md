---
url: https://docs.aws.amazon.com/lambda/latest/dg/lambda-managed-instances-dotnet-runtime.html
title: .NET runtime for Lambda Managed Instances
word_count: 943
filtered: true
elements_removed: 0
density_score: 0.87
---

.NET runtime for Lambda Managed Instances - AWS Lambda
.NET runtime for Lambda Managed Instances - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#lambda-managed-instances-dotnet-runtime)
[Concurrency configuration](#lambda-managed-instances-dotnet-concurrency-config)[Building functions for multi-concurrency](#lambda-managed-instances-dotnet-building)[Shared /tmp directory](#lambda-managed-instances-dotnet-shared-tmp)[Logging](#lambda-managed-instances-dotnet-logging)[Request context](#lambda-managed-instances-dotnet-request-context)[Initialization and shutdown](#lambda-managed-instances-dotnet-init-shutdown)[Dependency versions](#lambda-managed-instances-dotnet-dependencies)[Powertools for AWS Lambda (.NET)](#lambda-managed-instances-dotnet-powertools)[Next steps](#lambda-managed-instances-dotnet-next-steps)
# .NET runtime for Lambda Managed Instances
For .NET runtimes, Lambda Managed Instances use a single .NET process per execution environment. Multiple concurrent requests are processed using .NET Tasks.
## Concurrency configuration
The maximum number of concurrent requests which Lambda sends to each execution environment is controlled by the `PerExecutionEnvironmentMaxConcurrency` setting in the function configuration. This is an optional setting, and the default value varies depending on the runtime. For .NET runtimes, the default is 32 concurrent requests per vCPU, or you can configure your own value. Lambda automatically adjusts the number of concurrent requests up to the configured maximum based on the capacity of each execution environment to absorb those requests.
## Building functions for multi-concurrency
You should apply the same concurrency safety practices when using Lambda Managed Instances as you would in any other multi-concurrent environment. Since the handler object is shared across all Tasks any mutable state must be thread-safe. This includes collections, database connections and any static objects that are modified during request processing.
AWS SDK clients are thread safe and do not require special handling.
**Example: Database connection pools**
The following code uses a static database connection object which is shared between concurrent requests. The `SqlConnection` object is not thread safe.
```
`public class DBQueryHandler
{
// Single connection shared across threads - NOT SAFE
private SqlConnection connection;
public DBQueryHandler()
{
connection = new SqlConnection("your-connection-string-here");
connection.Open();
}
public string Handle(object input, ILambdaContext context)
{
using var cmd = connection.CreateCommand();
cmd.CommandText = "SELECT ..."; // your query
using var reader = cmd.ExecuteReader();
...
}
}`
```
To address this, use a separate connection for each request, drawn from a connection pool. ADO.NET providers like `Microsoft.Data.SqlClient` automatically support connection pooling when the connection object is opened.
```
`public class DBQueryHandler
{
public DBQueryHandler()
{
}
public string Handle(object input, ILambdaContext context)
{
using var connection = new SqlConnection("your-connection-string-here");
connection.Open();
using var cmd = connection.CreateCommand();
cmd.CommandText = "SELECT ..."; // your query
using var reader = cmd.ExecuteReader();
...
}
}`
```
**Example: Collections**
Standard .NET collections are not thread safe:
```
`public class Handler
{
private static List&lt;string&gt; items = new List&lt;string&gt;();
private static Dictionary&lt;string, object&gt; cache = new Dictionary&lt;string, object&gt;();
public string FunctionHandler(object input, ILambdaContext context)
{
items.Add(context.AwsRequestId);
cache["key"] = input;
return "Success";
}
}`
```
Use collections from the `System.Collections.Concurrent` namespace for concurrency safety:
```
`public class Handler
{
private static ConcurrentBag&lt;string&gt; items = new ConcurrentBag&lt;string&gt;();
private static ConcurrentDictionary&lt;string, object&gt; cache = new ConcurrentDictionary&lt;string, object&gt;();
public string FunctionHandler(object input, ILambdaContext context)
{
items.Add(context.AwsRequestId);
cache["key"] = input;
return "Success";
}
}`
```
## Shared /tmp directory
The `/tmp` directory is shared across all concurrent requests in the execution environment. Concurrent writes to the same file can cause data corruption, for example if another request overwrites the file. To address this, either implement file locking for shared files or use unique file names per request to avoid conflicts. Remember to clean up unneeded files to avoid exhausting the available space.
## Logging
Log interleaving (log entries from different requests being interleaved in logs) is normal in multi-concurrent systems. Functions using Lambda Managed Instances always use the structured JSON log format introduced with [advanced logging controls](./monitoring-logs.html#monitoring-cloudwatchlogs-advanced). This format includes the `requestId`, allowing log entries to be correlated to a single request. When you use the `context.Logger` object to generate logs, the `requestId` is automatically included in each log entry. For further information, see [Using Lambda advanced logging controls with .NET](./csharp-logging.html#csharp-logging-advanced).
## Request context
Use the `context.AwsRequestId` property to access to the request ID for the current request.
Use the `context.TraceId` property to access the X-Ray trace ID. This provides concurrency-safe access to the trace ID for the current request. Lambda does not support the `\_X\_AMZN\_TRACE\_ID` environment variable with Lambda Managed Instances. The X-Ray trace ID is propagated automatically when using the AWS SDK.
Use `ILambdaContext.RemainingTime` to detect timeouts. See [Error handling and recovery](./lambda-managed-instances-execution-environment.html#lambda-managed-instances-error-handling) for more information.
## Initialization and shutdown
Function initialization occurs once per execution environment. Objects created during initialization are shared across requests.
For Lambda functions with extensions, the execution environment emits a SIGTERM signal during shut down. This signal is used by extensions to trigger clean up tasks, such as flushing buffers. You can subscribe to SIGTERM events to trigger function clean-up tasks, such as closing database connections. To learn more about the execution environment lifecycle, see [Understanding the Lambda execution environment lifecycle](./lambda-runtime-environment.html).
## Dependency versions
Lambda Managed Instances requires the following minimum package versions:
* Amazon.Lambda.Core: version 2.7.1 or later
* Amazon.Lambda.RuntimeSupport: version 1.14.1 or later
* OpenTelemetry.Instrumentation.AWSLambda: version 1.14.0 or later
* AWSXRayRecorder.Core: version 2.16.0 or later
* AWSSDK.Core: version 4.0.0.32 or later
## Powertools for AWS Lambda (.NET)
[Powertools for AWS Lambda (.NET)](https://docs.aws.amazon.com/powertools/dotnet/) and [AWS Distro for OpenTelemetry - Instrumentation for DotNet](https://github.com/aws-observability/aws-otel-dotnet-instrumentation) currently do not support Lambda Managed Instances.
## Next steps
* Review [Java runtime for Lambda Managed Instances](./lambda-managed-instances-java-runtime.html)
* Review [Node.js runtime for Lambda Managed Instances](./lambda-managed-instances-nodejs-runtime.html)
* Review [Python runtime for Lambda Managed Instances](./lambda-managed-instances-python-runtime.html)
* Learn about [scaling Lambda Managed Instances](./lambda-managed-instances-scaling.html)
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Python runtime
Networking
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.