---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-swagger-extensions-endpoint-configuration.html
title: x-amazon-apigateway-endpoint-configuration object
word_count: 324
filtered: true
elements_removed: 0
density_score: 0.85
---

x-amazon-apigateway-endpoint-configuration object - Amazon API Gateway
x-amazon-apigateway-endpoint-configuration object - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#api-gateway-swagger-extensions-endpoint-configuration)
[x-amazon-apigateway-endpoint-configuration examples](#api-gateway-swagger-extensions-endpoint-configuration-example)
# x-amazon-apigateway-endpoint-configuration object
Specifies details of the endpoint configuration for an API. This extension
is an extended property of the [OpenAPI Operation](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.0.md#operation-object) object. This object should be present in [top-level vendor extensions](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/2.0.md#specification-extensions) for Swagger 2.0. For OpenAPI 3.0, it should be
present under the vendor extensions of the [Server object](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.0.md#server-object).
|Property name|Type|Description|
|`disableExecuteApiEndpoint`|Boolean|
Specifies whether clients can invoke your API by using the default
`execute-api` endpoint. By default, clients can invoke your API with
the default `https://{api\_id}.execute-api.{region}.amazonaws.com`
endpoint. To require that clients use a custom domain name to invoke
your API, specify `true`.
|
|`vpcEndpointIds`|An array of `String`|
A list of VpcEndpoint identifiers against which to create Route53
alias records for a REST API. It is only supported for REST APIs the
`PRIVATE` endpoint type.
|
|`ipAddressType`|`string`|
The IP address types that can invoke an HTTP API. Use `ipv4` to allow IPv4 address types
to invoke an HTTP API. Use `dualstack` to allow IPv4 and IPv6 address types to invoke an
HTTP API. It is only supported for HTTP APIs.
|
## x-amazon-apigateway-endpoint-configuration examples
The following example associates specified VPC endpoints to the REST API.
```
`"x-amazon-apigateway-endpoint-configuration": {
"vpcEndpointIds": ["vpce-0212a4ababd5b8c3e", "vpce-01d622316a7df47f9"]
}
`
```
The following example disables the default endpoint for an API.
```
`"x-amazon-apigateway-endpoint-configuration": {
"disableExecuteApiEndpoint": true
}`
```
The following example sets the IP address type to dualstack for an HTTP API.
```
`"x-amazon-apigateway-endpoint-configuration": {
"ipAddressType": "dualstack"
}`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
x-amazon-apigateway-endpoint-access-mode
x-amazon-apigateway-gateway-responses
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.