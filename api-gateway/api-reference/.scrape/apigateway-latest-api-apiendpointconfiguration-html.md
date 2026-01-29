---
url: https://docs.aws.amazon.com/apigateway/latest/api/API_EndpointConfiguration.html
title: EndpointConfiguration
word_count: 208
filtered: true
elements_removed: 0
density_score: 0.84
---

EndpointConfiguration - Amazon API Gateway
EndpointConfiguration - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/api/apigw-api.pdf#API_EndpointConfiguration)
[Contents](#API_EndpointConfiguration_Contents)[See Also](#API_EndpointConfiguration_SeeAlso)
# EndpointConfiguration
The endpoint configuration to indicate the types of endpoints an API (RestApi) or its custom domain name (DomainName) has and the IP address types that can invoke it.
## Contents
**
ipAddressType
**
The IP address types that can invoke an API (RestApi) or a DomainName. Use `ipv4` to allow only IPv4 addresses to
invoke an API or DomainName, or use `dualstack` to allow both IPv4 and IPv6 addresses to invoke an API or a DomainName. For the
`PRIVATE` endpoint type, only `dualstack` is supported.
Type: String
Valid Values: `ipv4 | dualstack`
Required: No
**
types
**
A list of endpoint types of an API (RestApi) or its custom domain name (DomainName). For an edge-optimized API and its custom domain name, the endpoint type is `"EDGE"`. For a regional API and its custom domain name, the endpoint type is `REGIONAL`. For a private API, the endpoint type is `PRIVATE`.
Type: Array of strings
Valid Values: `REGIONAL | EDGE | PRIVATE`
Required: No
**
vpcEndpointIds
**
A list of VpcEndpointIds of an API (RestApi) against which to create Route53 ALIASes. It is only supported for `PRIVATE` endpoint type.
Type: Array of strings
Required: No