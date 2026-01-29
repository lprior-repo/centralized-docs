---
url: https://docs.aws.amazon.com/lambda/latest/dg/snapstart-runtime-hooks-dotnet.html
title: Lambda SnapStart runtime hooks for .NET
word_count: 478
filtered: true
elements_removed: 0
density_score: 0.87
---

Lambda SnapStart runtime hooks for .NET - AWS Lambda
Lambda SnapStart runtime hooks for .NET - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#snapstart-runtime-hooks-dotnet)
[Registration and execution](#runtime-hooks-registration-dotnet)[Example](#runtime-hooks-dotnet-code-sample)
# Lambda SnapStart runtime hooks for .NET
You can use runtime hooks to implement code before Lambda creates a snapshot or after Lambda resumes a function
from a snapshot. .NET runtime hooks are available as part of the [Amazon.Lambda.Core](https://www.nuget.org/packages/Amazon.Lambda.Core) package (version 2.5.0 or later). This library provides two methods that you can use to define your runtime hooks:
* `RegisterBeforeSnapshot()`: Code to run before snapshot creation
* `RegisterAfterSnapshot()`: Code to run after resuming a function from a snapshot
###### Note
If you're using the [Lambda Annotations framework for .NET](./csharp-handler.html#csharp-handler-annotations), upgrade to [Amazon.Lambda.Annotations](https://www.nuget.org/packages/Amazon.Lambda.Annotations) version 1.6.0 or later to ensure compatibility with SnapStart.
## Runtime hook registration and execution
Register your hooks in your initialization code. Consider the following guidelines based on your Lambda function's [execution model](./csharp-handler.html#csharp-handler-setup):
* For the [executable assembly approach](./csharp-handler.html#csharp-executable-assembly-handlers), register your hooks before you start the Lambda bootstrap with `RunAsync`.
* For the [class library approach](./csharp-handler.html#csharp-class-library-handlers), register your hooks in the handler class constructor.
* For [ASP.NET Core applications](./csharp-package-asp.html), register your hooks before calling the `WebApplications.Run` method.
To register runtime hooks for SnapStart in .NET, use the following methods:
```
`Amazon.Lambda.Core.SnapshotRestore.RegisterBeforeSnapshot(BeforeCheckpoint);
Amazon.Lambda.Core.SnapshotRestore.RegisterAfterRestore(AfterCheckpoint);`
```
When multiple hook types are registered, the order that Lambda executes your runtime hooks is determined by the order of registration:
* `RegisterBeforeSnapshot()`: Executed in the reverse order of registration
* `RegisterAfterSnapshot()`: Executed in the order of registration
###### Note
When Lambda creates a snapshot, your initialization code can run for up to 15 minutes. The time limit is 130 seconds or the [configured function timeout](./configuration-timeout.html) (maximum 900 seconds), whichever is higher. Your `RegisterBeforeSnapshot()` runtime hooks count towards the initialization code time limit. When Lambda restores a snapshot, the runtime must load and `RegisterAfterSnapshot()` runtime hooks must complete within the timeout limit (10 seconds). Otherwise, you'll get a SnapStartTimeoutException.
## Example
The following example function shows how to run code before checkpointing (`RegisterBeforeSnapshot`) and
after restoring (`RegisterAfterRestore`).
```
`public class SampleClass
{
public SampleClass()
{
Amazon.Lambda.Core.SnapshotRestore.RegisterBeforeSnapshot(BeforeCheckpoint);
Amazon.Lambda.Core.SnapshotRestore.RegisterAfterRestore(AfterCheckpoint);
}
private ValueTask BeforeCheckpoint()
{
// Add logic to be executed before taking the snapshot
return ValueTask.CompletedTask;
}
private ValueTask AfterCheckpoint()
{
// Add logic to be executed after restoring the snapshot
return ValueTask.CompletedTask;
}
public APIGatewayProxyResponse FunctionHandler(APIGatewayProxyRequest request, ILambdaContext context)
{
// Add business logic
return new APIGatewayProxyResponse
{
StatusCode = 200
};
}
}`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Python
Monitoring
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.