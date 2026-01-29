---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/api-ref.html
title: API references
word_count: 805
filtered: true
elements_removed: 0
density_score: 0.85
---

API references - Amazon API Gateway
API references - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#api-ref)
[API Gateway service endpoints](#api-reference-service-endpoints)[Using IPv6 addresses in IAM policies](#api-reference-service-endpoints-dualstack-iam)
# API references
Amazon API Gateway provides APIs for creating and deploying your own HTTP and WebSocket
APIs. In addition, API Gateway APIs are available in standard AWS SDKs.
If you are using a language for which an AWS SDK exists, you may prefer to use the SDK
rather than using the API Gateway REST APIs directly. The SDKs make authentication simpler,
integrate easily with your development environment, and provide easy access to API Gateway
commands.
Here's where to find the AWS SDKs and API Gateway REST API reference documentation:
* [Tools for Amazon Web Services](https://aws.amazon.com/developer/tools/)
* [Amazon API Gateway REST API Reference](https://docs.aws.amazon.com/apigateway/latest/api/API_Operations.html)
* [Amazon API Gateway WebSocket and HTTP API Reference](https://docs.aws.amazon.com/apigatewayv2/latest/api-reference/api-reference.html)
## API Gateway service endpoints
An endpoint is a URL that serves as an entry point for an AWS web service. API Gateway supports the following endpoint types:
* [IPv4 endpoints](#api-reference-service-endpoints-ipv4)
* [Dualstack endpoints](#api-reference-service-endpoints-ipv6) (support both IPv4 and IPv6)
* [
FIPS endpoints](https://docs.aws.amazon.com/general/latest/gr/rande.html#FIPS-endpoints)
When you make a request, you can specify the endpoint to use. If you do not specify an endpoint, the IPv4
endpoint is used by default. To use a different endpoint type, you must specify it in your request. For examples
of how to do this, see [Specifying endpoints](#api-reference-service-endpoints-specify-endpoints). For a table of available endpoints, see [Amazon API Gateway endpoints](https://docs.aws.amazon.com/general/latest/gr/apigateway.html).
### IPv4 endpoints
IPv4 endpoints support IPv4 traffic only. IPv4 endpoints are available for all Regions.
If you specify the general endpoint, `apigateway.amazonaws.com`, we use the endpoint for
`us-east-1`. To use a different Region, specify its associated endpoint. For example, if you
specify `apigateway.us-east-2.amazonaws.com` as the endpoint, we direct your request to the `us-east-2` endpoint.
IPv4 endpoint names use the following naming convention:
* `apigateway.`region`.amazonaws.com`
For example, the IPv4 endpoint name for the `eu-west-1` Region is
`apigateway.eu-west-1.amazonaws.com`.
### Dualstack (IPv4 and IPv6) endpoints
Dualstack endpoints support both IPv4 and IPv6 traffic. When you make a request to a dualstack
endpoint, the endpoint URL resolves to an IPv6 or an IPv4 address, depending on the protocol used by
your network and client.
Dual-stack endpoint names use the following naming convention:
* `apigateway.`region`.api.aws`
For example, the dual-stack endpoint name for the `eu-west-1` Region is
`apigateway.eu-west-1.api.aws`.
### Specifying endpoints
The following examples show how to specify an endpoint for the
`us-east-2` Region using the AWS CLI for `apigateway`.
* **Dualstack**
```
`aws apigateway get-rest-apis --region us-east-2 --endpoint-url https://`apigateway.us-east-2.api.aws``
```
* **IPv4**
```
`aws apigateway get-rest-apis --region us-east-2 --endpoint-url https://`apigateway.us-east-2.amazonaws.com``
```
The following examples show how to specify an endpoint for the
`us-east-2` Region using the AWS CLI for `apigatewayv2`.
* **Dualstack**
```
`aws apigatewayv2 get-apis --region us-east-2 --endpoint-url https://`apigateway.us-east-2.api.aws``
```
* **IPv4**
```
`aws apigatewayv2 get-apis --region us-east-2 --endpoint-url https://`apigateway.us-east-2.amazonaws.com``
```
## Using IPv6 addresses in IAM policies
If you use any IAM user policies or API Gateway resource policies to control access to API Gateway or any API Gateway APIs,
confirm that your policies are updated to include IPv6 address ranges. Policies that aren’t updated to handle IPv6
addresses might impact client’s access to API Gateway when they start using the dualstack endpoint. For more information
about managing access permissions with IAM, see [Identity and access management for Amazon API Gateway](./security-iam.html)
IAM policies that filter IP addresses use [IP Address Condition
Operators](https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_elements.html#Conditions_IPAddress). The following identity policy allows IP addresses in the `54.240.143.\*` range to
get information about all of the resources of an HTTP or WebSocket API with the identifier of
`a123456789`. Any IP addresses
outside of this range will be denied access information about all of the resources in the API. Since all IPv6 addresses are outside of the allowed
range, this policy prevents IPv6 addresses from being able to access information about the API.
JSON
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Sid": "IPAllow",
"Effect": "Allow",
"Principal": "\*",
"Action": "apigateway:GET",
"Resource": "arn:aws:apigateway:us-east-1::/apis/a123456789/\*",
"Condition": {
"IpAddress": {"aws:SourceIp": "54.240.143.0/24"}
}
}
]
}`
`
```
You can modify the API policy's `Condition` element to allow both IPv4
(`54.240.143.0/24`) and IPv6 (`2001:DB8:1234:5678::/64`) address ranges as shown in the
following example. You can use the same type of `Condition` block shown in the example to update both
your IAM user policies and API Gateway resource policies.
```
` "Condition": {
"IpAddress": {
"aws:SourceIp": [
"54.240.143.0/24",
"2001:DB8:1234:5678::/64"
]
}
}`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Attribute-based access control
Quotas and important notes
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.