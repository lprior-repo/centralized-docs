---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-extensions-integrations.html
title: x-amazon-apigateway-integrations object
word_count: 262
filtered: true
elements_removed: 0
density_score: 0.70
---

x-amazon-apigateway-integrations object - Amazon API Gateway
x-amazon-apigateway-integrations object - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#api-gateway-extensions-integrations)
[x-amazon-apigateway-integrations example](#api-gateway-swagger-extensions-integrations-example)
# x-amazon-apigateway-integrations object
Defines a collection of integrations. You can define integrations in the
components section of your OpenAPI definition, and reuse the integrations for multiple
routes. Supported only for HTTP APIs.
|Property name|Type|Description|
|`integration`|[x-amazon-apigateway-integration object](./api-gateway-swagger-extensions-integration.html)|A collection of integration objects.|
## x-amazon-apigateway-integrations example
The following example creates an HTTP API that defines two integrations, and
references the integrations by using `$ref": "#/components/x-amazon-apigateway-integrations/`integration-name``.
```
`{
"openapi": "3.0.1",
"info":
{
"title": "Integrations",
"description": "An API that reuses integrations",
"version": "1.0"
},
"servers": [
{
"url": "https://example.com/{basePath}",
"description": "The production API server",
"variables":
{
"basePath":
{
"default": "example/path"
}
}
}],
"paths":
{
"/":
{
"get":
{
"x-amazon-apigateway-integration":
{
"$ref": "#/components/x-amazon-apigateway-integrations/integration1"
}
}
},
"/pets":
{
"get":
{
"x-amazon-apigateway-integration":
{
"$ref": "#/components/x-amazon-apigateway-integrations/integration1"
}
}
},
"/checkout":
{
"get":
{
"x-amazon-apigateway-integration":
{
"$ref": "#/components/x-amazon-apigateway-integrations/integration2"
}
}
}
},
"components": {
"x-amazon-apigateway-integrations":
{
"integration1":
{
"type": "aws\_proxy",
"httpMethod": "POST",
"uri": "arn:aws:apigateway:us-east-2:lambda:path/2015-03-31/functions/arn:aws:lambda:us-east-2:123456789012:function:`my-function`/invocations",
"passthroughBehavior": "when\_no\_templates",
"payloadFormatVersion": "1.0"
},
"integration2":
{
"type": "aws\_proxy",
"httpMethod": "POST",
"uri": "arn:aws:apigateway:us-east-2:lambda:path/2015-03-31/functions/arn:aws:lambda:us-east-2:123456789012:function:`example-function`/invocations",
"passthroughBehavior": "when\_no\_templates",
"payloadFormatVersion" : "1.0"
}
}
}
}
`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
x-amazon-apigateway-integration
x-amazon-apigateway-integration.requestTemplates
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.