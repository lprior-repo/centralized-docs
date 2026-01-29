---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/set-up-lambda-proxy-integrations.html
title: Lambda proxy integrations in
word_count: 2095
filtered: true
elements_removed: 0
density_score: 0.80
---

Lambda proxy integrations in API Gateway - Amazon API Gateway
Lambda proxy integrations in API Gateway - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#set-up-lambda-proxy-integrations)
[ Understand Lambda proxy integration ](#api-gateway-create-api-as-simple-proxy)[Support for
multi-value headers and query string parameters](#apigateway-multivalue-headers-and-parameters)[Input format of a
Lambda function for proxy integration](#api-gateway-simple-proxy-for-lambda-input-format)[Output format of
a Lambda function for proxy integration](#api-gateway-simple-proxy-for-lambda-output-format)
# Lambda proxy integrations in
API Gateway
The following section shows how to use a Lambda proxy integration.
###### Topics
* [Understand API Gateway Lambda proxy
integration](#api-gateway-create-api-as-simple-proxy)
* [Support for
multi-value headers and query string parameters](#apigateway-multivalue-headers-and-parameters)
* [Input format of a
Lambda function for proxy integration](#api-gateway-simple-proxy-for-lambda-input-format)
* [Output format of
a Lambda function for proxy integration](#api-gateway-simple-proxy-for-lambda-output-format)
* [Set up Lambda proxy
integration for API Gateway using the AWS CLI](./set-up-lambda-proxy-integration-using-cli.html)
* [Set
up a proxy resource with Lambda proxy integration with an OpenAPI definition](./api-gateway-set-up-lambda-proxy-integration-on-proxy-resource.html)
## Understand API Gateway Lambda proxy
integration
Amazon API Gateway Lambda proxy integration is a simple, powerful, and nimble mechanism to build
an API with a setup of a single API method. The Lambda proxy integration allows the client to
call a single Lambda function in the backend. The function accesses many resources or
features of other AWS services, including calling other Lambda functions.
In Lambda proxy integration, when a client submits an API request, API Gateway passes to
the integrated Lambda function an [event object](#api-gateway-simple-proxy-for-lambda-input-format), except that the order of the request parameters is not preserved. This [request data](#api-gateway-simple-proxy-for-lambda-input-format) includes the request
headers, query string parameters, URL path variables, payload, and API configuration data.
The configuration data can include current deployment stage name, stage variables, user
identity, or authorization context (if any). The backend Lambda function parses the incoming
request data to determine the response that it returns. For API Gateway to pass the Lambda output as
the API response to the client, the Lambda function must return the result in [this format](#api-gateway-simple-proxy-for-lambda-output-format).
Because API Gateway doesn't intervene very much between the client and the backend Lambda function for
the Lambda proxy integration, the client and the integrated Lambda function can
adapt to changes in each other without breaking the existing integration setup of the API.
To enable this, the client must follow application protocols enacted by the backend Lambda
function.
You can set up a Lambda proxy integration for any API method. But a Lambda proxy
integration is more potent when it is configured for an API method involving a generic proxy
resource. The generic proxy resource can be denoted by a special templated path variable of
`{proxy+}`, the catch-all `ANY` method placeholder, or both. The
client can pass the input to the backend Lambda function in the incoming request as request
parameters or applicable payload. The request parameters include headers, URL path
variables, query string parameters, and the applicable payload. The integrated Lambda
function verifies all of the input sources before processing the request and responding to
the client with meaningful error messages if any of the required input is missing.
When calling an API method integrated with the generic HTTP method of `ANY`
and the generic resource of `{proxy+}`, the client submits a request with a
particular HTTP method in place of `ANY`. The client also specifies a particular
URL path instead of `{proxy+}`, and includes any required headers, query string
parameters, or an applicable payload.
The following list summarizes runtime behaviors of different API methods with the Lambda
proxy integration:
* `ANY /{proxy+}`: The client must choose a particular HTTP method, must
set a particular resource path hierarchy, and can set any headers, query string
parameters, and applicable payload to pass the data as input to the integrated Lambda
function.
* `ANY /res`: The client must choose a particular HTTP method and can set
any headers, query string parameters, and applicable payload to pass the data as
input to the integrated Lambda function.
* `GET|POST|PUT|... /{proxy+}`: The client can set a particular resource
path hierarchy, any headers, query string parameters, and applicable payload to pass
the data as input to the integrated Lambda function.
* `GET|POST|PUT|... /res/{path}/...`: The client must choose a particular
path segment (for the `{path}` variable) and can set any request headers,
query string parameters, and applicable payload to pass input data to the integrated
Lambda function.
* `GET|POST|PUT|... /res`: The client can choose any request headers, query
string parameters, and applicable payload to pass input data to the integrated Lambda
function.
Both the proxy resource of `{proxy+}` and the custom resource of
`{custom}` are expressed as templated path variables. However
`{proxy+}` can refer to any resource along a path hierarchy, while
`{custom}` refers to a particular path segment only. For example, a grocery
store might organize its online product inventory by department names, produce categories,
and product types. The grocery store's website can then represent available products by the
following templated path variables of custom resources:
`/{department}/{produce-category}/{product-type}`. For example, apples are
represented by `/produce/fruit/apple` and carrots by
`/produce/vegetables/carrot`. It can also use `/{proxy+}` to
represent any department, any produce category, or any product type that a customer can
search for while shopping in the online store. For example, `/{proxy+}` can refer
to any of the following items:
* `/produce`
* `/produce/fruit`
* `/produce/vegetables/carrot`
To let customers search for any available product, its produce category, and the
associated store department, you can expose a single method of `GET /{proxy+}`
with read-only permissions. Similarly, to allow a supervisor to update the
`produce` department's inventory, you can set up another single method of
`PUT /produce/{proxy+}` with read/write permissions. To allow a cashier to
update the running total of a vegetable, you can set up a `POST
/produce/vegetables/{proxy+}` method with read/write permissions. To let a store
manager perform any possible action on any available product, the online store developer can
expose the `ANY /{proxy+}` method with read/write permissions. In any case, at
run time, the customer or the employee must select a particular product of a given type in a
chosen department, a specific produce category in a chosen department, or a specific
department.
For more information about setting up API Gateway proxy integrations, see [Set up a proxy integration with a proxy
resource](./api-gateway-set-up-simple-proxy.html).
Proxy integration requires that the client have more detailed knowledge of the
backend requirements. Therefore, to ensure optimal app performance and user experience, the
backend developer must communicate clearly to the client developer the requirements of the
backend, and provide a robust error feedback mechanism when the requirements are not met.
## Support for
multi-value headers and query string parameters
API Gateway supports multiple headers and query string parameters that have the same
name. Multi-value headers as well as single-value headers and parameters can be
combined in the same requests and responses. For more information, see [Input format of a
Lambda function for proxy integration](#api-gateway-simple-proxy-for-lambda-input-format) and [Output format of
a Lambda function for proxy integration](#api-gateway-simple-proxy-for-lambda-output-format).
## Input format of a
Lambda function for proxy integration
In Lambda proxy integration, API Gateway maps the entire client request to the input
`event` parameter of the backend Lambda function. The following example shows the structure of an event that API Gateway sends to a Lambda proxy integration.
In this example, we assume that the invocation to API Gateway was the following:
```
`curl 'https://a1b2c3.execute-api.us-east-1.amazonaws.com/my/path?parameter1=value1&amp;parameter2=value1&amp;parameter2=value2&amp;parameter3=value1,value2' -H 'header1: value1' -H 'header2: value1' -H 'header2: value2' -H 'header3: value1,value2'`
```
The output looks like the following:
```
`{
"resource": "/my/path",
"path": "/my/path",
"httpMethod": "GET",
"headers": {
"header1": "value1",
"header2": "value2",
"header3": "value1,value2"
},
"multiValueHeaders": {
"header1": ["value1"],
"header2": ["value1","value2"],
"header3": ["value1,value2"]
},
"queryStringParameters": {
"parameter1": "value1",
"parameter2": "value2",
"parameter3": "value1,value2"
},
"multiValueQueryStringParameters": {
"parameter1": ["value1"],
"parameter2": ["value1","value2"],
"parameter3": ["value1,value2"]
},
"requestContext": {
"accountId": "123456789012",
"apiId": "id",
"authorizer": {
"claims": null,
"scopes": null
},
"domainName": "id.execute-api.us-east-1.amazonaws.com",
"domainPrefix": "id",
"extendedRequestId": "request-id",
"httpMethod": "GET",
"identity": {
"accessKey": null,
"accountId": null,
"caller": null,
"cognitoAuthenticationProvider": null,
"cognitoAuthenticationType": null,
"cognitoIdentityId": null,
"cognitoIdentityPoolId": null,
"principalOrgId": null,
"sourceIp": "IP",
"user": null,
"userAgent": "user-agent",
"userArn": null,
"clientCert": {
"clientCertPem": "CERT\_CONTENT",
"subjectDN": "www.example.com",
"issuerDN": "Example issuer",
"serialNumber": "a1:a1:a1:a1:a1:a1:a1:a1:a1:a1:a1:a1:a1:a1:a1:a1",
"validity": {
"notBefore": "May 28 12:30:02 2019 GMT",
"notAfter": "Aug 5 09:36:04 2021 GMT"
}
}
},
"path": "/my/path",
"protocol": "HTTP/1.1",
"requestId": "id=",
"requestTime": "04/Mar/2020:19:15:17 +0000",
"requestTimeEpoch": 1583349317135,
"resourceId": null,
"resourcePath": "/my/path",
"stage": "$default"
},
"pathParameters": null,
"stageVariables": null,
"body": "Hello from Lambda!",
"isBase64Encoded": false
}`
```
###### Note
In the input:
* The `headers` key can only contain single-value
headers.
* The `multiValueHeaders` key can contain multi-value headers
as well as single-value headers.
* If you specify values for both `headers` and
`multiValueHeaders`, API Gateway merges them into a single
list. If the same key-value pair is specified in both, only the values
from `multiValueHeaders` will appear in the merged
list.
In the input to the backend Lambda function, the `requestContext` object
is a map of key-value pairs. In each pair, the key is the name of a [$context](./api-gateway-mapping-template-reference.html#context-variable-reference) variable property, and the
value is the value of that property. API Gateway might add new keys to the map.
Depending on the features that are enabled, the `requestContext` map
may vary from API to API. For example, in the preceding example, no authorization
type is specified, so no `$context.authorizer.\*` or
`$context.identity.\*` properties are present. When an authorization
type is specified, this causes API Gateway to pass authorized user information to the
integration endpoint in a `requestContext.identity` object as
follows:
* When the authorization type is `AWS\_IAM`, the authorized user
information includes `$context.identity.\*` properties.
* When the authorization type is `COGNITO\_USER\_POOLS` (Amazon Cognito
authorizer), the authorized user information includes
`$context.identity.cognito\*` and
`$context.authorizer.claims.\*` properties.
* When the authorization type is `CUSTOM` (Lambda authorizer), the
authorized user information includes
`$context.authorizer.principalId` and other applicable
`$context.authorizer.\*` properties.
## Output format of
a Lambda function for proxy integration
In Lambda proxy integration, API Gateway requires the backend Lambda function to return
output according to the following JSON format:
```
`{
"isBase64Encoded": `true|false`,
"statusCode": `httpStatusCode`,
"headers": { "`headerName`": "`headerValue`", ... },
"multiValueHeaders": { "`headerName`": ["`headerValue`", "`headerValue2`", ...], ... },
"body": "`...`"
}`
```
In the output:
* The `headers` and `multiValueHeaders` keys can be
unspecified if no extra response headers are to be returned.
* The `headers` key can only contain single-value headers.
* The `multiValueHeaders` key can contain multi-value headers as
well as single-value headers. You can use the `multiValueHeaders`
key to specify all of your extra headers, including any single-value
ones.
* If you specify values for both `headers` and
`multiValueHeaders`, API Gateway merges them into a single list. If
the same key-value pair is specified in both, only the values from
`multiValueHeaders` will appear in the merged list.
To enable CORS for the Lambda proxy integration, you must add
`Access-Control-Allow-Origin:`domain-name``
to the output `headers`.
``domain-name`` can be `\*` for
any domain name. The output `body` is marshalled to the frontend as the
method response payload. If `body` is a binary blob, you can encode it as
a Base64-encoded string by setting `isBase64Encoded` to `true`
and configuring `\*/\*` as a **Binary Media Type**.
Otherwise, you can set it to `false` or leave it unspecified.
###### Note
For more information about enabling binary support, see [Enabling binary
support using the API Gateway console](./api-gateway-payload-encodings-configure-with-console.html). For an
example Lambda function, see [Return binary media from a Lambda proxy
integration in API Gateway](./lambda-proxy-binary-media.html).
If the function output is of a different format, API Gateway returns a `502 Bad
Gateway` error response.
To return a response in a Lambda function in Node.js, you can use commands such as
the following:
* To return a successful result, call `callback(null, {"statusCode":
200, "body": "results"})`.
* To throw an exception, call `callback(new Error('internal server
error'))`.
* For a client-side error (if, for example, a required parameter is
missing), you can call `callback(null, {"statusCode": 400, "body":
"Missing parameters of ..."})` to return the error without
throwing an exception.
In a Lambda `async` function in Node.js, the equivalent syntax would be:
* To return a successful result, call `return {"statusCode": 200, "body": "results"}`.
* To throw an exception, call `throw new Error("internal server
error")`.
* For a client-side error (if, for example, a required parameter is
missing), you can call `return {"statusCode": 400, "body": "Missing parameters of ..."}` to return the error without
throwing an exception.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Lambda integration
Set up Lambda proxy
integration for API Gateway using the AWS CLI
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.