---
url: https://docs.aws.amazon.com/lambda/latest/dg/services-apigateway-errors.html
title: Handling Lambda errors with an API Gateway API
word_count: 336
filtered: true
elements_removed: 0
density_score: 0.84
---

Handling Lambda errors with an API Gateway API - AWS Lambda
Handling Lambda errors with an API Gateway API - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#services-apigateway-errors)
# Handling Lambda errors with an API Gateway API
API Gateway treats all invocation and function errors as internal errors. If the Lambda API rejects the invocation
request, API Gateway returns a 500 error code. If the function runs but returns an error, or returns a response in the
wrong format, API Gateway returns a 502. In both cases, the body of the response from API Gateway is `{"message":
"Internal server error"}`.
###### Note
API Gateway does not retry any Lambda invocations. If Lambda returns an error, API Gateway returns an error response to
the client.
The following example shows an X-Ray trace map for a request that resulted in a function error and a 502 from
API Gateway. The client receives the generic error message.
![Trace map for a function error with API Gateway.](https://docs.aws.amazon.com/images/lambda/latest/dg/images/tracemap-apig-502.png)
To customize the error response, you must catch errors in your code and format a response in the required
format.
###### Example [index.mjs](https://github.com/awsdocs/aws-lambda-developer-guide/tree/main/sample-apps/nodejs-apig/function/index.mjs) – Error
formatting
```
`var formatError = function(error){
var response = {
"statusCode": error.statusCode,
"headers": {
"Content-Type": "text/plain",
"x-amzn-ErrorType": error.code
},
"isBase64Encoded": false,
"body": error.code + ": " + error.message
}
return response
}`
```
API Gateway converts this response into an HTTP error with a custom status code and body. In the trace map, the
function node is green because it handled the error.
![Trace map for a formatted error with API Gateway.](https://docs.aws.amazon.com/images/lambda/latest/dg/images/tracemap-apig-404.png)
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Tutorial
API Gateway vs function URLs
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.