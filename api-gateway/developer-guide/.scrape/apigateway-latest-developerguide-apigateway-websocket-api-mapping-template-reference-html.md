---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-websocket-api-mapping-template-reference.html
title: WebSocket API
word_count: 974
filtered: true
elements_removed: 0
density_score: 0.65
---

WebSocket API mapping template reference for API Gateway - Amazon API Gateway
WebSocket API mapping template reference for API Gateway - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#apigateway-websocket-api-mapping-template-reference)
# WebSocket API
mapping template reference for API Gateway
This section summarizes the set of variables that are currently supported for WebSocket
APIs in API Gateway.
|Parameter|Description|
|`$context.connectionId`|
A unique ID for the connection that can be used to make a callback to
the client.
|
|`$context.connectedAt`|
The [Epoch](https://en.wikipedia.org/wiki/Unix_time)-formatted connection time.
|
|`$context.domainName`|
A domain name for the WebSocket API. This can be used to make a
callback to the client (instead of a hard-coded value).
|
|`$context.eventType`|
The event type: `CONNECT`, `MESSAGE`, or
`DISCONNECT`.
|
|`$context.messageId`|
A unique server-side ID for a message. Available only when the
`$context.eventType` is `MESSAGE`.
|
|`$context.routeKey`|
The selected route key.
|
|`$context.requestId`|
Same as `$context.extendedRequestId`.
|
|`$context.extendedRequestId`|An automatically generated ID for the API call, which contains more
useful information for debugging/troubleshooting.|
|`$context.apiId`|
The identifier API Gateway assigns to your API.
|
|`$context.authorizer.principalId`|
The principal user identification associated with the token sent by
the client and returned from an API Gateway Lambda authorizer (formerly known
as a custom authorizer) Lambda function.
|
|`$context.authorizer.`property``|
The stringified value of the specified key-value pair of the `context` map returned from an API Gateway Lambda
authorizer function. For example, if the authorizer returns the
following `context` map:
```
`"context" : {
"key": "value",
"numKey": 1,
"boolKey": true
}`
```
calling `$context.authorizer.key` returns the
`"value"` string, calling
`$context.authorizer.numKey` returns the `"1"`
string, and calling `$context.authorizer.boolKey` returns the
`"true"` string.
|
|`$context.error.messageString`|The quoted value of `$context.error.message`, namely
`"$context.error.message"`.|
|`$context.error.validationErrorString`|
A string containing a detailed validation error message.
|
|`$context.identity.accountId`|
The AWS account ID associated with the request.
|
|`$context.identity.apiKey`|
The API owner key associated with key-enabled API request.
|
|`$context.identity.apiKeyId`|The API key ID associated with the key-enabled API request|
|`$context.identity.caller`|
The principal identifier of the caller making the request.
|
|`$context.identity.cognitoAuthenticationProvider`|
A comma-separated list of all the Amazon Cognito authentication providers used by the caller making the
request. Available only if the request was signed with Amazon Cognito credentials.
For example, for an identity from an Amazon Cognito user pool, `cognito-idp.
`region`.amazonaws.com/`user\_pool\_id`,cognito-idp.`region`.amazonaws.com/`user\_pool\_id`:CognitoSignIn:`token
subject claim``
For information about the available Amazon Cognito authentication providers, see [Using Federated
Identities](https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-identity.html) in the *Amazon Cognito Developer Guide*.
|
|`$context.identity.cognitoAuthenticationType`|
The Amazon Cognito authentication type of the caller making the request. Available only if the request
was signed with Amazon Cognito credentials. Possible values include `authenticated` for authenticated
identities and `unauthenticated` for unauthenticated identities.
|
|`$context.identity.cognitoIdentityId`|
The Amazon Cognito identity ID of the caller making the request.
Available only if the request was signed with Amazon Cognito
credentials.
|
|`$context.identity.cognitoIdentityPoolId`|
The Amazon Cognito identity pool ID of the caller making the request.
Available only if the request was signed with Amazon Cognito
credentials.
|
|`$context.identity.sourceIp`|
The source IP address of the immediate TCP connection making the request to API Gateway endpoint.
|
|`$context.identity.user`|
The principal identifier of the user making the request.
|
|`$context.identity.userAgent`|
The User Agent of the API caller.
|
|`$context.identity.userArn`|
The Amazon Resource Name (ARN) of the effective user identified after
authentication.
|
|`$context.requestTime`|The [CLF](https://httpd.apache.org/docs/current/logs.html#common)-formatted request time (`dd/MMM/yyyy:HH:mm:ss
+-hhmm`).|
|`$context.requestTimeEpoch`|The [Epoch](https://en.wikipedia.org/wiki/Unix_time)-formatted request time, in milliseconds.|
|`$context.stage`|
The deployment stage of the API call (for example, Beta or
Prod).
|
|`$context.status`|
The response status.
|
|`$input.body`|
Returns the raw payload as a string.
|
|`$input.json(x)`|
This function evaluates a JSONPath expression and returns the
results as a JSON string.
For example, `$input.json('$.pets')` will return a JSON
string representing the pets structure.
For more information about JSONPath, see [JSONPath](https://goessner.net/articles/JsonPath/) or
[JSONPath for
Java](https://github.com/json-path/JsonPath).
|
|`$input.path(x)`|
Takes a JSONPath expression string (`x`) and returns
a JSON object representation of the result. This allows you to
access and manipulate elements of the payload natively in [Apache Velocity Template Language (VTL)](https://velocity.apache.org/engine/devel/vtl-reference.html).
For example, if the expression `$input.path('$.pets')`
returns an object like this:
```
`[
{
"id": 1,
"type": "dog",
"price": 249.99
},
{
"id": 2,
"type": "cat",
"price": 124.99
},
{
"id": 3,
"type": "fish",
"price": 0.99
}
]`
```
`$input.path('$.pets').count()` would return
`"3"`.
For more information about JSONPath, see [JSONPath](http://goessner.net/articles/JsonPath/) or
[JSONPath for
Java](https://github.com/jayway/JsonPath).
|
|`$stageVariables.`&lt;&lt;variable\_name&gt;&gt;``|
`&lt;&lt;variable\_name&gt;&gt;` represents a stage
variable name.
|
|`$stageVariables['`&lt;&lt;variable\_name&gt;&gt;`']`|
`&lt;&lt;variable\_name&gt;&gt;` represents any stage
variable name.
|
|`${stageVariables['`&lt;&lt;variable\_name&gt;&gt;`']}`|
`&lt;&lt;variable\_name&gt;&gt;` represents any stage
variable name.
|
|`$util.escapeJavaScript()`|
Escapes the characters in a string using JavaScript string
rules.
###### Note
This function will turn any regular single quotes (`'`)
into escaped ones (`\\'`). However, the escaped single
quotes are not valid in JSON. Thus, when the output from this
function is used in a JSON property, you must turn any escaped
single quotes (`\\'`) back to regular single quotes
(`'`). This is shown in the following example:
```
` $util.escapeJavaScript(`data`).replaceAll("\\\\'","'")`
```
|
|`$util.parseJson()`|
Takes "stringified" JSON and returns an object representation of the
result. You can use the result from this function to access and
manipulate elements of the payload natively in Apache Velocity Template
Language (VTL). For example, if you have the following payload:
```
`{"errorMessage":"{\\"key1\\":\\"var1\\",\\"key2\\":{\\"arr\\":[1,2,3]}}"}`
```
and use the following mapping template
```
`#set ($errorMessageObj = $util.parseJson($input.path('$.errorMessage')))
{
"errorMessageObjKey2ArrVal" : $errorMessageObj.key2.arr[0]
}
`
```
You will get the following output:
```
`{
"errorMessageObjKey2ArrVal" : 1
}
`
```
|
|`$util.urlEncode()`|
Converts a string into "application/x-www-form-urlencoded"
format.
|
|`$util.urlDecode()`|
Decodes an "application/x-www-form-urlencoded"
string.
|
|`$util.base64Encode()`|
Encodes the data into a base64-encoded string.
|
|`$util.base64Decode()`|
Decodes the data from a base64-encoded string.
|
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Data mapping
Binary media types
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.