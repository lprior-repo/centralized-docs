---
url: https://docs.aws.amazon.com/step-functions/latest/dg/connect-api-gateway.html
title: Create API Gateway REST APIs with Step Functions
word_count: 1527
filtered: true
elements_removed: 0
density_score: 0.81
---

Create API Gateway REST APIs with Step Functions - AWS Step Functions
Create API Gateway REST APIs with Step Functions - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#connect-api-gateway)
[API Gateway feature support](#connect-api-gateway-support)[Request format](#connect-api-gateway-requests)[Authentication and authorization](#connect-api-gateway-auth)[Service integration patterns](#connect-api-gateway-patterns)[Output format](#connect-api-gateway-output)[Error handling](#connect-api-gateway-errors)[IAM policies](#api-gateway-iam)
# Create API Gateway REST APIs with Step Functions
Learn how to use Amazon API Gateway to create, publish, maintain, and monitor HTTP and REST APIs with Step Functions. To
integrate with API Gateway, you define a `Task` state in Step Functions that directly calls an
API Gateway HTTP or API Gateway REST endpoint, without writing code or relying on other infrastructure.
A `Task` state definition includes all the necessary information for the API
call. You can also select different authorization methods.
To learn about integrating with AWS services in Step Functions, see [Integrating services](./integrate-services.html) and [Passing parameters to a service API in Step Functions](./connect-parameters.html).
###### Key features of Optimized API Gateway integration
* `apigateway:invoke:` has no equivalent in the AWS SDK service integration. Instead, the Optimized API Gateway service calls your API Gateway endpoint directly.
## API Gateway feature support
The Step Functions API Gateway integration supports some, but not all API Gateway features. For a more
detailed list of supported features, see the following.
* Supported by both the Step Functions API Gateway REST API and API Gateway HTTP API
integrations:
* **Authorizers**: IAM (using [Signature Version 4](https://docs.aws.amazon.com/general/latest/gr/sigv4_signing.html)),
No Auth, Lambda Authorizers (request-parameter based and token-based with
custom header)
* **API types**: Regional
* **API management**: API Gateway API domain names, API
stage, Path, Query Parameters, Request Body
* Supported by the Step Functions API Gateway HTTP API integration. The Step Functions API Gateway REST API integration that provides the option for Edge-optimized APIs are not supported.
* Unsupported by the Step Functions API Gateway integration:
* **Authorizers**: Amazon Cognito, Native Open ID Connect /
OAuth 2.0, Authorization header for token-based Lambda authorizers
* **API types**: Private
* **API management**: Custom domain names
For more information about API Gateway and its HTTP and REST APIs, see the following.
* The [Amazon API Gateway
concepts](https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-basic-concept.html) page.
* [Choosing between HTTP APIs and
REST APIs](https://docs.aws.amazon.com/apigateway/latest/developerguide/http-api-vs-rest.html) in the API Gateway developer guide.
## Request format
When you create your `Task` state definition, Step Functions validates the
parameters, builds the necessary URL to perform the call, then calls the API. The
response includes the HTTP status code, headers and response body. The request format
has both required and optional parameters.
### Required request
parameters
* `ApiEndpoint`
* Type: `String`
* The hostname of an API Gateway URL. The format is
``&lt;API
ID&gt;`.execute-api.`region`.amazonaws.com`.
The API ID can only contain a combination of the following
alphanumeric characters:
`0123456789abcdefghijklmnopqrstuvwxyz`
* `Method`
* Type: `Enum`
* The HTTP method, which must be one of the following:
* `GET`
* `POST`
* `PUT`
* `DELETE`
* `PATCH`
* `HEAD`
* `OPTIONS`
### Optional request
parameters
* `Headers`
* Type: `JSON`
* HTTP headers allow a list of values associated with the same
key.
* `Stage`
* Type: `String`
* The name of the stage where the API is deployed to in API Gateway.
It's optional for any HTTP API that uses the `$default`
stage.
* `Path`
* Type: `String`
* Path parameters that are appended after the API endpoint.
* `QueryParameters`
* Type: `JSON`
* Query strings only allow a list of values associated with the same key.
* `RequestBody`
* Type: `JSON` or `String`
* The HTTP Request body. Its type can be either a `JSON`
object or `String`. `RequestBody` is only
supported for `PATCH`, `POST`, and
`PUT` HTTP methods.
* `AllowNullValues`
* Type: `BOOLEAN` – default value: `false`
* With the default setting, any **null** values in the request input state will **not** be sent to your API. In the following example, the `category` field will **not** be included in the request, unless `AllowNullValues` is set to `true` in your state machine definition.
```
`{
"NewPet": {
"type": "turtle",
"price": 123,
"category": null
}
}`
```
###### Note
By default, fields with **null** values in the request input state will **not** be sent to your API. You can force null values to be sent to your API by setting `AllowNullValues` to `true` in your state machine definition.
* `AuthType`
* Type: `JSON`
* The authentication method. The default method is
`NO\_AUTH`. The allowed values are:
* `NO\_AUTH`
* `IAM\_ROLE`
* `RESOURCE\_POLICY`
See **Authentication and authorization** for more
information.
###### Note
For security considerations, the following HTTP header keys are not currently
permitted:
* Anything prefixed with `X-Forwarded`, `X-Amz` or
`X-Amzn`.
* `Authorization`
* `Connection`
* `Content-md5`
* `Expect`
* `Host`
* `Max-Forwards`
* `Proxy-Authenticate`
* `Server`
* `TE`
* `Transfer-Encoding`
* `Trailer`
* `Upgrade`
* `Via`
* `Www-Authenticate`
The following code example shows how to invoke API Gateway using Step Functions.
```
`{
"Type": "Task",
"Resource":"arn:aws:states:::apigateway:invoke",
"Arguments": {
"ApiEndpoint": "example.execute-api.us-east-1.amazonaws.com",
"Method": "GET",
"Headers": {
"key": ["value1", "value2"]
},
"Stage": "prod",
"Path": "bills",
"QueryParameters": {
"billId": ["123456"]
},
"RequestBody": {},
"AuthType": "NO\_AUTH"
}
}
`
```
## Authentication and authorization
You can use the following authentication methods:
* **No authorization**: Call the API directly with no
authorization method.
* **IAM role**: With this method, Step Functions assumes the role of
the state machine, signs the request with [Signature Version 4](https://docs.aws.amazon.com/general/latest/gr/sigv4_signing.html) (SigV4),
then calls the API.
* **Resource policy**: Step Functions authenticates the request, and
then calls the API. You must attach a resource policy to the API which specifies
the following:
1. The state machine that will invoke API Gateway.
###### Important
You must specify your state machine to limit access to it. If you
do not, then any state machine that authenticates its API Gateway request
with **Resource policy** authentication to your API
will be granted access.
2. That Step Functions is the service calling API Gateway: `"Service":
"states.amazonaws.com"`.
3. The resource you want to access, including:
* The `region`.
* The `account-id` in the specified
region.
* The `api-id`.
* The `stage-name`.
* The `HTTP-VERB` (method).
* The `resource-path-specifier`.
For an example resource policy, see [IAM
policies for Step Functions and API Gateway](#api-gateway-iam).
For more information on the resource format, see [Resource format of permissions for executing API in API Gateway ](https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-control-access-using-iam-policies-to-invoke-api.html#api-gateway-iam-policy-resource-format-for-executing-api) in the
API Gateway Developer Guide.
###### Note
Resource policies are only supported for the REST API.
## Service integration patterns
The API Gateway integration supports two service integration patterns:
* [Request Response](./connect-to-resource.html#connect-default), which is
the default integration pattern. It lets Step Functions progress to the next step
immediately after it receives an HTTP response.
* [Wait for a Callback with Task Token](./connect-to-resource.html#connect-wait-token)
(`.waitForTaskToken`), which waits until a task token is returned
with a payload. To use the `.waitForTaskToken` pattern, append
.waitForTaskToken to the end of the **Resource** field of your
task definition as shown in the following example:
```
`{
"Type": "Task",
"Resource":"arn:aws:states:::apigateway:invoke**.waitForTaskToken**",
"Arguments": {
"ApiEndpoint": "example.execute-api.us-east-1.amazonaws.com",
"Method": "POST",
"Headers": {
"TaskToken": "{% $states.context.Task.Token %}"
},
"Stage": "prod",
"Path": "bills/add",
"QueryParameters": {},
"RequestBody": {
"billId": "my-new-bill"
},
"AuthType": "IAM\_ROLE"
}
}`
```
## Output format
The following output parameters are provided:
|Name|Type|Description|
|`ResponseBody`|`JSON` or `String`|The response body of the API call.|
|`Headers`|`JSON`|The response headers.|
|`StatusCode`|`Integer`|The HTTP status code of the response.|
|`StatusText`|`String`|The status text of the response.|
An example response:
```
`{
"ResponseBody": {
"myBills": []
},
"Headers": {
"key": ["value1", "value2"]
},
"StatusCode": 200,
"StatusText": "OK"
}`
```
## Error handling
When an error occurs, an `error` and `cause` is returned as
follows:
* If the HTTP status code is available, then the error will be returned in the
format `ApiGateway.`&lt;HTTP Status
Code&gt;``.
* If the HTTP status code is not available, then the error will be returned in
the format
`ApiGateway.`&lt;Exception&gt;``.
In both cases, the `cause` is returned as a string.
The following example shows a response where an error has occurred:
```
`{
"error": "ApiGateway.403",
"cause": "{\\"message\\":\\"Missing Authentication Token\\"}"
}`
```
###### Note
A status code of `2XX` indicates success, and no error will be
returned. All other status codes or thrown exceptions will result in an
error.
## IAM policies for calls to Amazon API Gateway
The following example templates show how AWS Step Functions generates IAM policies based on the resources in your state machine definition. For more information, see [How Step Functions generates IAM policies for integrated
services](./service-integration-iam-templates.html) and [Discover service integration patterns in Step Functions](./connect-to-resource.html).
*Resources*:
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Action": [
"execute-api:Invoke"
],
"Resource": [
"arn:aws:execute-api:`us-east-1`:`123456789012`:ENDPOINT/STAGE/GET/pets",
"arn:aws:execute-api:`us-east-1`:`123456789012`:ENDPOINT/STAGE/POST/pets"
],
"Effect": "Allow"
}
]
}`
`
```
The following code example shows a resource policy for calling API Gateway.
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Principal": {
"Service": "**states.amazonaws.com**"
},
"Action": "execute-api:Invoke",
"Resource": "arn:aws:execute-api:`us-east-1`:`123456789012`:myApi-id/`stage-name`/`HTTP-VERB`/`resource-path-specifier`",
"Condition": {
"StringEquals": {
"aws:SourceArn": [
"`&lt;SourceStateMachineArn&gt;`"
]
}
}
}
]
}`
`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Integrating optimized services
Amazon Athena
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.