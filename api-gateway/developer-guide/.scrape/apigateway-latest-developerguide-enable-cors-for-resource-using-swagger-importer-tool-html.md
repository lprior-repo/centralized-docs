---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/enable-cors-for-resource-using-swagger-importer-tool.html
title: Enable CORS on a
word_count: 730
filtered: true
elements_removed: 0
density_score: 0.72
---

Enable CORS on a resource using the API Gateway import API - Amazon API Gateway
Enable CORS on a resource using the API Gateway import API - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#enable-cors-for-resource-using-swagger-importer-tool)
[Example Options method](#enable-cors-for-resource-using-swagger-importer-tool-options)[Example API](#enable-cors-for-resource-using-swagger-importer-tool-complete-example)
# Enable CORS on a
resource using the API Gateway import API
If you are using the [API Gateway Import API](./api-gateway-import-api.html),
you can set up CORS support using an OpenAPI file. You must first define an
`OPTIONS` method in your resource that returns the required
headers.
###### Note
Web browsers expect Access-Control-Allow-Headers, and Access-Control-Allow-Origin
headers to be set up in each API method that accepts CORS requests. In addition,
some browsers first make an HTTP request to an `OPTIONS` method in the
same resource, and then expect to receive the same headers.
## Example `Options` method
The following example creates an `OPTIONS` method for a mock
integration.
OpenAPI 3.0
```
`/users:
options:
summary: CORS support
description: |
Enable CORS by returning correct headers
tags:
- CORS
responses:
200:
description: Default response for CORS method
headers:
Access-Control-Allow-Origin:
schema:
type: "string"
Access-Control-Allow-Methods:
schema:
type: "string"
Access-Control-Allow-Headers:
schema:
type: "string"
content: {}
x-amazon-apigateway-integration:
type: mock
requestTemplates:
application/json: "{\\"statusCode\\": 200}"
passthroughBehavior: "never"
responses:
default:
statusCode: "200"
responseParameters:
method.response.header.Access-Control-Allow-Headers: "'Content-Type,X-Amz-Date,Authorization,X-Api-Key'"
method.response.header.Access-Control-Allow-Methods: "'\*'"
method.response.header.Access-Control-Allow-Origin: "'\*'"
`
```
OpenAPI 2.0
```
`/users:
options:
summary: CORS support
description: |
Enable CORS by returning correct headers
consumes:
- "application/json"
produces:
- "application/json"
tags:
- CORS
x-amazon-apigateway-integration:
type: mock
requestTemplates: "{\\"statusCode\\": 200}"
passthroughBehavior: "never"
responses:
"default":
statusCode: "200"
responseParameters:
method.response.header.Access-Control-Allow-Headers : "'Content-Type,X-Amz-Date,Authorization,X-Api-Key'"
method.response.header.Access-Control-Allow-Methods : "'\*'"
method.response.header.Access-Control-Allow-Origin : "'\*'"
responses:
200:
description: Default response for CORS method
headers:
Access-Control-Allow-Headers:
type: "string"
Access-Control-Allow-Methods:
type: "string"
Access-Control-Allow-Origin:
type: "string"`
```
Once you have configured the `OPTIONS` method for your resource, you can
add the required headers to the other methods in the same resource that need to accept
CORS requests.
1. Declare the **Access-Control-Allow-Origin** and
**Headers** to the response types.
OpenAPI 3.0
```
` responses:
200:
description: Default response for CORS method
headers:
Access-Control-Allow-Origin:
schema:
type: "string"
Access-Control-Allow-Methods:
schema:
type: "string"
Access-Control-Allow-Headers:
schema:
type: "string"
content: {}`
```
OpenAPI 2.0
```
` responses:
200:
description: Default response for CORS method
headers:
Access-Control-Allow-Headers:
type: "string"
Access-Control-Allow-Methods:
type: "string"
Access-Control-Allow-Origin:
type: "string"`
```
2. In the `x-amazon-apigateway-integration` tag, set up the mapping
for those headers to your static values:
OpenAPI 3.0
```
` responses:
default:
statusCode: "200"
responseParameters:
method.response.header.Access-Control-Allow-Headers: "'Content-Type,X-Amz-Date,Authorization,X-Api-Key'"
method.response.header.Access-Control-Allow-Methods: "'\*'"
method.response.header.Access-Control-Allow-Origin: "'\*'"
responseTemplates:
application/json: |
{}`
```
OpenAPI 2.0
```
` responses:
"default":
statusCode: "200"
responseParameters:
method.response.header.Access-Control-Allow-Headers : "'Content-Type,X-Amz-Date,Authorization,X-Api-Key'"
method.response.header.Access-Control-Allow-Methods : "'\*'"
method.response.header.Access-Control-Allow-Origin : "'\*'"`
```
## Example API
The following example creates a complete API with an `OPTIONS` method and a `GET` method with an `HTTP` integration.
OpenAPI 3.0
```
`openapi: "3.0.1"
info:
title: "cors-api"
description: "cors-api"
version: "2024-01-16T18:36:01Z"
servers:
- url: "/{basePath}"
variables:
basePath:
default: "/test"
paths:
/:
get:
operationId: "GetPet"
responses:
"200":
description: "200 response"
headers:
Access-Control-Allow-Origin:
schema:
type: "string"
content: {}
x-amazon-apigateway-integration:
httpMethod: "GET"
uri: "http://petstore.execute-api.us-east-1.amazonaws.com/petstore/pets"
responses:
default:
statusCode: "200"
responseParameters:
method.response.header.Access-Control-Allow-Origin: "'\*'"
passthroughBehavior: "never"
type: "http"
options:
responses:
"200":
description: "200 response"
headers:
Access-Control-Allow-Origin:
schema:
type: "string"
Access-Control-Allow-Methods:
schema:
type: "string"
Access-Control-Allow-Headers:
schema:
type: "string"
content:
application/json:
schema:
$ref: "#/components/schemas/Empty"
x-amazon-apigateway-integration:
responses:
default:
statusCode: "200"
responseParameters:
method.response.header.Access-Control-Allow-Methods: "'GET,OPTIONS'"
method.response.header.Access-Control-Allow-Headers: "'Content-Type,X-Amz-Date,Authorization,X-Api-Key'"
method.response.header.Access-Control-Allow-Origin: "'\*'"
requestTemplates:
application/json: "{\\"statusCode\\": 200}"
passthroughBehavior: "never"
type: "mock"
components:
schemas:
Empty:
type: "object"`
```
OpenAPI 2.0
```
`swagger: "2.0"
info:
description: "cors-api"
version: "2024-01-16T18:36:01Z"
title: "cors-api"
basePath: "/test"
schemes:
- "https"
paths:
/:
get:
operationId: "GetPet"
produces:
- "application/json"
responses:
"200":
description: "200 response"
headers:
Access-Control-Allow-Origin:
type: "string"
x-amazon-apigateway-integration:
httpMethod: "GET"
uri: "http://petstore.execute-api.us-east-1.amazonaws.com/petstore/pets"
responses:
default:
statusCode: "200"
responseParameters:
method.response.header.Access-Control-Allow-Origin: "'\*'"
passthroughBehavior: "never"
type: "http"
options:
consumes:
- "application/json"
produces:
- "application/json"
responses:
"200":
description: "200 response"
schema:
$ref: "#/definitions/Empty"
headers:
Access-Control-Allow-Origin:
type: "string"
Access-Control-Allow-Methods:
type: "string"
Access-Control-Allow-Headers:
type: "string"
x-amazon-apigateway-integration:
responses:
default:
statusCode: "200"
responseParameters:
method.response.header.Access-Control-Allow-Methods: "'GET,OPTIONS'"
method.response.header.Access-Control-Allow-Headers: "'Content-Type,X-Amz-Date,Authorization,X-Api-Key'"
method.response.header.Access-Control-Allow-Origin: "'\*'"
requestTemplates:
application/json: "{\\"statusCode\\": 200}"
passthroughBehavior: "never"
type: "mock"
definitions:
Empty:
type: "object"`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Enable CORS using the console
Test CORS for an API Gateway API
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.