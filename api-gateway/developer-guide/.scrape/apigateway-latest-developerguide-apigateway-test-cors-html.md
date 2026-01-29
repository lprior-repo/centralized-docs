---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-test-cors.html
title: Test CORS for an API Gateway API
word_count: 216
filtered: true
elements_removed: 0
density_score: 0.89
---

Test CORS for an API Gateway API - Amazon API Gateway
Test CORS for an API Gateway API - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#apigateway-test-cors)
# Test CORS for an API Gateway API
You can test your API's CORS configuration by invoking your API, and checking the CORS
headers in the response. The following `curl` command sends an OPTIONS
request to a deployed API.
```
`curl -v -X `OPTIONS` https://`{restapi\_id}`.execute-api.`{region}`.amazonaws.com/`{stage\_name}``
```
```
`&lt;&lt; HTTP/1.1 200 OK
&lt;&lt; Date: Tue, 19 May 2020 00:55:22 GMT
&lt;&lt; Content-Type: application/json
&lt;&lt; Content-Length: 0
&lt;&lt; Connection: keep-alive
&lt;&lt; x-amzn-RequestId: a1b2c3d4-5678-90ab-cdef-abc123
&lt;&lt; Access-Control-Allow-Origin: \*
&lt;&lt; Access-Control-Allow-Headers: Content-Type,Authorization,X-Amz-Date,X-Api-Key,X-Amz-Security-Token
&lt;&lt; x-amz-apigw-id: Abcd=
&lt;&lt; Access-Control-Allow-Methods: DELETE,GET,HEAD,OPTIONS,PATCH,POST,PUT`
```
The `Access-Control-Allow-Origin`,
`Access-Control-Allow-Headers`, and
`Access-Control-Allow-Methods` headers in the response show that the API
supports CORS. For more information, see [CORS for REST APIs in API Gateway](./how-to-cors.html).
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Enable CORS using OpenAPI definition
Binary media types
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.