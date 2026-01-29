---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-extensions-integration-tls-config.html
title: x-amazon-apigateway-integration.tlsConfig object
word_count: 373
filtered: true
elements_removed: 0
density_score: 0.80
---

x-amazon-apigateway-integration.tlsConfig object - Amazon API Gateway
x-amazon-apigateway-integration.tlsConfig object - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#api-gateway-extensions-integration-tls-config)
[x-amazon-apigateway-integration.tlsConfig examples](#api-gateway-extensions-integration-tls-config-example)
# x-amazon-apigateway-integration.tlsConfig object
Specifies the TLS configuration for an integration.
|Property name|Type|Description|
|`insecureSkipVerification`|`Boolean`|
Supported only for REST APIs. Specifies whether or not API Gateway skips verification that the certificate for an integration endpoint is
issued by a [supported certificate authority](https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-supported-certificate-authorities-for-http-endpoints.html). This isn’t recommended, but it enables you to
use certificates that are signed by private certificate authorities, or certificates
that are self-signed. If enabled, API Gateway still performs basic certificate
validation, which includes checking the certificate's expiration date, hostname, and
presence of a root certificate authority. The root certificate belonging to the private authority must satisfy the following constraints:
* x509 extension `keyUsage` must have `keyCertSign`.
* x509 extension `basicConstraints` must have `CA:TRUE`.
Supported only for `HTTP` and
`HTTP\_PROXY` integrations.
###### Warning
Enabling `insecureSkipVerification` isn't recommended, especially for integrations with public
HTTPS endpoints. If you enable `insecureSkipVerification`, you increase the risk of man-in-the-middle attacks.
|
|`serverNameToVerify`|`string`|
Supported only for HTTP API private integrations. If you specify a server name,
API Gateway uses it to verify the hostname on the integration's
certificate. The server name is also included in the TLS handshake
to support Server Name Indication (SNI) or virtual hosting.
|
## x-amazon-apigateway-integration.tlsConfig examples
The following OpenAPI 3.0 example enables `insecureSkipVerification` for a REST API HTTP proxy integration.
```
`"x-amazon-apigateway-integration": {
"uri": "http://petstore-demo-endpoint.execute-api.com/petstore/pets",
"responses": {
default": {
"statusCode": "200"
}
},
"passthroughBehavior": "when\_no\_match",
"httpMethod": "ANY",
"tlsConfig" : {
"insecureSkipVerification" : true
}
"type": "http\_proxy",
}`
```
The following OpenAPI 3.0 example specifies a `serverNameToVerify` for an HTTP API private integration.
```
`"x-amazon-apigateway-integration" : {
"payloadFormatVersion" : "1.0",
"connectionId" : "abc123",
"type" : "http\_proxy",
"httpMethod" : "ANY",
"uri" : "arn:aws:elasticloadbalancing:us-west-2:123456789012:listener/app/my-load-balancer/50dc6c495c0c9188/0467ef3c8400ae65",
"connectionType" : "VPC\_LINK",
"tlsConfig" : {
"serverNameToVerify" : "example.com"
}
}`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
x-amazon-apigateway-integration.responseParameters
x-amazon-apigateway-minimum-compression-size
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.