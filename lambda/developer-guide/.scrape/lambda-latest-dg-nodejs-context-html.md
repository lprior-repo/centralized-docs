---
url: https://docs.aws.amazon.com/lambda/latest/dg/nodejs-context.html
title: Using the Lambda context object to retrieve Node.js function information
word_count: 443
filtered: true
elements_removed: 0
density_score: 0.91
---

Using the Lambda context object to retrieve Node.js function information - AWS Lambda
Using the Lambda context object to retrieve Node.js function information - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#nodejs-context)
# Using the Lambda context object to retrieve Node.js function information
When Lambda runs your function, it passes a context object to the [handler](./nodejs-handler.html).
This object provides methods and properties that provide information about the invocation, function, and execution
environment.
###### Context methods
* `getRemainingTimeInMillis()` – Returns the number of milliseconds left before the execution times out.
###### Context properties
* `functionName` – The name of the Lambda function.
* `functionVersion` – The [version](./configuration-versions.html) of the function.
* `invokedFunctionArn` – The Amazon Resource Name (ARN) that's used to invoke the function. Indicates if the invoker
specified a version number or alias.
* `memoryLimitInMB` – The amount of memory that's allocated for the function.
* `awsRequestId` – The identifier of the invocation request.
* `logGroupName` – The log group for the function.
* `logStreamName` – The log stream for the function instance.
* `identity` – (mobile apps) Information about the Amazon Cognito identity that authorized the request.
* `cognitoIdentityId` – The authenticated Amazon Cognito identity.
* `cognitoIdentityPoolId` – The Amazon Cognito identity pool that authorized the invocation.
* `clientContext` – (mobile apps) Client context that's provided to Lambda by the client application.
* `client.installation\_id`
* `client.app\_title`
* `client.app\_version\_name`
* `client.app\_version\_code`
* `client.app\_package\_name`
* `env.platform\_version`
* `env.platform`
* `env.make`
* `env.model`
* `env.locale`
* `custom` – Custom values that are set by the client application.
* `callbackWaitsForEmptyEventLoop` – By default (`true`), when using a callback-based function handler, Lambda waits for the event loop to be empty after the callback runs before ending the function invoke. Set to `false` to send the response and end the invoke immediately after the callback runs instead of waiting for the event loop to be empty. Outstanding events continue to run during the next invocation. Note that Lambda supports callback-based function handlers for Node.js 22 and earlier runtimes only.
The following example function logs context information and returns the location of the logs.
###### Example index.js file
```
`exports.handler = async function(event, context) {
console.log('Remaining time: ', context.getRemainingTimeInMillis())
console.log('Function name: ', context.functionName)
return context.logStreamName
}`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Layers
Logging
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.