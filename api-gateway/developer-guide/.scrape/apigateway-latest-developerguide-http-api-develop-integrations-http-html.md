---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/http-api-develop-integrations-http.html
title: Create HTTP proxy integrations
word_count: 295
filtered: true
elements_removed: 0
density_score: 0.80
---

Create HTTP proxy integrations for HTTP APIs - Amazon API Gateway
Create HTTP proxy integrations for HTTP APIs - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#http-api-develop-integrations-http)
[HTTP proxy integration with
path variables](#http-api-develop-integrations-http-proxy)
# Create HTTP proxy integrations
for HTTP APIs
An HTTP proxy integration enables you to connect an API route to a publicly routable HTTP
endpoint. With this integration type, API Gateway passes the entire request and response between
the frontend and the backend.
To create an HTTP proxy integration, provide the URL of a publicly routable HTTP
endpoint.
## HTTP proxy integration with
path variables
You can use path variables in HTTP API routes.
For example, the route `/pets/{petID}` catches requests to
`/pets/6`. You can reference path variables in the integration URI to
send the variable's contents to an integration. An example is
`/pets/extendedpath/{petID}`.
You can use greedy path variables to catch all child resources of a route. To create a
greedy path variable, add `+` to the variable name—for example,
`{proxy+}`.
To set up a route with an HTTP proxy integration that catches all requests, create an
API route with a greedy path variable (for example, `/parent/{proxy+}`).
Integrate the route with an HTTP endpoint (for example,
`https://petstore-demo-endpoint.execute-api.com/petstore/{proxy}`) on the
`ANY` method. The greedy path variable must be at the end of the resource
path.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
AWS Lambda integrations
AWS service integrations
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.