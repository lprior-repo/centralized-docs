---
url: https://docs.aws.amazon.com/apigateway/latest/api/API_Integration.html
title: Integration
word_count: 1089
filtered: true
elements_removed: 0
density_score: 0.82
---

Integration - Amazon API Gateway
Integration - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/api/apigw-api.pdf#API_Integration)
[Contents](#API_Integration_Contents)[See Also](#API_Integration_SeeAlso)
# Integration
Represents an `HTTP`, `HTTP\_PROXY`, `AWS`, `AWS\_PROXY`, or Mock integration.
## Contents
**
cacheKeyParameters
**
A list of request parameters whose values API Gateway caches. To be valid values for `cacheKeyParameters`, these parameters must also be specified for Method `requestParameters`.
Type: Array of strings
Required: No
**
cacheNamespace
**
Specifies a group of related cached parameters. By default, API Gateway uses the resource ID as the `cacheNamespace`. You can specify the same `cacheNamespace` across resources to return the same cached data for requests to different resources.
Type: String
Required: No
**
connectionId
**
The ID of the VpcLink used for the integration when `connectionType=VPC\_LINK` and undefined, otherwise.
Type: String
Required: No
**
connectionType
**
The type of the network connection to the integration endpoint. The valid value is `INTERNET` for connections through the public routable internet or `VPC\_LINK` for private connections between API Gateway and a network load balancer in a VPC. The default value is `INTERNET`.
Type: String
Valid Values: `INTERNET | VPC\_LINK`
Required: No
**
contentHandling
**
Specifies how to handle request payload content type conversions. Supported values are `CONVERT\_TO\_BINARY` and `CONVERT\_TO\_TEXT`, with the following behaviors:
If this property is not defined, the request payload will be passed through from the method request to integration request without modification, provided that the `passthroughBehavior` is configured to support payload pass-through.
Type: String
Valid Values: `CONVERT\_TO\_BINARY | CONVERT\_TO\_TEXT`
Required: No
**
credentials
**
Specifies the credentials required for the integration, if any. For AWS integrations, three options are available. To specify an IAM Role for API Gateway to assume, use the role's Amazon Resource Name (ARN). To require that the caller's identity be passed through from the request, specify the string `arn:aws:iam::\\\*:user/\\\*`. To use resource-based permissions on supported AWS services, specify null.
Type: String
Required: No
**
httpMethod
**
Specifies the integration's HTTP method type. For the Type property, if you specify `MOCK`, this property is optional. For Lambda integrations, you must set the integration method to `POST`. For all other types, you must specify this property.
Type: String
Required: No
**
integrationResponses
**
Specifies the integration's responses.
Type: String to [IntegrationResponse](./API_IntegrationResponse.html) object map
Required: No
**
integrationTarget
**
The ALB or NLB listener to send the request to.
Type: String
Required: No
**
passthroughBehavior
**
Specifies how the method request body of an unmapped content type will be passed through
the integration request to the back end without transformation. A content type is unmapped if
no mapping template is defined in the integration or the content type does not match any of
the mapped content types, as specified in `requestTemplates`. The valid value is one of the
following: `WHEN\_NO\_MATCH`: passes the method request body through the integration request to
the back end without transformation when the method request content type does not match any
content type associated with the mapping templates defined in the integration request.
`WHEN\_NO\_TEMPLATES`: passes the method request body through the integration request to the back
end without transformation when no mapping template is defined in the integration request. If
a template is defined when this option is selected, the method request of an unmapped
content-type will be rejected with an HTTP 415 Unsupported Media Type response. `NEVER`: rejects
the method request with an HTTP 415 Unsupported Media Type response when either the method
request content type does not match any content type associated with the mapping templates
defined in the integration request or no mapping template is defined in the integration
request.
Type: String
Required: No
**
requestParameters
**
A key-value map specifying request parameters that are passed from the method request to the back end. The key is an integration request parameter name and the associated value is a method request parameter value or static value that must be enclosed within single quotes and pre-encoded as required by the back end. The method request parameter value must match the pattern of `method.request.{location}.{name}`, where `location` is `querystring`, `path`, or `header` and `name` must be a valid and unique method request parameter name.
Type: String to string map
Required: No
**
requestTemplates
**
Represents a map of Velocity templates that are applied on the request payload based on the value of the Content-Type header sent by the client. The content type value is the key in this map, and the template (as a String) is the value.
Type: String to string map
Required: No
**
responseTransferMode
**
The response transfer mode of the integration.
Type: String
Valid Values: `BUFFERED | STREAM`
Required: No
**
timeoutInMillis
**
Custom timeout between 50 and 29,000 milliseconds. The default value is 29,000 milliseconds or 29 seconds. You can increase the default value to longer than 29 seconds for Regional or private APIs only.
Type: Integer
Required: No
**
tlsConfig
**
Specifies the TLS configuration for an integration.
Type: [TlsConfig](./API_TlsConfig.html) object
Required: No
**
type
**
Specifies an API method integration type. The valid value is one of the following:
For the HTTP and HTTP proxy integrations, each integration can specify a protocol (`http/https`), port and path. Standard 80 and 443 ports are supported as well as custom ports above 1024. An HTTP or HTTP proxy integration with a `connectionType` of `VPC\_LINK` is referred to as a private integration and uses a VpcLink to connect API Gateway to a network load balancer of a VPC.
Type: String
Valid Values: `HTTP | AWS | MOCK | HTTP\_PROXY | AWS\_PROXY`
Required: No
**
uri
**
Specifies Uniform Resource Identifier (URI) of the integration endpoint.
For `HTTP` or `HTTP\_PROXY` integrations, the URI must be a fully formed, encoded HTTP(S) URL
according to the RFC-3986 specification for standard integrations. If `connectionType` is `VPC\_LINK` specify the Network Load Balancer DNS name.
For `AWS` or `AWS\_PROXY` integrations, the URI is of
the form `arn:aws:apigateway:{region}:{subdomain.service|service}:path|action/{service\_api}`.
Here, {Region} is the API Gateway region (e.g., us-east-1); {service} is the name of the
integrated AWS service (e.g., s3); and {subdomain} is a designated subdomain supported by
certain AWS service for fast host-name lookup. action can be used for an AWS service
action-based API, using an Action={name}&amp;{p1}={v1}&amp;p2={v2}... query string. The ensuing
{service\_api} refers to a supported action {name} plus any required input parameters.
Alternatively, path can be used for an AWS service path-based API. The ensuing service\_api
refers to the path to an AWS service resource, including the region of the integrated AWS
service, if applicable. For example, for integration with the S3 API of GetObject, the uri can
be either `arn:aws:apigateway:us-west-2:s3:action/GetObject&amp;Bucket={bucket}&amp;Key={key}` or
`arn:aws:apigateway:us-west-2:s3:path/{bucket}/{key}`
Type: String
Required: No