---
url: https://docs.aws.amazon.com/lambda/latest/api/API_InvokeAsync.html
title: API InvokeAsync.html
word_count: 346
filtered: true
elements_removed: 0
density_score: 0.84
---

InvokeAsync - AWS Lambda
InvokeAsync - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/api/lambda-api.pdf#API_InvokeAsync)
[Request Syntax](#API_InvokeAsync_RequestSyntax)[URI Request Parameters](#API_InvokeAsync_RequestParameters)[Request Body](#API_InvokeAsync_RequestBody)[Response Syntax](#API_InvokeAsync_ResponseSyntax)[Response Elements](#API_InvokeAsync_ResponseElements)[Errors](#API_InvokeAsync_Errors)[See Also](#API_InvokeAsync_SeeAlso)
###### Note
For asynchronous function invocation, use [Invoke](./API_Invoke.html).
Invokes a function asynchronously.
###### Note
The payload limit is 256KB. For larger payloads, for up to 1MB, use [Invoke](./API_Invoke.html).
###### Note
If you do use the InvokeAsync action, note that it doesn't support the use of X-Ray active tracing. Trace ID is not
propagated to the function, even if X-Ray active tracing is turned on.
## URI Request Parameters
The request uses the following URI parameters.
**
[FunctionName](#API_InvokeAsync_RequestSyntax)
**
The name or ARN of the Lambda function.
###### Name formats
* **Function name** – `my-function`.
* **Function ARN** – `arn:aws:lambda:us-west-2:123456789012:function:my-function`.
* **Partial ARN** – `123456789012:function:my-function`.
The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64
characters in length.
Length Constraints: Minimum length of 1. Maximum length of 256.
Pattern: `(arn:(aws[a-zA-Z-]\*)?:lambda:)?((eusc-)?[a-z]{2}((-gov)|(-iso([a-z]?)))?-[a-z]+-\\d{1}:)?(\\d{12}:)?(function:)?([a-zA-Z0-9-\_\\.]+)(:(\\$LATEST(\\.PUBLISHED)?|[a-zA-Z0-9-\_]+))?`
Required: Yes
## Request Body
The request accepts the following binary data.
**
[InvokeArgs](#API_InvokeAsync_RequestSyntax)
**
The JSON that you want to provide to your Lambda function as input.
Required: Yes
## Response Elements
If the action is successful, the service sends back the following HTTP response.
**
[Status](#API_InvokeAsync_ResponseSyntax)
**
The status code.
## Errors
For information about the errors that are common to all actions, see [Common Errors](./CommonErrors.html).
**
InvalidRequestContentException
**
The request body could not be parsed as JSON, or a request header is invalid. For example, the 'x-amzn-RequestId'
header is not a valid UUID string.
**
message
**
The exception message.
**
Type
**
The exception type.
HTTP Status Code: 400
**
InvalidRuntimeException
**
The runtime or runtime version specified is not supported.
HTTP Status Code: 502
**
ResourceConflictException
**
The resource already exists, or another operation is in progress.
**
message
**
The exception message.
**
Type
**
The exception type.
HTTP Status Code: 409
**
ResourceNotFoundException
**
The resource specified in the request does not exist.
HTTP Status Code: 404
**
ServiceException
**
The AWS Lambda service encountered an internal error.
HTTP Status Code: 500