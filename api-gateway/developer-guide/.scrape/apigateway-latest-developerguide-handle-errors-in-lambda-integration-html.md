---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/handle-errors-in-lambda-integration.html
title: Handle Lambda errors in API Gateway
word_count: 1331
filtered: true
elements_removed: 0
density_score: 0.80
---

Handle Lambda errors in API Gateway - Amazon API Gateway
Handle Lambda errors in API Gateway - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#handle-errors-in-lambda-integration)
[Handle standard Lambda
errors in API Gateway](#handle-standard-errors-in-lambda-integration)[Handle custom Lambda errors
in API Gateway](#handle-custom-errors-in-lambda-integration)
# Handle Lambda errors in API Gateway
For Lambda custom integrations, you must map errors returned by Lambda in the integration
response to standard HTTP error responses for your clients. Otherwise, Lambda errors are
returned as `200 OK` responses by default and the result is not intuitive for
your API users.
There are two types of errors that Lambda can return: standard errors and custom errors.
In your API, you must handle these differently.
With the Lambda proxy integration, Lambda is required to return an output of the following
format:
```
`{
"isBase64Encoded" : "boolean",
"statusCode": "number",
"headers": { ... },
"body": "JSON string"
}`
```
In this output, `statusCode` is typically `4XX` for a client error
and `5XX` for a server error. API Gateway handles these errors by mapping the Lambda
error to an HTTP error response, according to the specified `statusCode`. For
API Gateway to pass the error type (for example, `InvalidParameterException`), as part
of the response to the client, the Lambda function must include a header (for example,
`"X-Amzn-ErrorType":"InvalidParameterException"`) in the `headers`
property.
###### Topics
* [Handle standard Lambda
errors in API Gateway](#handle-standard-errors-in-lambda-integration)
* [Handle custom Lambda errors
in API Gateway](#handle-custom-errors-in-lambda-integration)
## Handle standard Lambda
errors in API Gateway
A standard AWS Lambda error has the following format:
```
`{
"errorMessage": "&lt;replaceable&gt;string&lt;/replaceable&gt;",
"errorType": "&lt;replaceable&gt;string&lt;/replaceable&gt;",
"stackTrace": [
"&lt;replaceable&gt;string&lt;/replaceable&gt;",
...
]
}`
```
Here, `errorMessage` is a string expression of the error. The
`errorType` is a language-dependent error or exception type. The
`stackTrace` is a list of string expressions showing the stack trace
leading to the occurrence of the error.
For example, consider the following JavaScript (Node.js) Lambda function.
```
`export const handler = function(event, context, callback) {
callback(new Error("Malformed input ..."));
};`
```
This function returns the following standard Lambda error, containing `Malformed
input ...` as the error message:
```
`{
"errorMessage": "Malformed input ...",
"errorType": "Error",
"stackTrace": [
"export const handler (/var/task/index.js:3:14)"
]
}`
```
Similarly, consider the following Python Lambda function, which raises an
`Exception` with the same `Malformed input ...` error message.
```
`def lambda\_handler(event, context):
raise Exception('Malformed input ...')`
```
This function returns the following standard Lambda error:
```
`{
"stackTrace": [
[
"/var/task/lambda\_function.py",
3,
"lambda\_handler",
"raise Exception('Malformed input ...')"
]
],
"errorType": "Exception",
"errorMessage": "Malformed input ..."
}`
```
Note that the `errorType` and `stackTrace` property values are
language-dependent. The standard error also applies to any error object that is an
extension of the `Error` object or a subclass of the `Exception`
class.
To map the standard Lambda error to a method response, you must first decide on an
HTTP status code for a given Lambda error. You then set a regular expression pattern on
the `[selectionPattern](https://docs.aws.amazon.com/apigateway/latest/api/API_IntegrationResponse.html#selectionPattern)` property of the [IntegrationResponse](https://docs.aws.amazon.com/apigateway/latest/api/API_IntegrationResponse.html)
associated with the given HTTP status code. In the API Gateway console, this
`selectionPattern` is denoted as **Lambda error regex**
in the **Integration response** section, under each integration response.
###### Note
API Gateway uses Java pattern-style regexes for response mapping. For more information, see [Pattern](https://docs.oracle.com/javase/8/docs/api/java/util/regex/Pattern.html) in the Oracle documentation.
For example, use the following [put-integration-response](https://docs.aws.amazon.com/cli/latest/reference/apigateway/put-integration-response.html) to set up a
new `selectionPattern` expression:
```
`aws apigateway put-integration-response --rest-api-id z0vprf0mdh --resource-id x3o5ih --http-method GET --status-code 400 --selection-pattern "Malformed.\*" --region us-west-2`
```
Make sure that you also set up the corresponding error code (`400`) on the
[method response](./api-gateway-method-settings-method-response.html#setup-method-response-status-code). Otherwise,
API Gateway throws an invalid configuration error response at runtime.
###### Note
At runtime, API Gateway matches the Lambda error's `errorMessage` against the
pattern of the regular expression on the `selectionPattern` property. If
there is a match, API Gateway returns the Lambda error as an HTTP response of the
corresponding HTTP status code. If there is no match, API Gateway returns the error as a
default response or throws an invalid configuration exception if no default response
is configured.
Setting the `selectionPattern` value to `.\*` for a given response amounts to
resetting this response as the default response. This is because such a selection pattern will match all error
messages, including null, i.e., any unspecified error message. The resulting mapping overrides the default
mapping. If you use
`.+` as the selection pattern to filter responses, it might not match a response containing
be aware that it may not match a response containing a newline ('`\\n`) character.
To update an existing `selectionPattern` value using the AWS CLI,
call the [update-integration-response](https://docs.aws.amazon.com/cli/latest/reference/apigateway/update-integration-response.html) operation to replace the
`/selectionPattern` path value with the specified regex expression of the
`Malformed\*` pattern.
To set the `selectionPattern` expression using the API Gateway console, enter the
expression in the **Lambda error regex** text box when setting up or
updating an integration response of a specified HTTP status code.
## Handle custom Lambda errors
in API Gateway
Instead of the standard error described in the preceding section, AWS Lambda allows
you to return a custom error object as JSON string. The error can be any valid JSON
object. For example, the following JavaScript (Node.js) Lambda function
returns a custom error:
```
`export const handler = (event, context, callback) =&gt; {
...
// Error caught here:
var myErrorObj = {
errorType : "InternalServerError",
httpStatus : 500,
requestId : context.awsRequestId,
trace : {
"function": "abc()",
"line": 123,
"file": "abc.js"
}
}
callback(JSON.stringify(myErrorObj));
};`
```
You must turn the `myErrorObj` object into a JSON string before calling
`callback` to exit the function. Otherwise, the `myErrorObj`
is returned as a string of `"[object Object]"`. When a method of your API is
integrated with the preceding Lambda function, API Gateway receives an integration response
with the following payload:
```
`{
"errorMessage": "{\\"errorType\\":\\"InternalServerError\\",\\"httpStatus\\":500,\\"requestId\\":\\"e5849002-39a0-11e7-a419-5bb5807c9fb2\\",\\"trace\\":{\\"function\\":\\"abc()\\",\\"line\\":123,\\"file\\":\\"abc.js\\"}}"
}`
```
As with any integration response, you can pass through this error response as-is to
the method response. Or you can have a mapping template to transform the payload
into a different format. For example, consider the following body-mapping template for a
method response of `500` status code:
```
`{
errorMessage: $input.path('$.errorMessage');
}`
```
This template translates the integration response body that contains the custom error
JSON string to the following method response body. This method response body contains
the custom error JSON object:
```
`{
"errorMessage" : {
errorType : "InternalServerError",
httpStatus : 500,
requestId : context.awsRequestId,
trace : {
"function": "abc()",
"line": 123,
"file": "abc.js"
}
}
};`
```
Depending on your API requirements, you may need to pass some or all of the custom
error properties as method response header parameters. You can achieve this by applying
the custom error mappings from the integration response body to the method response
headers.
For example, the following OpenAPI extension defines a mapping from the
`errorMessage.errorType`, `errorMessage.httpStatus`,
`errorMessage.trace.function`, and `errorMessage.trace`
properties to the `error\_type`, `error\_status`,
`error\_trace\_function`, and `error\_trace` headers,
respectively.
```
`"x-amazon-apigateway-integration": {
"responses": {
"default": {
"statusCode": "200",
"responseParameters": {
"method.response.header.error\_trace\_function": "integration.response.body.errorMessage.trace.function",
"method.response.header.error\_status": "integration.response.body.errorMessage.httpStatus",
"method.response.header.error\_type": "integration.response.body.errorMessage.errorType",
"method.response.header.error\_trace": "integration.response.body.errorMessage.trace"
},
...
}
}
}`
```
At runtime, API Gateway deserializes the `integration.response.body` parameter
when performing header mappings. However, this deserialization applies only to
body-to-header mappings for Lambda custom error responses and does not apply to
body-to-body mappings using `$input.body`. With these
custom-error-body-to-header mappings, the client receives the following headers as part
of the method response, provided that the `error\_status`,
`error\_trace`, `error\_trace\_function`, and
`error\_type` headers are declared in the method request.
```
`"error\_status":"500",
"error\_trace":"{\\"function\\":\\"abc()\\",\\"line\\":123,\\"file\\":\\"abc.js\\"}",
"error\_trace\_function":"abc()",
"error\_type":"InternalServerError"`
```
The `errorMessage.trace` property of the integration response body is a
complex property. It is mapped to the `error\_trace` header as a JSON string.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Set up asynchronous invocation of the
backend Lambda function
HTTP integration
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.