---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/websocket-api-logging.html
title: Configure logging for WebSocket APIs in API Gateway
word_count: 1298
filtered: true
elements_removed: 0
density_score: 0.85
---

Configure logging for WebSocket APIs in API Gateway - Amazon API Gateway
Configure logging for WebSocket APIs in API Gateway - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#websocket-api-logging)
# Configure logging for WebSocket APIs in API Gateway
You can enable logging to write logs to CloudWatch Logs. There are two types of API logging in CloudWatch: execution logging and access logging. In
execution logging, API Gateway manages the CloudWatch Logs. The process includes creating log groups and
log streams, and reporting to the log streams any caller's requests and responses.
To improve your security posture, we recommend that you use execution logging at the `ERROR` or
`INFO` level. You might need to do this to comply with various compliance frameworks. For more information, see [Amazon API Gateway controls](https://docs.aws.amazon.com/securityhub/latest/userguide/apigateway-controls.html)
in the *AWS Security Hub User Guide*.
In access logging, you, as an API developer, want to log who has accessed your API and how the caller accessed
the API. You can create your own log group or choose an existing log group that could be managed by API Gateway. To
specify the access details, you select `$context`
variables (expressed in a format of your choosing) and choose a log group as the destination.
For instructions on how to set up CloudWatch logging, see [Set up CloudWatch API logging using the
API Gateway console](./set-up-logging.html#set-up-access-logging-using-console).
When you specify the **Log Format**, you can choose which context
variables to log. The following variables are supported.
|Parameter|Description|
|`$context.apiId`|
The identifier API Gateway assigns to your API.
|
|`$context.authorize.error`|The authorization error message.|
|`$context.authorize.latency`|The authorization latency in ms.|
|`$context.authorize.status`|The status code returned from an authorization attempt.|
|`$context.authorizer.error`|The error message returned from an authorizer.|
|`$context.authorizer.integrationLatency`|The Lambda authorizer latency in ms.|
|`$context.authorizer.integrationStatus`|The status code returned from a Lambda authorizer.|
|`$context.authorizer.latency`|The authorizer latency in ms.|
|`$context.authorizer.requestId`|The AWS endpoint's request ID.|
|`$context.authorizer.status`|The status code returned from an authorizer.|
|`$context.authorizer.principalId`|
The principal user identification that is associated with the
token sent by the client and returned from an API Gateway Lambda authorizer
Lambda function. (A Lambda authorizer was formerly known as a custom
authorizer.)
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
|`$context.authenticate.error`|The error message returned from an authentication attempt.|
|`$context.authenticate.latency`|The authentication latency in ms.|
|`$context.authenticate.status`|The status code returned from an authentication attempt.|
|`$context.connectedAt`|
The [Epoch](https://en.wikipedia.org/wiki/Unix_time)-formatted connection time.
|
|`$context.connectionId`|
A unique ID for the connection that can be used to make a callback to
the client.
|
|`$context.domainName`|
A domain name for the WebSocket API. This can be used to make a
callback to the client (instead of a hardcoded value).
|
|`$context.error.message`|
A string that contains an API Gateway error message.
|
|`$context.error.messageString`|The quoted value of `$context.error.message`, namely
`"$context.error.message"`.|
|`$context.error.responseType`|
The error response type.
|
|`$context.error.validationErrorString`|
A string that contains a detailed validation error message.
|
|`$context.eventType`|
The event type: `CONNECT`, `MESSAGE`, or
`DISCONNECT`.
|
|`$context.extendedRequestId`|Equivalent to `$context.requestId`.|
|`$context.identity.accountId`|
The AWS account ID associated with the request.
|
|`$context.identity.apiKey`|
The API owner key associated with key-enabled API request.
|
|`$context.identity.apiKeyId`|The API key ID associated with the key-enabled API request|
|`$context.identity.caller`|
The principal identifier of the caller that signed the request. Supported for routes that use IAM authorization.
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
|`$context.identity.principalOrgId`|
The [AWS organization ID](https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_org_details.html). Supported for routes that use IAM authorization.
|
|`$context.identity.sourceIp`|
The source IP address of the TCP connection making the request to
API Gateway.
|
|`$context.identity.user`|
The principal identifier of the user that will be authorized against resource access. Supported for routes that use IAM authorization.
|
|`$context.identity.userAgent`|
The user agent of the API caller.
|
|`$context.identity.userArn`|
The Amazon Resource Name (ARN) of the effective user identified after
authentication.
|
|`$context.integration.error`|The error message returned from an integration.|
|`$context.integration.integrationStatus`|For Lambda proxy integration, the status code returned from AWS Lambda,
not from the backend Lambda function code.|
|`$context.integration.latency`|The integration latency in ms. Equivalent to `$context.integrationLatency`.|
|`$context.integration.requestId`|The AWS endpoint's request ID. Equivalent to `$context.awsEndpointRequestId`.|
|`$context.integration.status`|The status code returned from an integration. For Lambda proxy
integrations, this is the status code that your Lambda function code
returns. Equivalent to `$context.integrationStatus`.|
|`$context.integrationLatency`|The integration latency in ms, available for access logging only.|
|`$context.messageId`|
A unique server-side ID for a message. Available only when the
`$context.eventType` is `MESSAGE`.
|
|`$context.requestId`|
Same as `$context.extendedRequestId`.
|
|`$context.requestTime`|The [CLF](https://httpd.apache.org/docs/current/logs.html#common)-formatted request time (`dd/MMM/yyyy:HH:mm:ss
+-hhmm`).|
|`$context.requestTimeEpoch`|The [Epoch](https://en.wikipedia.org/wiki/Unix_time)-formatted request time, in milliseconds.|
|`$context.routeKey`|
The selected route key.
|
|`$context.stage`|
The deployment stage of the API call (for example, beta or
prod).
|
|`$context.status`|
The response status.
|
|`$context.waf.error`|The error message returned from AWS WAF.|
|`$context.waf.latency`|The AWS WAF latency in ms.|
|`$context.waf.status`|The status code returned from AWS WAF.|
Examples of some commonly used access log formats are shown in the API Gateway console and are listed as follows.
* `CLF` ([Common Log
Format](https://httpd.apache.org/docs/current/logs.html#common)):
```
`$context.identity.sourceIp $context.identity.caller \\
$context.identity.user [$context.requestTime] "$context.eventType $context.routeKey $context.connectionId" \\
$context.status $context.requestId`
```
The continuation characters (`\\`) are meant as a visual aid. The
log format must be a single line. You can add a newline character
(`\\n`) at the end of the log format to include a newline at the
end of each log entry.
* `JSON`:
```
`{
"requestId":"$context.requestId", \\
"ip": "$context.identity.sourceIp", \\
"caller":"$context.identity.caller", \\
"user":"$context.identity.user", \\
"requestTime":"$context.requestTime", \\
"eventType":"$context.eventType", \\
"routeKey":"$context.routeKey", \\
"status":"$context.status", \\
"connectionId":"$context.connectionId"
}`
```
The continuation characters (`\\`) are meant as a visual aid. The
log format must be a single line. You can add a newline character
(`\\n`) at the end of the log format to include a newline at the
end of each log entry.
* `XML`:
```
`&lt;&lt;request id="$context.requestId"&gt;&gt; \\
&lt;&lt;ip&gt;&gt;$context.identity.sourceIp&lt;&lt;/ip&gt;&gt; \\
&lt;&lt;caller&gt;&gt;$context.identity.caller&lt;&lt;/caller&gt;&gt; \\
&lt;&lt;user&gt;&gt;$context.identity.user&lt;&lt;/user&gt;&gt; \\
&lt;&lt;requestTime&gt;&gt;$context.requestTime&lt;&lt;/requestTime&gt;&gt; \\
&lt;&lt;eventType&gt;&gt;$context.eventType&lt;&lt;/eventType&gt;&gt; \\
&lt;&lt;routeKey&gt;&gt;$context.routeKey&lt;&lt;/routeKey&gt;&gt; \\
&lt;&lt;status&gt;&gt;$context.status&lt;&lt;/status&gt;&gt; \\
&lt;&lt;connectionId&gt;&gt;$context.connectionId&lt;&lt;/connectionId&gt;&gt; \\
&lt;&lt;/request&gt;&gt;`
```
The continuation characters (`\\`) are meant as a visual aid. The
log format must be a single line. You can add a newline character
(`\\n`) at the end of the log format to include a newline at the
end of each log entry.
* `CSV` (comma-separated values):
```
`$context.identity.sourceIp,$context.identity.caller, \\
$context.identity.user,$context.requestTime,$context.eventType, \\
$context.routeKey,$context.connectionId,$context.status, \\
$context.requestId`
```
The continuation characters (`\\`) are meant as a visual aid. The
log format must be a single line. You can add a newline character
(`\\n`) at the end of the log format to include a newline at the
end of each log entry.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Metrics
API Gateway ARNs
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.