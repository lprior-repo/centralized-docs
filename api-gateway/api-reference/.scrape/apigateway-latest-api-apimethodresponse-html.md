---
url: https://docs.aws.amazon.com/apigateway/latest/api/API_MethodResponse.html
title: MethodResponse
word_count: 252
filtered: true
elements_removed: 0
density_score: 0.86
---

MethodResponse - Amazon API Gateway
MethodResponse - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/api/apigw-api.pdf#API_MethodResponse)
[Contents](#API_MethodResponse_Contents)[See Also](#API_MethodResponse_SeeAlso)
# MethodResponse
Represents a method response of a given HTTP status code returned to the client. The method response is passed from the back end through the associated integration response that can be transformed using a mapping template.
## Contents
**
responseModels
**
Specifies the Model resources used for the response's content-type. Response models are represented as a key/value map, with a content-type as the key and a Model name as the value.
Type: String to string map
Required: No
**
responseParameters
**
A key-value map specifying required or optional response parameters that API Gateway can send back to the caller. A key defines a method response header and the value specifies whether the associated method response header is required or not. The expression of the key must match the pattern `method.response.header.{name}`, where `name` is a valid and unique header name. API Gateway passes certain integration response data to the method response headers specified here according to the mapping you prescribe in the API's IntegrationResponse. The integration response data that can be mapped include an integration response header expressed in `integration.response.header.{name}`, a static value enclosed within a pair of single quotes (e.g., `'application/json'`), or a JSON expression from the back-end response payload in the form of `integration.response.body.{JSON-expression}`, where `JSON-expression` is a valid JSON expression without the `$` prefix.)
Type: String to boolean map
Required: No
**
statusCode
**
The method response's status code.
Type: String
Pattern: `[1-5]\\d\\d`
Required: No