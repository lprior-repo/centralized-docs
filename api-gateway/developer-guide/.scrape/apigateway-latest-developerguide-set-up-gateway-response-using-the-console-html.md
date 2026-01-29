---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/set-up-gateway-response-using-the-console.html
title: Set up a gateway response
word_count: 581
filtered: true
elements_removed: 0
density_score: 0.83
---

Set up a gateway response for a REST API using the API Gateway console - Amazon API Gateway
Set up a gateway response for a REST API using the API Gateway console - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#set-up-gateway-response-using-the-console)
# Set up a gateway response
for a REST API using the API Gateway console
The following example shows how to set up a gateway response for a REST API using the API Gateway console
###### To customize a gateway response using the API Gateway console
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. Choose a REST API.
3. In the main navigation pane, choose **Gateway
responses**.
4. Choose a response type, and then choose **Edit**. In
this walkthrough, we use **Missing authentication token**
as an example.
5. You can change the API Gateway-generated **Status code** to return
a different status code that meets your API's requirements. In this example, the
customization changes the status code from the default (`403`) to
`404` because this error message occurs when a client calls an
unsupported or invalid resource that can be thought of as not found.
6. To return custom headers, choose **Add response header** under
**Response headers**. For illustration purposes, we add the
following custom headers:
```
`Access-Control-Allow-Origin:'a.b.c'
x-request-id:method.request.header.x-amzn-RequestId
x-request-path:method.request.path.petId
x-request-query:method.request.querystring.q
`
```
In the preceding header mappings, a static domain name (`'a.b.c'`)
is mapped to the `Allow-Control-Allow-Origin` header to allow CORS
access to the API; the input request header of `x-amzn-RequestId` is
mapped to `request-id` in the response; the `petId` path
variable of the incoming request is mapped to the `request-path`
header in the response; and the `q` query parameter of the original
request is mapped to the `request-query` header of the
response.
7. Under **Response templates**, keep
`application/json` for **Content Type** and
enter the following body mapping template in the **Template body** editor:
```
`{
"message":"$context.error.messageString",
"type": "$context.error.responseType",
"statusCode": "'404'",
"stage": "$context.stage",
"resourcePath": "$context.resourcePath",
"stageVariables.a": "$stageVariables.a"
}`
```
This example shows how to map `$context` and
`$stageVariables` properties to properties of the gateway
response body.
8. Choose **Save changes**.
9. Deploy the API to a new or existing stage.
Test your gateway response by calling the following CURL command, assuming the corresponding API
method's invoke URL is
`https://`o81lxisefl`.execute-api.us-east-1.amazonaws.com/custErr/pets/{petId}`:
```
`curl -v -H 'x-amzn-RequestId:123344566' https://o81lxisefl.execute-api.us-east-1.amazonaws.com/custErr/pets/5/type?q=1`
```
Because the extra query string parameter `q=1` isn't compatible
with the API, an error is returned from the specified gateway response.
You should get a gateway response similar to the following:
```
`&gt;&gt; GET /custErr/pets/5?q=1 HTTP/1.1
Host: o81lxisefl.execute-api.us-east-1.amazonaws.com
User-Agent: curl/7.51.0
Accept: \*/\*
HTTP/1.1 404 Not Found
Content-Type: application/json
Content-Length: 334
Connection: keep-alive
Date: Tue, 02 May 2017 03:15:47 GMT
x-amzn-RequestId: 123344566
Access-Control-Allow-Origin: a.b.c
x-amzn-ErrorType: MissingAuthenticationTokenException
header-1: static
x-request-query: 1
x-request-path: 5
X-Cache: Error from cloudfront
Via: 1.1 441811a054e8d055b893175754efd0c3.cloudfront.net (CloudFront)
X-Amz-Cf-Id: nNDR-fX4csbRoAgtQJ16u0rTDz9FZWT-Mk93KgoxnfzDlTUh3flmzA==
{
"message":"Missing Authentication Token",
"type": MISSING\_AUTHENTICATION\_TOKEN,
"statusCode": '404',
"stage": custErr,
"resourcePath": /pets/{petId},
"stageVariables.a": a
}`
```
The preceding example assumes that the API backend is [Pet
Store](http://petstore-demo-endpoint.execute-api.com/petstore/pets) and the API has a stage variable, `a`,
defined.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Gateway responses
Set up a gateway response using
the API Gateway REST API
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.