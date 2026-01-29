---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/response-transfer-mode-lambda.html
title: Set up a Lambda proxy integration with payload response streaming in API Gateway
word_count: 967
filtered: true
elements_removed: 0
density_score: 0.88
---

Set up a Lambda proxy integration with payload response streaming in API Gateway - Amazon API Gateway
Set up a Lambda proxy integration with payload response streaming in API Gateway - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#response-transfer-mode-lambda)
[Differences in Lambda proxy integrations between
stream and buffered response transfer mode](#response-transfer-mode-lambda-comparison)[Lambda proxy integration format for response streaming](#response-transfer-mode-lambda-format)
# Set up a Lambda proxy integration with payload response streaming in API Gateway
You can stream the response of a Lambda function to improve time to first byte (TTFB) performance and send
partial responses back to the client as they become available. API Gateway requires you to use the [InvokeWithResponseStream](https://docs.aws.amazon.com/lambda/latest/api/API_InvokeWithResponseStream.html) Lambda API to invoke
your Lambda function. API Gateway passes an event object to the Lambda function. The backend Lambda function parses the
incoming request data to determine the response that it returns. In order for API Gateway to stream the Lambda output, the
Lambda function must output the
[format](#response-transfer-mode-lambda-format) required by API Gateway.
## Differences in Lambda proxy integrations between
stream and buffered response transfer mode
The following list describes the differences between a Lambda proxy integration and a Lambda proxy integration for response streaming:
* API Gateway uses the
[InvokeWithResponseStream](https://docs.aws.amazon.com/lambda/latest/api/API_InvokeWithResponseStream.html) API to invoke the Lambda proxy integration for response
streaming. This results in a different URI, which is the
following:
```
`arn:aws:apigateway:`us-west-1`:lambda:path/2021-11-15/functions/arn:aws:lambda:`us-west-1`:111122223333:function:my-function-name/response-streaming-invocations`
```
This ARN uses a different date for the API version and a different service action compared to Lambda proxy integration.
If you use the API Gateway console for response streaming, the console uses the correct
URI for you.
* In a Lambda proxy integration, API Gateway sends the response to client only after it receives the full response
from Lambda. In a Lambda proxy integration for response
streaming, API Gateway begins the payload stream after it receives the valid metadata and delimiter from
Lambda.
* Lambda proxy integration for response streaming uses the same input format as the proxy integration, but it
requires a different output format.
## Lambda proxy integration format for response streaming
When API Gateway invokes a Lambda function with response streaming, the input format is the same as the input format
of a Lambda function for proxy integration. For more information, see [Input format of a
Lambda function for proxy integration](./set-up-lambda-proxy-integrations.html#api-gateway-simple-proxy-for-lambda-input-format).
When Lambda streams a response to API Gateway, the response must adhere to the following format. This format uses a
delimiter to separate the metadata JSON and the raw payload. In this case, the payload data is streamed as it is
transmitted by your streaming Lambda function:
```
`{
"headers": {"`headerName`": "`headerValue`", ...},
"multiValueHeaders": { "`headerName`": ["`headerValue`", "`headerValue2`", ...], ... },
"cookies" : ["`cookie1`", "`cookie2`"],
"statusCode": `httpStatusCode`
}&lt;DELIMITER&gt;`PAYLOAD1 | PAYLOAD2 | PAYLOAD3``
```
In the output:
* The `headers`, `multiValueHeaders`, `cookies`, and
`statusCode` keys can be
unspecified if no extra response headers are to be returned.
* The `headers` key can only contain single-value headers.
* The output expects the headers to contain either `Transfer-Encoding: chunked` or
`Content-length: `number``. If your function doesn't return either of
these headers, API Gateway appends `Transfer-Encoding: chunked` to the response header.
* The `multiValueHeaders` key can contain multi-value headers as
well as single-value headers. You can use the `multiValueHeaders`
key to specify all of your extra headers, including any single-value
ones.
* If you specify values for both `headers` and
`multiValueHeaders`, API Gateway merges them into a single list. If
the same key-value pair is specified in both, only the values from
`multiValueHeaders` will appear in the merged list.
* The metadata must be valid JSON. Only `headers`, `multiValueHeaders`,
`cookies` and the `statusCode` keys are supported.
* You must provide a delimiter after the metadata JSON. The delimiter must be 8 null bytes and it must appear within the first 16KB of stream
data.
* API Gateway doesn't require a specific format for the method response payload.
If you're using a function URL to stream your Lambda function, you must modify the input and the output of your
Lambda function to satisfy these requirements.
If your Lambda function output doesn't adhere to the requirements of this format, API Gateway might still
invoke your Lambda function. The following table shows the combinations of API integration request settings and
Lambda function code that is supported by API Gateway. This includes supported combinations for response transfer mode of
buffered.
|Response transfer mode|Function code adheres to the required format|Lambda invoke API|Supported by API Gateway|
|
Stream
|
Yes
|
[InvokeWithResponseStream](https://docs.aws.amazon.com/lambda/latest/api/API_InvokeWithResponseStream.html)
|
Yes. API Gateway streams your response.
|
|
Stream
|
No
|
[InvokeWithResponseStream](https://docs.aws.amazon.com/lambda/latest/api/API_InvokeWithResponseStream.html)
|
No. API Gateway invokes your Lambda function and return a 500 error response.
|
|
Stream
|
Yes
|
[Invoke](https://docs.aws.amazon.com/lambda/latest/api/API_Invoke.html)
|
No. API Gateway doesn't support this integration configuration.
|
|
Stream
|
No
|
[Invoke](https://docs.aws.amazon.com/lambda/latest/api/API_Invoke.html)
|
No. API Gateway doesn't support this integration configuration.
|
|
Buffered
|
Yes
|
[InvokeWithResponseStream](https://docs.aws.amazon.com/lambda/latest/api/API_InvokeWithResponseStream.html)
|
No. API Gateway doesn't support this integration configuration.
|
|
Buffered
|
No
|
[InvokeWithResponseStream](https://docs.aws.amazon.com/lambda/latest/api/API_InvokeWithResponseStream.html)
|
No. API Gateway doesn't support this integration configuration.
|
|
Buffered
|
Yes
|
[Invoke](https://docs.aws.amazon.com/lambda/latest/api/API_Invoke.html)
|
API Gateway returns the HTTP headers and status code but not the response body.
|
|
Buffered
|
No
|
[Invoke](https://docs.aws.amazon.com/lambda/latest/api/API_Invoke.html)
|
Yes. This is a Lambda proxy integration. For more information, see
[Lambda proxy integration](./set-up-lambda-proxy-integrations.html).
|
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Set up an HTTP proxy integration with payload response streaming
Configure a Lambda proxy integration with payload response streaming
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.