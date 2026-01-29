---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/http-api-parameter-mapping.html
title: Transform API requests and responses for HTTP APIs in API Gateway
word_count: 846
filtered: true
elements_removed: 0
density_score: 0.89
---

Transform API requests and responses for HTTP APIs in API Gateway - Amazon API Gateway
Transform API requests and responses for HTTP APIs in API Gateway - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#http-api-parameter-mapping)
[Transforming API requests](#http-api-mapping-request-parameters)[Transforming API responses](#http-api-mapping-response-parameters)[Reserved headers](#http-api-mapping-reserved-headers)[Examples](#http-api-parameter-mapping-examples)
# Transform API requests and responses for HTTP APIs in API Gateway
You can modify API requests from clients before they reach your backend integrations. You can also change the
response from integrations before API Gateway returns the response to clients. You use *parameter
mapping* to modify API requests and responses for HTTP APIs. To use parameter mapping, you specify
API request or response parameters to modify, and specify how to modify those parameters.
## Transforming API requests
You use request parameters to change requests before they reach your backend integrations. You can modify
headers, query strings, or the request path.
Request parameters are a key-value map. The key identifies the location of the request parameter to change,
and how to change it. The value specifies the new data for the parameter.
The following table shows supported keys.
|Type|Syntax|
|Header|`append|overwrite|remove:header.`headername``|
|Query string|`append|overwrite|remove:querystring.`querystring-name``|
|Path|`overwrite:path`|
The following table shows supported values that you can map to parameters.
|Type|Syntax|Notes|
|Header value|$request.header.`name` or ${request.header.`name`}|Header names are case-insensitive. API Gateway combines multiple header values with commas, for example
`"header1": "value1,value2"`. Some headers are reserved. To learn more, see [Reserved headers](#http-api-mapping-reserved-headers).|
|Query string value|$request.querystring.`name` or ${request.querystring.`name`}|Query string names are case-sensitive. API Gateway combines multiple values with commas, for example
`"querystring1" "Value1,Value2"`.|
|Request body|$request.body.`name` or ${request.body.`name`}|A JSON path expression. Recursive descent (`$request.body..name)` and filter expressions
(`?(expression)`) aren't supported.
###### Note
When you specify a JSON path, API Gateway truncates the request body at 100 KB and then applies the
selection expression. To send payloads larger than 100 KB, specify `$request.body`.
|
|Request path|$request.path or ${request.path}|The request path, without the stage name.|
|Path parameter|$request.path.`name` or ${request.path.`name`}|The value of a path parameter in the request. For example if the route is `/pets/{petId}`,
you can map the `petId` parameter from the request with
`$request.path.petId`.|
|Context variable|$context.`variableName` or ${context.`variableName`}|The value of a [context variable](./http-api-logging-variables.html).
###### Note
Only the special characters `.` and `\_` are supported.
|
|Stage variable|$stageVariables.`variableName` or ${stageVariables.`variableName`}|The value of a [stage variable](./http-api-stages.stage-variables.html).|
|Static value|`string`|A constant value.|
###### Note
To use multiple variables in a selection expression, enclose the variable in brackets. For example,
`${request.path.name} ${request.path.id}`.
## Transforming API responses
You use response parameters to transform the HTTP response from a backend integration before returning the
response to clients. You can modify headers or the status code of a response before API Gateway returns the response to
clients.
You configure response parameters for each status code that your integration returns. Response parameters are
a key-value map. The key identifies the location of the request parameter to change, and how to change it. The
value specifies the new data for the parameter.
The following table shows supported keys.
|Type|Syntax|
|Header|`append|overwrite|remove:header.`headername``|
|Status code|`overwrite:statuscode`|
The following table shows supported values that you can map to parameters.
|Type|Syntax|Notes|
|Header value|$response.header.`name` or ${response.header.`name`}|Header names are case-insensitive. API Gateway combines multiple header values with commas, for example
`"header1": "value1,value2"`. Some headers are reserved. To learn more, see [Reserved headers](#http-api-mapping-reserved-headers).|
|Response body|$response.body.`name` or ${response.body.`name`}|A JSON path expression. Recursive descent (`$response.body..name`) and filter expressions
(`?(expression)`) aren't supported.
###### Note
When you specify a JSON path, API Gateway truncates the response body at 100 KB and then applies the
selection expression. To send payloads larger than 100 KB, specify `$response.body`.
|
|Context variable|$context.`variableName` or ${context.`variableName`}|The value of a supported [context variable](./http-api-logging-variables.html).|
|Stage variable|$stageVariables.`variableName` or ${stageVariables.`variableName`}|The value of a [stage variable](./http-api-stages.stage-variables.html).|
|Static value|`string`|A constant value.|
###### Note
To use multiple variables in a selection expression, enclose the variable in brackets. For example,
`${request.path.name} ${request.path.id}`.
## Examples
The following AWS CLI examples configure parameter mappings. For example CloudFormation templates, see [GitHub](https://github.com/awsdocs/amazon-api-gateway-developer-guide/tree/main/cloudformation-templates).
### Change the response from an integration
The following [create-integration](https://docs.aws.amazon.com/cli/latest/reference/apigatewayv2/create-integration.html) command
configures response parameters for an integration. When the integrations returns a 500 status code, API Gateway
changes the status code to 403, and adds `header1`1 to the response. When the integration returns a
404 status code, API Gateway adds an `error` header to the response.
```
`aws apigatewayv2 create-integration \\
--api-id abcdef123 \\
--integration-type HTTP\_PROXY \\
--payload-format-version 1.0 \\
--integration-uri 'https://api.example.com' \\
--integration-method ANY \\
--response-parameters '{"500" : {"append:header.header1": "$context.requestId", "overwrite:statuscode" : "403"}, "404" : {"append:header.error" : "$stageVariables.environmentId"} }'`
```
### Remove configured parameter mappings
The following [update-integration](https://docs.aws.amazon.com/cli/latest/reference/apigatewayv2/create-integration.html) command
removes previously configured request parameters for `append:header.header1`. It also removes
previously configured response parameters for a 200 status code.
```
`aws apigatewayv2 update-integration \\
--api-id abcdef123 \\
--integration-id hijk456 \\
--request-parameters '{"append:header.header1" : ""}' \\
--response-parameters '{"200" : {}}' `
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
CORS
OpenAPI
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.