---
url: https://docs.aws.amazon.com/apigateway/latest/api/API_RestApi.html
title: API RestApi.html
word_count: 494
filtered: true
elements_removed: 0
density_score: 0.79
---

RestApi - Amazon API Gateway
RestApi - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/api/apigw-api.pdf#API_RestApi)
[Contents](#API_RestApi_Contents)[See Also](#API_RestApi_SeeAlso)
## Contents
**
apiKeySource
**
The source of the API key for metering requests according to a usage plan. Valid values
are: &gt;`HEADER` to read the API key from the `X-API-Key` header of a
request. `AUTHORIZER` to read the API key from the `UsageIdentifierKey`
from a custom authorizer.
Type: String
Valid Values: `HEADER | AUTHORIZER`
Required: No
**
apiStatus
**
The ApiStatus of the RestApi.
Type: String
Valid Values: `UPDATING | AVAILABLE | PENDING | FAILED`
Required: No
**
apiStatusMessage
**
The status message of the RestApi. When the status message is `UPDATING` you can still invoke it.
Type: String
Required: No
**
binaryMediaTypes
**
The list of binary media types supported by the RestApi. By default, the RestApi supports only UTF-8-encoded text payloads.
Type: Array of strings
Required: No
**
createdDate
**
The timestamp when the API was created.
Type: Timestamp
Required: No
**
description
**
The API's description.
Type: String
Required: No
**
disableExecuteApiEndpoint
**
Specifies whether clients can invoke your API by using the default `execute-api` endpoint.
By default, clients can invoke your API with the default
`https://{api\_id}.execute-api.{region}.amazonaws.com` endpoint. To require that clients use a
custom domain name to invoke your API, disable the default endpoint.
Type: Boolean
Required: No
**
endpointAccessMode
**
The endpoint access mode of the RestApi.
Type: String
Valid Values: `BASIC | STRICT`
Required: No
**
endpointConfiguration
**
The endpoint configuration of this RestApi showing the endpoint types and IP address types of the API.
Type: [EndpointConfiguration](./API_EndpointConfiguration.html) object
Required: No
**
id
**
The API's identifier. This identifier is unique across all of your APIs in API Gateway.
Type: String
Required: No
**
minimumCompressionSize
**
A nullable integer that is used to enable compression (with non-negative between 0 and 10485760 (10M) bytes, inclusive) or disable compression (with a null value) on an API. When compression is enabled, compression or decompression is not applied on the payload if the payload size is smaller than this value. Setting it to zero allows compression for any payload size.
Type: Integer
Required: No
**
name
**
The API's name.
Type: String
Required: No
**
policy
**
A stringified JSON policy document that applies to this RestApi regardless of the caller and Method configuration.
Type: String
Required: No
**
rootResourceId
**
The API's root resource ID.
Type: String
Required: No
**
securityPolicy
**
The Transport Layer Security (TLS) version + cipher suite for this RestApi.
Type: String
Valid Values: `TLS\_1\_0 | TLS\_1\_2 | SecurityPolicy\_TLS13\_1\_3\_2025\_09 | SecurityPolicy\_TLS13\_1\_3\_FIPS\_2025\_09 | SecurityPolicy\_TLS13\_1\_2\_PFS\_PQ\_2025\_09 | SecurityPolicy\_TLS13\_1\_2\_FIPS\_PQ\_2025\_09 | SecurityPolicy\_TLS13\_1\_2\_PQ\_2025\_09 | SecurityPolicy\_TLS13\_1\_2\_2021\_06 | SecurityPolicy\_TLS13\_2025\_EDGE | SecurityPolicy\_TLS12\_PFS\_2025\_EDGE | SecurityPolicy\_TLS12\_2018\_EDGE`
Required: No
**
tags
**
The collection of tags. Each tag element is associated with a given resource.
Type: String to string map
Required: No
**
version
**
A version identifier for the API.
Type: String
Required: No
**
warnings
**
The warning messages reported when `failonwarnings` is turned on during API import.
Type: Array of strings
Required: No