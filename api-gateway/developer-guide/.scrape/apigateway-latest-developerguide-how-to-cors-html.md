---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/how-to-cors.html
title: CORS for REST APIs in API Gateway
word_count: 881
filtered: true
elements_removed: 0
density_score: 0.85
---

CORS for REST APIs in API Gateway - Amazon API Gateway
CORS for REST APIs in API Gateway - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#how-to-cors)
[Determining whether to enable CORS
support](#apigateway-cors-request-types)[Enabling CORS for a simple request](#apigateway-cors-simple-request)[Enabling CORS for a non-simple request](#apigateway-enable-cors-non-simple)[Enabling CORS support for proxy integrations](#apigateway-enable-cors-proxy)
# CORS for REST APIs in API Gateway
[Cross-origin resource
sharing (CORS)](https://developer.mozilla.org/en-US/docs/Web/HTTP/CORS) is a browser security feature that restricts cross-origin HTTP
requests that are initiated from scripts running in the browser. For more information, see [What is CORS?](https://aws.amazon.com/what-is/cross-origin-resource-sharing/).
## Determining whether to enable CORS
support
A *cross-origin* HTTP request is one that is made to:
* A different *domain* (for example, from
`example.com` to `amazondomains.com`)
* A different *subdomain* (for example, from
`example.com` to `petstore.example.com`)
* A different *port* (for example, from
`example.com` to `example.com:10777`)
* A different *protocol* (for example, from
`https://example.com` to `http://example.com`)
If you cannot access your API and receive an error message that contains `Cross-Origin Request Blocked`,
you might need to enable CORS.
Cross-origin HTTP requests can be divided into two types: *simple*
requests and *non-simple* requests.
## Enabling CORS for a simple request
An HTTP request is *simple* if all of the following conditions are
true:
* It is issued against an API resource that allows only `GET`,
`HEAD`, and `POST` requests.
* If it is a `POST` method request, it must include an
`Origin` header.
* The request payload content type is `text/plain`,
`multipart/form-data`, or
`application/x-www-form-urlencoded`.
* The request does not contain custom headers.
* Any additional requirements that are listed in the [Mozilla CORS documentation for simple requests](https://developer.mozilla.org/en-US/docs/Web/HTTP/CORS#Simple_requests).
For simple cross-origin `POST` method requests, the response from your
resource needs to include the header `Access-Control-Allow-Origin: '\*'` or `Access-Control-Allow-Origin:`'origin'``.
All other cross-origin HTTP requests are *non-simple* requests.
## Enabling CORS for a non-simple request
If
your API's resources receive non-simple requests, you must enable additional CORS support depending on your integration type.
### Enabling CORS for non-proxy integrations
For these integrations, the [CORS
protocol](https://fetch.spec.whatwg.org/#http-cors-protocol) requires the browser to send a preflight request to the server and wait for approval (or a
request for credentials) from the server before sending the actual request. You must configure your API to send
an appropriate response to the preflight request.
To create a preflight response:
1. Create an `OPTIONS` method with a mock integration.
2. Add the following response headers to the 200 method response:
* `Access-Control-Allow-Headers`
* `Access-Control-Allow-Methods`
* `Access-Control-Allow-Origin`
* Set the integration passthrough behavior to `NEVER`. In this case, the method request of an
unmapped content type will be rejected with an HTTP 415 Unsupported Media Type response. For more information, see
[Method request behavior for payloads without mapping
templates for REST APIs in API Gateway](./integration-passthrough-behaviors.html).
* Enter values for the response headers. To allow all origins, all methods, and common headers, use the following header values:
* `Access-Control-Allow-Headers: 'Content-Type,X-Amz-Date,Authorization,X-Api-Key,X-Amz-Security-Token'`
* `Access-Control-Allow-Methods: 'DELETE,GET,HEAD,OPTIONS,PUT,POST,PATCH'`
* `Access-Control-Allow-Origin: '\*'`
After creating the preflight request, you must return the `Access-Control-Allow-Origin: '\*'` or
`Access-Control-Allow-Origin:`'origin'`` header for all CORS-enabled methods for at least all 200 responses.
### Enabling CORS for non-proxy integrations using the AWS Management Console
You can use the AWS Management Console to enable CORS. API Gateway creates an `OPTIONS` method and adds the
`Access-Control-Allow-Origin` header to your existing method
integration responses. This doesn’t always work, and sometimes you need to manually
modify the integration response to return the
`Access-Control-Allow-Origin` header for all CORS-enabled methods for at least all 200 responses.
If you have binary media types set to `\*/\*` for your API, when API Gateway creates an
`OPTIONS` method, change the `contentHandling`
to `CONVERT\_TO\_TEXT`.
The following
[update-integration](https://docs.aws.amazon.com/cli/latest/reference/apigateway/update-integration.html) command changes the
`contentHandling` to `CONVERT\_TO\_TEXT` for an integration request:
```
`aws apigateway update-integration \\
--rest-api-id `abc123` \\
--resource-id `aaa111` \\
--http-method OPTIONS \\
--patch-operations op='replace',path='/contentHandling',value='CONVERT\_TO\_TEXT'`
```
The following [update-integration-response](https://docs.aws.amazon.com/cli/latest/reference/apigateway/update-integration-response.html) command changes the
`contentHandling` to `CONVERT\_TO\_TEXT` for an integration response:
```
`aws apigateway update-integration-response \\
--rest-api-id `abc123` \\
--resource-id `aaa111` \\
--http-method OPTIONS \\
--status-code 200 \\
--patch-operations op='replace',path='/contentHandling',value='CONVERT\_TO\_TEXT'`
```
## Enabling CORS support for proxy integrations
For a Lambda proxy integration or HTTP proxy integration, your backend is responsible for returning the `Access-Control-Allow-Origin`,
`Access-Control-Allow-Methods`, and
`Access-Control-Allow-Headers` headers, because a proxy integration doesn't return an integration response.
The following example Lambda functions return the required CORS headers:
Node.js
```
`export const handler = async (event) =&gt; {
const response = {
statusCode: 200,
headers: {
"Access-Control-Allow-Headers" : "`Content-Type`",
"Access-Control-Allow-Origin": "`https://www.example.com`",
"Access-Control-Allow-Methods": "`OPTIONS,POST,GET`"
},
body: JSON.stringify('Hello from Lambda!'),
};
return response;
};`
```
Python 3
```
`import json
def lambda\_handler(event, context):
return {
'statusCode': 200,
'headers': {
'Access-Control-Allow-Headers': '`Content-Type`',
'Access-Control-Allow-Origin': '`https://www.example.com`',
'Access-Control-Allow-Methods': '`OPTIONS,POST,GET`'
},
'body': json.dumps('Hello from Lambda!')
}`
```
###### Topics
* [Enable CORS on a resource using the API Gateway
console](./how-to-cors-console.html)
* [Enable CORS on a
resource using the API Gateway import API](./enable-cors-for-resource-using-swagger-importer-tool.html)
* [Test CORS for an API Gateway API](./apigateway-test-cors.html)
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Gateway response types for API Gateway
Enable CORS using the console
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.