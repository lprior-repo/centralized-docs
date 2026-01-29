---
url: https://docs.aws.amazon.com/lambda/latest/dg/csharp-context.html
title: Using the Lambda context object to retrieve C# function information
word_count: 437
filtered: true
elements_removed: 0
density_score: 0.82
---

Using the Lambda context object to retrieve C# function information - AWS Lambda
Using the Lambda context object to retrieve C# function information - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#csharp-context)
# Using the Lambda context object to retrieve C# function information
When Lambda runs your function, it passes a context object to the [handler](./csharp-handler.html).
This object provides properties with information about the invocation, function, and execution environment.
###### Context properties
* `FunctionName` – The name of the Lambda function.
* `FunctionVersion` – The [version](./configuration-versions.html) of the function.
* `InvokedFunctionArn` – The Amazon Resource Name (ARN) that's used to invoke the function. Indicates if the invoker
specified a version number or alias.
* `MemoryLimitInMB` – The amount of memory that's allocated for the function.
* `AwsRequestId` – The identifier of the invocation request.
* `LogGroupName` – The log group for the function.
* `LogStreamName` – The log stream for the function instance.
* `RemainingTime` (`TimeSpan`) – The number of milliseconds left before the execution times out.
* `Identity` – (mobile apps) Information about the Amazon Cognito identity that authorized the request.
* `ClientContext` – (mobile apps) Client context that's provided to Lambda by the client application.
* `Logger` The [logger object](./csharp-logging.html) for the function.
You can use information in the `ILambdaContext` object to output information about your function's invocation for monitoring
purposes. The following code provides an example of how to add context information to a structured logging framework. In this example, the
function adds `AwsRequestId` to the log outputs. The function also uses the `RemainingTime` property to cancel an
inflight task if the Lambda function timeout is about to be reached.
```
`[assembly: LambdaSerializer(typeof(Amazon.Lambda.Serialization.SystemTextJson.DefaultLambdaJsonSerializer))]
namespace GetProductHandler;
public class Function
{
private readonly IDatabaseRepository \_repo;
public Function()
{
this.\_repo = new DatabaseRepository();
}
public async Task&lt;&lt;APIGatewayProxyResponse&gt;&gt; FunctionHandler(APIGatewayProxyRequest request, ILambdaContext context)
{
Logger.AppendKey("AwsRequestId", context.AwsRequestId);
var id = request.PathParameters["id"];
using var cts = new CancellationTokenSource();
try
{
cts.CancelAfter(context.RemainingTime.Add(TimeSpan.FromSeconds(-1)));
var databaseRecord = await this.\_repo.GetById(id, cts.Token);
return new APIGatewayProxyResponse
{
StatusCode = (int)HttpStatusCode.OK,
Body = JsonSerializer.Serialize(databaseRecord)
};
}
catch (Exception ex)
{
return new APIGatewayProxyResponse
{
StatusCode = (int)HttpStatusCode.InternalServerError,
Body = JsonSerializer.Serialize(new { error = ex.Message })
};
}
finally
{
cts.Cancel();
}
}
}`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Native AOT compilation
Logging
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.