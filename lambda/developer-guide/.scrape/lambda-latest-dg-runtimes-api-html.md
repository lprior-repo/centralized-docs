---
url: https://docs.aws.amazon.com/lambda/latest/dg/runtimes-api.html
title: Using the Lambda runtime API for custom runtimes
word_count: 864
filtered: true
elements_removed: 0
density_score: 0.88
---

Using the Lambda runtime API for custom runtimes - AWS Lambda
Using the Lambda runtime API for custom runtimes - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#runtimes-api)
[Next invocation](#runtimes-api-next)[Invocation response](#runtimes-api-response)[Initialization error](#runtimes-api-initerror)[Invocation error](#runtimes-api-invokeerror)
# Using the Lambda runtime API for custom runtimes
AWS Lambda provides an HTTP API for [custom runtimes](./runtimes-custom.html) to receive invocation
events from Lambda and send response data back within the Lambda [execution
environment](./lambda-runtimes.html). This section contains the API reference for the Lambda runtime API.
###### Lambda Managed Instances support concurrent requests
Lambda Managed Instances use the same runtime API as Lambda (default) functions. The key difference is that
Managed Instances can accept concurrent `/next` and `/response` requests up to the
configured `AWS\_LAMBDA\_MAX\_CONCURRENCY` limit. This enables multiple invocations to be processed
simultaneously within a single execution environment. For more information about Managed Instances, see
[Understanding the Lambda Managed Instances execution environment](./lambda-managed-instances-execution-environment.html).
![Architecture diagram of the execution environment.](https://docs.aws.amazon.com/images/lambda/latest/dg/images/telemetry-api-concept-diagram.png)
The OpenAPI specification for the runtime API version **2018-06-01** is available
in [runtime-api.zip](samples/runtime-api.zip)
To create an API request URL, runtimes get the API endpoint from the `AWS\_LAMBDA\_RUNTIME\_API` environment variable, add the API version,
and add the desired resource path.
###### API methods
* [Next invocation](#runtimes-api-next)
* [Invocation response](#runtimes-api-response)
* [Initialization error](#runtimes-api-initerror)
* [Invocation error](#runtimes-api-invokeerror)
## Next invocation
**Path** – `/runtime/invocation/next`
**Method** – **GET**
The runtime sends this message to Lambda to request an invocation event. The response body contains the payload
from the invocation, which is a JSON document that contains event data from the function trigger. The response
headers contain additional data about the invocation.
## Invocation response
**Path** –
`/runtime/invocation/`AwsRequestId`/response`
**Method** – **POST**
After the function has run to completion, the runtime sends an invocation response to Lambda. For synchronous
invocations, Lambda sends the response to the client.
###### Example success request
```
`REQUEST\_ID=156cb537-e2d4-11e8-9b34-d36013741fb9
curl "http://${AWS\_LAMBDA\_RUNTIME\_API}/2018-06-01/runtime/invocation/$REQUEST\_ID/response" -d "SUCCESS"`
```
## Initialization error
If the function returns an error or the runtime encounters an error during initialization, the runtime uses
this method to report the error to Lambda.
**Path** – `/runtime/init/error`
**Method** – **POST**
**Headers**
`Lambda-Runtime-Function-Error-Type` – Error type that the runtime encountered. Required:
no.
This header consists of a string value. Lambda accepts any string, but we recommend a format of
&lt;category.reason&gt;. For example:
* Runtime.NoSuchHandler
* Runtime.APIKeyNotFound
* Runtime.ConfigInvalid
* Runtime.UnknownReason
**Body parameters**
`ErrorRequest` – Information about the error.
Required: no.
This field is a JSON object with the following structure:
```
`{
errorMessage: string (text description of the error),
errorType: string,
stackTrace: array of strings
}`
```
Note that Lambda accepts any value for `errorType`.
The following example shows a Lambda function error message in which the function could not parse the event data
provided in the invocation.
###### Example Function error
```
`{
"errorMessage" : "Error parsing event data.",
"errorType" : "InvalidEventDataException",
"stackTrace": [ ]
} `
```
###### Response body parameters
* `StatusResponse` – String. Status information, sent with 202 response codes.
* `ErrorResponse` – Additional error information, sent with the error response codes.
ErrorResponse contains an error type and an error message.
###### Response codes
* 202 – Accepted
* 403 – Forbidden
* 500 – Container error. Non-recoverable state. Runtime should exit promptly.
###### Example initialization error request
```
`ERROR="{\\"errorMessage\\" : \\"Failed to load function.\\", \\"errorType\\" : \\"InvalidFunctionException\\"}"
curl "http://${AWS\_LAMBDA\_RUNTIME\_API}/2018-06-01/runtime/init/error" -d "$ERROR" --header "Lambda-Runtime-Function-Error-Type: Unhandled"`
```
## Invocation error
If the function returns an error or the runtime encounters an error, the runtime uses this method to report
the error to Lambda.
**Path** –
`/runtime/invocation/`AwsRequestId`/error`
**Method** – **POST**
**Headers**
`Lambda-Runtime-Function-Error-Type` – Error type that the runtime encountered. Required:
no.
This header consists of a string value. Lambda accepts any string, but we recommend a format of
&lt;category.reason&gt;. For example:
* Runtime.NoSuchHandler
* Runtime.APIKeyNotFound
* Runtime.ConfigInvalid
* Runtime.UnknownReason
**Body parameters**
`ErrorRequest` – Information about the error.
Required: no.
This field is a JSON object with the following structure:
```
`{
errorMessage: string (text description of the error),
errorType: string,
stackTrace: array of strings
}`
```
Note that Lambda accepts any value for `errorType`.
The following example shows a Lambda function error message in which the function could not parse the event data
provided in the invocation.
###### Example Function error
```
`{
"errorMessage" : "Error parsing event data.",
"errorType" : "InvalidEventDataException",
"stackTrace": [ ]
} `
```
###### Response body parameters
* `StatusResponse` – String. Status information, sent with 202 response codes.
* `ErrorResponse` – Additional error information, sent with the error response codes.
ErrorResponse contains an error type and an error message.
###### Response codes
* 202 – Accepted
* 400 – Bad Request
* 403 – Forbidden
* 500 – Container error. Non-recoverable state. Runtime should exit promptly.
###### Example error request
```
`REQUEST\_ID=156cb537-e2d4-11e8-9b34-d36013741fb9
ERROR="{\\"errorMessage\\" : \\"Error parsing event data.\\", \\"errorType\\" : \\"InvalidEventDataException\\"}"
curl "http://${AWS\_LAMBDA\_RUNTIME\_API}/2018-06-01/runtime/invocation/$REQUEST\_ID/error" -d "$ERROR" --header "Lambda-Runtime-Function-Error-Type: Unhandled"`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Runtime modifications
OS-only runtimes
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.