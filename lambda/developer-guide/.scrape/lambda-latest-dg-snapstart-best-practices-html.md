---
url: https://docs.aws.amazon.com/lambda/latest/dg/snapstart-best-practices.html
title: Create S3 and SSM clients outside of Lambda handler
word_count: 1071
filtered: true
elements_removed: 0
density_score: 0.84
---

Maximize Lambda SnapStart performance - AWS Lambda
Maximize Lambda SnapStart performance - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#snapstart-best-practices)
[Performance tuning](#snapstart-tuning)[Networking best practices](#snapstart-networking)
## Performance tuning
To maximize the benefits of SnapStart, consider the following code optimization recommendations for your runtime.
###### Note
SnapStart works best when used with function invocations at scale. Functions that are invoked infrequently might not experience the same performance improvements.
To maximize the benefits of SnapStart, we recommend that you preload dependencies and initialize resources that contribute to startup
latency in your initialization code instead of in the function handler. This moves the latency associated with
heavy class loading out of the invocation path, optimizing startup performance with SnapStart.
If you can't preload dependencies or resources during initialization, then we recommend that you preload them with dummy
invocations. To do this, update the function handler code, as shown in the following example from the [pet store
function](https://github.com/awslabs/aws-serverless-java-container/tree/main/samples/spring/pet-store) on the AWS Labs GitHub repository.
```
`private static SpringLambdaContainerHandler&lt;AwsProxyRequest, AwsProxyResponse&gt; handler;
static {
try {
handler = SpringLambdaContainerHandler.getAwsProxyHandler(PetStoreSpringAppConfig.class);
// Use the onStartup method of the handler to register the custom filter
handler.onStartup(servletContext -&gt; {
FilterRegistration.Dynamic registration = servletContext.addFilter("CognitoIdentityFilter", CognitoIdentityFilter.class);
registration.addMappingForUrlPatterns(EnumSet.of(DispatcherType.REQUEST), false, "/\*");
});
// Send a fake Amazon API Gateway request to the handler to load classes ahead of time
ApiGatewayRequestIdentity identity = new ApiGatewayRequestIdentity();
identity.setApiKey("foo");
identity.setAccountId("foo");
identity.setAccessKey("foo");
AwsProxyRequestContext reqCtx = new AwsProxyRequestContext();
reqCtx.setPath("/pets");
reqCtx.setStage("default");
reqCtx.setAuthorizer(null);
reqCtx.setIdentity(identity);
AwsProxyRequest req = new AwsProxyRequest();
req.setHttpMethod("GET");
req.setPath("/pets");
req.setBody("");
req.setRequestContext(reqCtx);
Context ctx = new TestContext();
handler.proxy(req, ctx);
} catch (ContainerInitializationException e) {
// if we fail here. We re-throw the exception to force another cold start
e.printStackTrace();
throw new RuntimeException("Could not initialize Spring framework", e);
}
}`
```
To maximize the benefits of SnapStart, focus on efficient code organization and resource management within your Python functions. As a general guideline, perform heavy computational tasks during the [initialization phase](./lambda-runtime-environment.html#runtimes-lifecycle-ib). This approach moves time-consuming operations out of the invocation path, improving overall function performance. To implement this strategy effectively, we recommend the following best practices:
* Import dependencies outside of the function handler.
* Create `boto3` instances outside of the handler.
* Initialize static resources or configurations before the handler is invoked.
* Consider using a before-snapshot [runtime hook](./snapstart-runtime-hooks-python.html) for resource-intensive tasks such as downloading external files, pre-loading frameworks like Django, or loading machine learning models.
###### Example — Optimize Python function for SnapStart
```
`# Import all dependencies outside of Lambda handler
from snapshot\_restore\_py import register\_before\_snapshot
import boto3
import pandas
import pydantic
# Create S3 and SSM clients outside of Lambda handler
s3\_client = boto3.client("s3")
# Register the function to be called before snapshot
@register\_before\_snapshot
def download\_llm\_models():
# Download an object from S3 and save to tmp
# This files will persist in this snapshot
with open('/tmp/FILE\_NAME', 'wb') as f:
s3\_client.download\_fileobj('amzn-s3-demo-bucket', 'OBJECT\_NAME', f)
...
def lambda\_handler(event, context):
...`
```
To reduce just-in-time (JIT) compilation and assembly loading time, consider invoking your function handler from a `RegisterBeforeCheckpoint` [runtime hook](./snapstart-runtime-hooks-dotnet.html). Because of how .NET tiered compilation works, you’ll get optimal results by invoking the handler multiple times, as shown in the following example.
###### Important
Make sure that your dummy function invocation does not produce unintended side effects, such as initiating business transactions.
```
`public class Function
{
public Function()
{
Amazon.Lambda.Core.SnapshotRestore.RegisterBeforeSnapshot(FunctionWarmup);
}
// Warmup method that calls the function handler before snapshot to warm up the .NET code and runtime.
// This speeds up future cold starts after restoring from a snapshot.
private async ValueTask FunctionWarmup()
{
var request = new APIGatewayProxyRequest
{
Path = "/heathcheck",
HttpMethod = "GET"
};
for (var i = 0; i &lt; 10; i++)
{
await FunctionHandler(request, null);
}
}
public async Task&lt;APIGatewayProxyResponse&gt; FunctionHandler(APIGatewayProxyRequest request, ILambdaContext context)
{
//
// Process HTTP request
//
var response = new APIGatewayProxyResponse
{
StatusCode = 200
};
return await Task.FromResult(response);
}
}`
```
## Networking best practices
The state of connections that your function establishes during the initialization phase isn't guaranteed when Lambda resumes your function from a snapshot. In most cases, network connections that an AWS SDK establishes automatically resume. For other connections, we recommend the following best practices.
###### Re-establish network connections
Always re-establish your network connections when your function resumes from a snapshot. We recommend that you re-establish network connections in the function handler. Alternatively, you can use an after-restore [runtime hook](./snapstart-runtime-hooks.html).
###### Don't use hostname as a unique execution environment identifier
We recommend against using `hostname` to identify your execution environment as a unique node or
container in your applications. With SnapStart, a single snapshot is used as the initial state for multiple
execution environments. All execution environments return the same `hostname` value for
`InetAddress.getLocalHost()` (Java), `socket.gethostname()` (Python), and `Dns.GetHostName()` (.NET). For applications that require a unique execution environment identity
or `hostname` value, we recommend that you generate a unique ID in the function handler. Or, use an
after-restore [runtime hook](./snapstart-runtime-hooks.html) to generate a unique ID, and then use the unique ID as
the identifier for the execution environment.
###### Avoid binding connections to fixed source ports
We recommend that you avoid binding network connections to fixed source ports. Connections are re-established when a function resumes from a snapshot, and network connections that are bound to a fixed source port might fail.
######
Avoid using Java DNS cache
Lambda functions already cache DNS responses. If you use another DNS cache with SnapStart, then you might experience connection timeouts when the function resumes from a snapshot.
The `java.util.logging.Logger` class can indirectly enable the JVM DNS cache.
To override the default settings, set [networkaddress.cache.ttl](https://docs.oracle.com/en/java/javase/21/docs/api/java.base/java/net/InetAddress.html#inetaddress-caching-heading) to 0 before initializing `logger`.
Example:
```
`public class MyHandler {
// first set TTL property
static{
java.security.Security.setProperty("networkaddress.cache.ttl" , "0");
}
// then instantiate logger
var logger = org.apache.logging.log4j.LogManager.getLogger(MyHandler.class);
}`
```
To prevent `UnknownHostException` failures in the Java 11 runtime, we recommend setting `networkaddress.cache.negative.ttl` to 0. In Java 17 and later runtimes, this step isn't necessary. You can set this
property for a Lambda function with the `AWS\_LAMBDA\_JAVA\_NETWORKADDRESS\_CACHE\_NEGATIVE\_TTL=0` environment variable.
Disabling the JVM DNS cache does not disable Lambda's managed DNS caching.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Security model
Troubleshooting
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.