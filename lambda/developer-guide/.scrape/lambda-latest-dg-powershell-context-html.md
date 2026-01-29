---
url: https://docs.aws.amazon.com/lambda/latest/dg/powershell-context.html
title: Using the Lambda context object to retrieve PowerShell function information
word_count: 324
filtered: true
elements_removed: 0
density_score: 0.90
---

Using the Lambda context object to retrieve PowerShell function information - AWS Lambda
Using the Lambda context object to retrieve PowerShell function information - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#powershell-context)
# Using the Lambda context object to retrieve PowerShell function information
When Lambda runs your function, it passes context information by making a `$LambdaContext` variable
available to the [handler](./powershell-handler.html). This variable provides methods and properties
with information about the invocation, function, and execution environment.
###### Context properties
* `FunctionName` – The name of the Lambda function.
* `FunctionVersion` – The [version](./configuration-versions.html) of the function.
* `InvokedFunctionArn` – The Amazon Resource Name (ARN) that's used to invoke the function. Indicates if the invoker
specified a version number or alias.
* `MemoryLimitInMB` – The amount of memory that's allocated for the function.
* `AwsRequestId` – The identifier of the invocation request.
* `LogGroupName` – The log group for the function.
* `LogStreamName` – The log stream for the function instance.
* `RemainingTime` – The number of milliseconds left before the execution times out.
* `Identity` – (mobile apps) Information about the Amazon Cognito identity that authorized the request.
* `ClientContext` – (mobile apps) Client context that's provided to Lambda by the client application.
* `Logger` – The [logger object](./powershell-logging.html) for the
function.
The following PowerShell code snippet shows a simple handler function that prints some of the context
information.
```
`#Requires -Modules @{ModuleName='AWSPowerShell.NetCore';ModuleVersion='3.3.618.0'}
Write-Host 'Function name:' $LambdaContext.FunctionName
Write-Host 'Remaining milliseconds:' $LambdaContext.RemainingTime.TotalMilliseconds
Write-Host 'Log group name:' $LambdaContext.LogGroupName
Write-Host 'Log stream name:' $LambdaContext.LogStreamName`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Handler
Logging
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.