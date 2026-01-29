---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/how-to-method-settings.html
title: Methods for REST APIs in API Gateway
word_count: 812
filtered: true
elements_removed: 0
density_score: 0.82
---

Methods for REST APIs in API Gateway - Amazon API Gateway
Methods for REST APIs in API Gateway - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#how-to-method-settings)
[Prerequisites](#method-setting-prerequisites)
# Methods for REST APIs in API Gateway
In API Gateway, an API method embodies a [method
request](https://docs.aws.amazon.com/apigateway/latest/api/API_Method.html) and a [method
response](https://docs.aws.amazon.com/apigateway/latest/api/API_MethodResponse.html). You set up an API method to define what a client should or must do to
submit a request to access the service at the backend and to define the responses that the
client receives in return. For input, you can choose method request parameters, or an
applicable payload, for the client to provide the required or optional data at run time. For
output, you determine the method response status code, headers, and applicable body as
targets to map the backend response data into, before they are returned to the client. To
help the client developer understand the behaviors and the input and output formats of your
API, you can [document your API](./api-gateway-documenting-api.html) and [provide proper error messages](./api-gateway-gatewayResponse-definition.html#customize-gateway-responses) for [invalid requests](./api-gateway-method-request-validation.html).
An API method request is an HTTP request. To set up the method request, you configure an
HTTP method (or verb), the path to an API [resource](https://docs.aws.amazon.com/apigateway/latest/api/API_Resource.html), headers, applicable query string parameters. You also configure a
payload when the HTTP method is `POST`, `PUT`, or `PATCH`.
For example, to retrieve a pet using the [PetStore sample API](./api-gateway-create-api-from-example.html), you define the API method request of `GET
/pets/{petId}`, where `{petId}` is a path parameter that can take a
number at run time.
```
`GET /pets/1
Host: apigateway.us-east-1.amazonaws.com
...`
```
If the client specifies an incorrect path, for example, `/pet/1` or
`/pets/one` instead of `/pets/1`, an exception is thrown.
An API method response is an HTTP response with a given status code. For a non-proxy
integration, you must set up method responses to specify the required or optional targets of
mappings. These transform integration response headers or body to associated method response
headers or body. The mapping can be as simple as an [identity transform](https://en.wikipedia.org/wiki/Identity_transform) that
passes the headers or body through the integration as-is. For example, the following
`200` method response shows an example of passthrough of a successful
integration response as-is.
```
`200 OK
Content-Type: application/json
...
{
"id": "1",
"type": "dog",
"price": "$249.99"
}`
```
In principle, you can define a method response corresponding to a specific response from
the backend. Typically, this involves any 2XX, 4XX, and 5XX responses. However, this may not
be practical, because often you may not know in advance all the responses that a backend may
return. In practice, you can designate one method response as the default to handle the
unknown or unmapped responses from the backend. It is good practice to designate the 500
response as the default. In any case, you must set up at least one method response for
non-proxy integrations. Otherwise, API Gateway returns a 500 error response to the client even
when the request succeeds at the backend.
To support a strongly typed SDK, such as a Java SDK, for your API, you should define the
data model for input for the method request, and define the data model for output of the
method response.
## Prerequisites
Before setting up an API method, verify the following:
* You must have the method available in API Gateway. Follow the instructions in [Tutorial: Create a REST API with an HTTP non-proxy
integration](./api-gateway-create-api-step-by-step.html).
* If you want the method to communicate with a Lambda function, you must have
already created the Lambda invocation role and Lambda execution role in IAM. You
must also have created the Lambda function with which your method will
communicate in AWS Lambda. To create the roles and function, use the instructions
in [Create a Lambda function for Lambda
non-proxy integration](./getting-started-lambda-non-proxy-integration.html#getting-started-new-lambda) of the [Choose an AWS Lambda
integration tutorial](./getting-started-with-lambda-integration.html).
* If you want the method to communicate with an HTTP or HTTP proxy integration,
you must have already created, and have access to, the HTTP endpoint URL with
which your method will communicate.
* Verify that your certificates for HTTP and HTTP proxy endpoints are supported
by API Gateway. For details see [API Gateway-supported certificate authorities for HTTP and HTTP proxy integrations in API Gateway](./api-gateway-supported-certificate-authorities-for-http-endpoints.html).
###### Topics
* [Set up a method request in
API Gateway](./api-gateway-method-settings-method-request.html)
* [Set up a method response in
API Gateway](./api-gateway-method-settings-method-response.html)
* [Set up a method using the API Gateway
console](./how-to-set-up-method-using-console.html)
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Change the IP address type of a REST API
Set up method request
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.