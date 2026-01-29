---
url: https://docs.aws.amazon.com/lambda/latest/dg/typescript-context.html
title: Using the Lambda context object to retrieve TypeScript function information
word_count: 524
filtered: true
elements_removed: 0
density_score: 0.91
---

Using the Lambda context object to retrieve TypeScript function information - AWS Lambda
Using the Lambda context object to retrieve TypeScript function information - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#typescript-context)
# Using the Lambda context object to retrieve TypeScript function information
When Lambda runs your function, it passes a context object to the [handler](./typescript-handler.html).
This object provides methods and properties that provide information about the invocation, function, and execution
environment.
To enable type checking for the context object, you must add the [@types/aws-lambda](https://www.npmjs.com/package/@types/aws-lambda) package as a development dependency and import the `Context` type. For more information, see [Type definitions for Lambda](./lambda-typescript.html#typescript-type-definitions).
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
* `Custom` – Custom values that are set by the client application.
* `callbackWaitsForEmptyEventLoop` – By default (`true`), when using a callback-based function handler, Lambda waits for the event loop to be empty after the callback runs before ending the function invoke. Set to `false` to send the response and end the invoke immediately after the callback runs instead of waiting for the event loop to be empty. Outstanding events continue to run during the next invocation. Note that Lambda supports callback-based function handlers for Node.js 22 and earlier runtimes only.
###### Example index.ts file
The following example function logs context information and returns the location of the logs.
###### Note
Before using this code in a Lambda function, you must add the [@types/aws-lambda](https://www.npmjs.com/package/@types/aws-lambda) package as a development dependency. This package contains the type definitions for Lambda. For more information, see [Type definitions for Lambda](./lambda-typescript.html#typescript-type-definitions).
```
`import { Context } from 'aws-lambda';
export const lambdaHandler = async (event: string, context: Context): Promise&lt;string&gt; =&gt; {
console.log('Remaining time: ', context.getRemainingTimeInMillis());
console.log('Function name: ', context.functionName);
return context.logStreamName;
};`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Deploy container images
Logging
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.