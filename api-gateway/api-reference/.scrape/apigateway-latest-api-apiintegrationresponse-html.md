---
url: https://docs.aws.amazon.com/apigateway/latest/api/API_IntegrationResponse.html
title: IntegrationResponse
word_count: 377
filtered: true
elements_removed: 0
density_score: 0.83
---

IntegrationResponse - Amazon API Gateway
IntegrationResponse - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/api/apigw-api.pdf#API_IntegrationResponse)
[Contents](#API_IntegrationResponse_Contents)[See Also](#API_IntegrationResponse_SeeAlso)
# IntegrationResponse
Represents an integration response. The status code must map to an existing MethodResponse, and parameters and templates can be used to transform the back-end response.
## Contents
**
contentHandling
**
Specifies how to handle response payload content type conversions. Supported values are `CONVERT\_TO\_BINARY` and `CONVERT\_TO\_TEXT`, with the following behaviors:
If this property is not defined, the response payload will be passed through from the integration response to the method response without modification.
Type: String
Valid Values: `CONVERT\_TO\_BINARY | CONVERT\_TO\_TEXT`
Required: No
**
responseParameters
**
A key-value map specifying response parameters that are passed to the method response from the back end.
The key is a method response header parameter name and the mapped value is an integration response header value, a static value enclosed within a pair of single quotes, or a JSON expression from the integration response body. The mapping key must match the pattern of `method.response.header.{name}`, where `name` is a valid and unique header name. The mapped non-static value must match the pattern of `integration.response.header.{name}` or `integration.response.body.{JSON-expression}`, where `name` is a valid and unique response header name and `JSON-expression` is a valid JSON expression without the `$` prefix.
Type: String to string map
Required: No
**
responseTemplates
**
Specifies the templates used to transform the integration response body. Response templates are represented as a key/value map, with a content-type as the key and a template as the value.
Type: String to string map
Required: No
**
selectionPattern
**
Specifies the regular expression (regex) pattern used to choose an integration response based on the response from the back end. For example, if the success response returns nothing and the error response returns some string, you could use the `.+` regex to match error response. However, make sure that the error response does not contain any newline (`\\n`) character in such cases. If the back end is an AWS Lambda function, the AWS Lambda function error header is matched. For all other HTTP and AWS back ends, the HTTP status code is matched.
Type: String
Required: No
**
statusCode
**
Specifies the status code that is used to map the integration response to an existing MethodResponse.
Type: String
Pattern: `[1-5]\\d\\d`
Required: No