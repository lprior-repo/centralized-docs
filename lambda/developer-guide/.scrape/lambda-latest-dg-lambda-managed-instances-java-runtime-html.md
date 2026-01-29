---
url: https://docs.aws.amazon.com/lambda/latest/dg/lambda-managed-instances-java-runtime.html
title: Java runtime for Lambda Managed Instances
word_count: 1140
filtered: true
elements_removed: 0
density_score: 0.85
---

Java runtime for Lambda Managed Instances - AWS Lambda
Java runtime for Lambda Managed Instances - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#lambda-managed-instances-java-runtime)
[Concurrency configuration](#lambda-managed-instances-java-concurrency-config)[Building functions for multi-concurrency](#lambda-managed-instances-java-building)[Shared /tmp directory](#lambda-managed-instances-java-shared-tmp)[Logging](#lambda-managed-instances-java-logging)[Request context](#lambda-managed-instances-java-request-context)[Initialization and shutdown](#lambda-managed-instances-java-init-shutdown)[Dependency versions](#lambda-managed-instances-java-dependencies)[Powertools for AWS Lambda (Java)](#lambda-managed-instances-java-powertools)[Next steps](#lambda-managed-instances-java-next-steps)
# Java runtime for Lambda Managed Instances
For Java runtimes, Lambda Managed Instances use OS threads for concurrency. Lambda loads your handler object once per execution environment during initialization and then creates multiple threads. These threads execute in parallel and require thread-safe handling of state and shared resources. Each thread shares the same handler object and any static fields.
## Concurrency configuration
The maximum number of concurrent requests which Lambda sends to each execution environment is controlled by the `PerExecutionEnvironmentMaxConcurrency` setting in the function configuration. This is an optional setting, and the default value varies depending on the runtime. For Java runtimes, the default is 32 concurrent requests per vCPU, or you can configure your own value. This value also determines the number of threads used by the Java runtime. Lambda automatically adjusts the number of concurrent requests up to the configured maximum based on the capacity of each execution environment to absorb those requests.
## Building functions for multi-concurrency
You should apply the same thread safety practices when using Lambda Managed Instances as you would in any other multi-threaded environment. Since the handler object is shared across all runtime worker threads, any mutable state must be thread-safe. This includes collections, database connections, and any static objects that are modified during request processing.
AWS SDK clients are thread safe and do not require special handling.
**Example: Database connection pools**
The following code uses a static database connection object which is shared between threads. Depending on the connection library used, this may not be thread safe.
```
`public class DBQueryHandler implements RequestHandler&lt;Object, String&gt; {
// Single connection shared across all threads - NOT SAFE
private static Connection connection;
public DBQueryHandler() {
this.connection = DriverManager.getConnection(jdbcUrl, username, password);
}
@Override
public String handleRequest(Object input, Context context) {
PreparedStatement stmt = connection.prepareStatement(query);
ResultSet rs = stmt.executeQuery();
// Multiple threads using same connection causes issues
return result.toString();
}
}`
```
A thread-safe approach is to use a connection pool. In the following example, the function handler retrieves a connection from the pool. The connection is only used in the context of a single request.
```
`public class DBQueryHandler implements RequestHandler&lt;Object, String&gt; {
private static HikariDataSource dataSource;
public DBQueryHandler() {
HikariConfig config = new HikariConfig();
config.setJdbcUrl("jdbc:mysql://localhost:3306/your\_database");
dataSource = new HikariDataSource(config); // Create pool once per Lambda container
}
@Override
public String handleRequest(Object input, Context context) {
String query = "SELECT column\_name FROM your\_table LIMIT 10";
StringBuilder result = new StringBuilder("Data:\\n");
// try-with-resources automatically calls close() on the connection,
// which returns it to the HikariCP pool (does NOT close the physical DB connection)
try (Connection connection = dataSource.getConnection();
PreparedStatement stmt = connection.prepareStatement(query);
ResultSet rs = stmt.executeQuery()) {
while (rs.next()) {
result.append(rs.getString("column\_name")).append("\\n");
}
} catch (Exception e) {
context.getLogger().log("Error: " + e.getMessage());
return "Error";
}
return result.toString();
}
}`
```
**Example: Collections**
Standard Java collections are not thread safe:
```
`public class Handler implements RequestHandler&lt;Object, String&gt; {
private static List&lt;String&gt; items = new ArrayList&lt;&gt;();
private static Map&lt;String, Object&gt; cache = new HashMap&lt;&gt;();
@Override
public String handleRequest(Object input, Context context) {
items.add("list item"); // Not thread-safe
cache.put("key", input); // Not thread-safe
return "Success";
}
}`
```
Instead, use thread-safe collections:
```
`public class Handler implements RequestHandler&lt;Object, String&gt; {
private static final List&lt;String&gt; items =
Collections.synchronizedList(new ArrayList&lt;&gt;());
private static final ConcurrentHashMap&lt;String, Object&gt; cache =
new ConcurrentHashMap&lt;&gt;();
@Override
public String handleRequest(Object input, Context context) {
items.add("list item"); // Thread-safe
cache.put("key", input); // Thread-safe
return "Success";
}
}`
```
## Shared /tmp directory
The `/tmp` directory is shared across all concurrent requests in the execution environment. Concurrent writes to the same file can cause data corruption, for example if another process overwrites the file. To address this, either implement file locking for shared files or use unique file names per thread or per request to avoid conflicts. Remember to clean up unneeded files to avoid exhausting the available space.
## Logging
Log interleaving (log entries from different requests being interleaved in logs) is normal in multi-concurrent systems.
Functions using Lambda Managed Instances always use the structured JSON log format introduced with [advanced logging controls](./monitoring-logs.html#monitoring-cloudwatchlogs-advanced). This format includes the `requestId`, allowing log entries to be correlated to a single request. When you use the `LambdaLogger` object from `context.getLogger()` the `requestId` is automatically included in each log entry. For further information, see [Using Lambda advanced logging controls with Java](./java-logging.html#java-logging-advanced).
## Request context
The `context` object is bound to the request thread. Using `context.getAwsRequestId()` provides thread-safe access to the request ID for the current request.
Use `context.getXrayTraceId()` to access the X-Ray trace ID. This provides thread-safe access to the trace ID for the current request. Lambda does not support the `\_X\_AMZN\_TRACE\_ID` environment variable with Lambda Managed Instances. The X-Ray trace ID is propagated automatically when using the AWS SDK.
Use `com.amazonaws.services.lambda.runtime.Context.getRemainingTimeInMillis()` to detect timeouts. See [Error handling and recovery](./lambda-managed-instances-execution-environment.html#lambda-managed-instances-error-handling) for more information.
If you use virtual threads in your program or create threads during initialization, you will need to pass any required request context to these threads.
## Initialization and shutdown
Function initialization occurs once per execution environment. Objects created during initialization are shared across threads.
For Lambda functions with extensions, the execution environment emits a SIGTERM signal during shut down. This signal is used by extensions to trigger clean up tasks, such as flushing buffers. You can subscribe to SIGTERM events to trigger function clean-up tasks, such as closing database connections. To learn more about the execution environment lifecycle, see [Understanding the Lambda execution environment lifecycle](./lambda-runtime-environment.html).
## Dependency versions
Lambda Managed Instances requires the following minimum package versions:
* AWS SDK for Java 2.0: version 2.34.0 or later
* AWS X-Ray SDK for Java: version 2.20.0 or later
* AWS Distro for OpenTelemetry - Instrumentation for Java: version 2.20.0 or later
* Powertools for AWS Lambda (Java): version 2.8.0 or later
## Powertools for AWS Lambda (Java)
Powertools for AWS Lambda (Java) is compatible with Lambda Managed Instances and provides utilities for logging, tracing, metrics, and more. For more information, see [Powertools for AWS Lambda (Java)](https://github.com/aws-powertools/powertools-lambda-java).
## Next steps
* Review [Node.js runtime for Lambda Managed Instances](./lambda-managed-instances-nodejs-runtime.html)
* Review [Python runtime for Lambda Managed Instances](./lambda-managed-instances-python-runtime.html)
* Review [.NET runtime for Lambda Managed Instances](./lambda-managed-instances-dotnet-runtime.html)
* Learn about [scaling Lambda Managed Instances](./lambda-managed-instances-scaling.html)
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Runtimes
Node.js runtime
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.