---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/request-response-data-mappings.html
title: Parameter mapping examples for REST APIs in API Gateway
word_count: 2110
filtered: true
elements_removed: 0
density_score: 0.76
---

Parameter mapping examples for REST APIs in API Gateway - Amazon API Gateway
Parameter mapping examples for REST APIs in API Gateway - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#request-response-data-mappings)
[Example 1: Map a method request parameter to an
integration request parameter](#request-response-data-mappings-example-1)[Example 2: Map multiple method request parameters to
different integration request parameters](#request-response-data-mappings-example-2)[Example 3: Map fields from the JSON request body to integration request parameters](#request-response-data-mappings-example-3)[Example 4: Map the integration response to the method response](#request-response-data-mappings-example-4)
# Parameter mapping examples for REST APIs in API Gateway
The following examples show how to create parameter mapping expressions using the API Gateway console, OpenAPI, and
CloudFormation templates. For an example of how to use parameter mapping to create the required CORS headers, see [CORS for REST APIs in API Gateway](./how-to-cors.html).
## Example 1: Map a method request parameter to an
integration request parameter
The following example maps the method request header parameter `puppies` to the integration request header parameter
`DogsAge0`.
AWS Management Console
###### To map the method request parameter
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. Choose a REST API.
3. Choose a method.
Your method must have a non-proxy integration.
4. For **Method request settings**, choose **Edit**.
5. Choose **HTTP request headers**.
6. Choose **Add header**.
7. For **Name**, enter `puppies`.
8. Choose **Save**.
9. Choose the **Integration request** tab, and then for **Integration request
settings**, choose **Edit**.
The AWS Management Console automatically adds a parameter mapping from `method.request.header.puppies
` to `puppies` for you, but you need to change the
**Name** to match the request header parameter that is expected by your integration endpoint.
10. For
**Name**, enter `DogsAge0`.
11. Choose **Save**.
12. Redeploy your API for the changes to take effect.
The following steps show you how to verify that your parameter mapping was successful.
###### (Optional) Test your parameter mapping
1. Choose the **Test** tab. You might need to choose the right arrow button to show the tab.
2. For headers, enter `puppies:true`.
3. Choose **Test**.
4. In the **Logs**, the result should look like the following:
```
`Tue Feb 04 00:28:36 UTC 2025 : Method request headers: {puppies=true}
Tue Feb 04 00:28:36 UTC 2025 : Method request body before transformations:
Tue Feb 04 00:28:36 UTC 2025 : Endpoint request URI: http://petstore-demo-endpoint.execute-api.com/petstore/pets
Tue Feb 04 00:28:36 UTC 2025 : Endpoint request headers: {DogsAge0=true, x-amzn-apigateway-api-id=abcd1234, Accept=application/json, User-Agent=AmazonAPIGateway\_aaaaaaa, X-Amzn-Trace-Id=Root=1-abcd-12344}`
```
The request header parameter has changed from
`puppies` to `DogsAge0`.
CloudFormation
In this example, you use the
[body](https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-restapi.html#cfn-apigateway-restapi-body) property to import an OpenAPI definition file into API Gateway.
```
`AWSTemplateFormatVersion: 2010-09-09
Resources:
Api:
Type: 'AWS::ApiGateway::RestApi'
Properties:
Body:
openapi: 3.0.1
info:
title: ParameterMappingExample
version: "2025-02-04T00:30:41Z"
paths:
/pets:
get:
parameters:
- name: puppies
in: header
schema:
type: string
responses:
"200":
description: 200 response
x-amazon-apigateway-integration:
httpMethod: GET
uri: http://petstore-demo-endpoint.execute-api.com/petstore/pets
responses:
default:
statusCode: "200"
requestParameters:
integration.request.header.DogsAge0: method.request.header.puppies
passthroughBehavior: when\_no\_match
type: http
ApiGatewayDeployment:
Type: 'AWS::ApiGateway::Deployment'
DependsOn: Api
Properties:
RestApiId: !Ref Api
ApiGatewayDeployment20250219:
Type: 'AWS::ApiGateway::Deployment'
DependsOn: Api
Properties:
RestApiId: !Ref Api
Stage:
Type: 'AWS::ApiGateway::Stage'
Properties:
DeploymentId: !Ref ApiGatewayDeployment20250219
RestApiId: !Ref Api
StageName: prod`
```
OpenAPI
```
`{
"openapi" : "3.0.1",
"info" : {
"title" : "ParameterMappingExample",
"version" : "2025-02-04T00:30:41Z"
},
"paths" : {
"/pets" : {
"get" : {
"parameters" : [ {
"name" : "puppies",
"in" : "header",
"schema" : {
"type" : "string"
}
} ],
"responses" : {
"200" : {
"description" : "200 response"
}
},
"x-amazon-apigateway-integration" : {
"httpMethod" : "GET",
"uri" : "http://petstore-demo-endpoint.execute-api.com/petstore/pets",
"responses" : {
"default" : {
"statusCode" : "200"
}
},
"requestParameters" : {
"integration.request.header.DogsAge0" : "method.request.header.puppies"
},
"passthroughBehavior" : "when\_no\_match",
"type" : "http"
}
}
}
}
}`
```
## Example 2: Map multiple method request parameters to
different integration request parameters
The following example maps the multi-value method request query string parameter
`methodRequestQueryParam` to the integration request query
string parameter `integrationQueryParam` and maps the method request header parameter
`methodRequestHeaderParam` to the integration request path parameter
`integrationPathParam`.
AWS Management Console
###### To map the method request parameters
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. Choose a REST API.
3. Choose a method.
Your method must have a non-proxy integration.
4. For **Method request settings**, choose **Edit**.
5. Choose **URL query string parameters**.
6. Choose **Add query string**.
7. For **Name**, enter `methodRequestQueryParam`.
8. Choose **HTTP request headers**.
9. Choose **Add header**.
10. For
**Name**, enter `methodRequestHeaderParam`.
11. Choose **Save**.
12. Choose the **Integration request** tab, and then for **Integration request
settings**, choose **Edit**.
13. Choose **URL path parameters**.
14. Choose **Add path parameter**.
15. For
**Name**, enter `integrationPathParam`.
16. For **Mapped from**, enter `method.request.header.methodRequestHeaderParam`.
This maps the method request header you specified in the method request to a new integration request
path parameter.
17. Choose **URL query string parameters**.
18. Choose **Add query string**.
19. For
**Name**, enter `integrationQueryParam`.
20. For **Mapped from**, enter `method.request.multivaluequerystring.methodRequestQueryParam`.
This maps the multivalue query string parameter to a new single valued integration request query
string parameter.
21. Choose **Save**.
22. Redeploy your API for the changes to take effect.
CloudFormation
In this example, you use the
[body](https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-restapi.html#cfn-apigateway-restapi-body) property to import an OpenAPI definition file into API Gateway.
The following OpenAPI definition creates the following parameter mappings for an HTTP
integration:
* The method request's
header, named `methodRequestHeaderParam`, into the integration request path
parameter, named `integrationPathParam`
* The multi-value method request query
string, named `methodRequestQueryParam`, into the integration request
query string, named `integrationQueryParam`
```
`AWSTemplateFormatVersion: 2010-09-09
Resources:
Api:
Type: 'AWS::ApiGateway::RestApi'
Properties:
Body:
openapi: 3.0.1
info:
title: Parameter mapping example 2
version: "2025-01-15T19:12:31Z"
paths:
/:
post:
parameters:
- name: methodRequestQueryParam
in: query
schema:
type: string
- name: methodRequestHeaderParam
in: header
schema:
type: string
responses:
"200":
description: 200 response
x-amazon-apigateway-integration:
httpMethod: GET
uri: http://petstore-demo-endpoint.execute-api.com/petstore/pets
responses:
default:
statusCode: "200"
requestParameters:
integration.request.querystring.integrationQueryParam: method.request.multivaluequerystring.methodRequestQueryParam
integration.request.path.integrationPathParam: method.request.header.methodRequestHeaderParam
requestTemplates:
application/json: '{"statusCode": 200}'
passthroughBehavior: when\_no\_templates
timeoutInMillis: 29000
type: http
ApiGatewayDeployment:
Type: 'AWS::ApiGateway::Deployment'
DependsOn: Api
Properties:
RestApiId: !Ref Api
ApiGatewayDeployment20250219:
Type: 'AWS::ApiGateway::Deployment'
DependsOn: Api
Properties:
RestApiId: !Ref Api
Stage:
Type: 'AWS::ApiGateway::Stage'
Properties:
DeploymentId: !Ref ApiGatewayDeployment20250219
RestApiId: !Ref Api
StageName: prod`
```
OpenAPI
The following OpenAPI definition creates the following parameter mappings for an HTTP
integration:
* The method request's
header, named `methodRequestHeaderParam`, into the integration request path
parameter, named `integrationPathParam`
* The multi-value method request query
string, named `methodRequestQueryParam`, into the integration request
query string, named `integrationQueryParam`
```
`{
"openapi" : "3.0.1",
"info" : {
"title" : "Parameter mapping example 2",
"version" : "2025-01-15T19:12:31Z"
},
"paths" : {
"/" : {
"post" : {
"parameters" : [ {
"name" : "methodRequestQueryParam",
"in" : "query",
"schema" : {
"type" : "string"
}
}, {
"name" : "methodRequestHeaderParam",
"in" : "header",
"schema" : {
"type" : "string"
}
} ],
"responses" : {
"200" : {
"description" : "200 response"
}
},
"x-amazon-apigateway-integration" : {
"httpMethod" : "GET",
"uri" : "http://petstore-demo-endpoint.execute-api.com/petstore/pets",
"responses" : {
"default" : {
"statusCode" : "200"
}
},
"requestParameters" : {
"integration.request.querystring.integrationQueryParam" : "method.request.multivaluequerystring.methodRequestQueryParam",
"integration.request.path.integrationPathParam" : "method.request.header.methodRequestHeaderParam"
},
"requestTemplates" : {
"application/json" : "{\\"statusCode\\": 200}"
},
"passthroughBehavior" : "when\_no\_templates",
"timeoutInMillis" : 29000,
"type" : "http"
}
}
}
}
}
`
```
## Example 3: Map fields from the JSON request body to integration request parameters
You can also map integration request parameters from fields in the JSON request body using a [JSONPath expression](http://goessner.net/articles/JsonPath/index.html#e2). The following example maps
the method request body to an integration request header named `body-header` and maps part of the request
body, as expressed by a JSON expression to an integration request header named `pet-price`.
To test this example, provide an input that contains a price category, such as the following:
```
`[
{
"id": 1,
"type": "dog",
"price": 249.99
}
]`
```
AWS Management Console
###### To map the method request parameters
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. Choose a REST API.
3. Choose a `POST`, `PUT`, `PATCH`, or `ANY` method.
Your method must have a non-proxy integration.
4. For **Integration request settings**, choose **Edit**.
5. Choose **URL request headers parameters**.
6. Choose **Add request header parameter**.
7. For
**Name**, enter `body-header`.
8. For **Mapped from**, enter `method.request.body`.
This maps the method request body to a new integration request header parameter.
9. Choose **Add request header parameter**.
10. For
**Name**, enter `pet-price`.
11. For **Mapped from**, enter `
method.request.body[0].price`.
This maps a part of the method request body to a new integration request header parameter.
12. Choose **Save**.
13. Redeploy your API for the changes to take effect.
CloudFormation
In this example, you use the
[body](https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-restapi.html#cfn-apigateway-restapi-body) property to import an OpenAPI definition file into API Gateway.
```
`AWSTemplateFormatVersion: 2010-09-09
Resources:
Api:
Type: 'AWS::ApiGateway::RestApi'
Properties:
Body:
openapi: 3.0.1
info:
title: Parameter mapping example 3
version: "2025-01-15T19:19:14Z"
paths:
/:
post:
responses:
"200":
description: 200 response
x-amazon-apigateway-integration:
httpMethod: GET
uri: http://petstore-demo-endpoint.execute-api.com/petstore/pets
responses:
default:
statusCode: "200"
requestParameters:
integration.request.header.pet-price: method.request.body[0].price
integration.request.header.body-header: method.request.body
requestTemplates:
application/json: '{"statusCode": 200}'
passthroughBehavior: when\_no\_templates
timeoutInMillis: 29000
type: http
ApiGatewayDeployment:
Type: 'AWS::ApiGateway::Deployment'
DependsOn: Api
Properties:
RestApiId: !Ref Api
ApiGatewayDeployment20250219:
Type: 'AWS::ApiGateway::Deployment'
DependsOn: Api
Properties:
RestApiId: !Ref Api
Stage:
Type: 'AWS::ApiGateway::Stage'
Properties:
DeploymentId: !Ref ApiGatewayDeployment20250219
RestApiId: !Ref Api
StageName: prod`
```
OpenAPI
The following OpenAPI definition map integration request parameters from fields in the JSON request body.
```
`{
"openapi" : "3.0.1",
"info" : {
"title" : "Parameter mapping example 3",
"version" : "2025-01-15T19:19:14Z"
},
"paths" : {
"/" : {
"post" : {
"responses" : {
"200" : {
"description" : "200 response"
}
},
"x-amazon-apigateway-integration" : {
"httpMethod" : "GET",
"uri" : "http://petstore-demo-endpoint.execute-api.com/petstore/pets",
"responses" : {
"default" : {
"statusCode" : "200"
}
},
"requestParameters" : {
"integration.request.header.pet-price" : "method.request.body[0].price",
"integration.request.header.body-header" : "method.request.body"
},
"requestTemplates" : {
"application/json" : "{\\"statusCode\\": 200}"
},
"passthroughBehavior" : "when\_no\_templates",
"timeoutInMillis" : 29000,
"type" : "http"
}
}
}
}
}`
```
## Example 4: Map the integration response to the method response
You can also map the integration response to the method response. The following example maps the integration
response body to a method response header named `location`, maps the integration response header
`x-app-id` to the method response header `id`, and maps the multi-valued integration
response header `item` to the method response header `items`.
AWS Management Console
###### To map the integration response
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. Choose a REST API.
3. Choose a method.
Your method must have a non-proxy integration.
4. Choose the **Method response** tab, and then for
**Response 200**, choose **Edit**.
5. For **Header name**, choose **Add header**.
6. Create three headers named `id`, `item`, and
`location`.
7. Choose **Save**.
8. Choose the **Integration response** tab, and then for
**Default - Response**, choose **Edit**.
9. Under **Header mappings**, enter the following.
1. For **id**, enter `integration.response.header.x-app-id`
2. For **item**, enter `integration.response.multivalueheader.item`
3. For **location**, enter `integration.response.body.redirect.url`
4. Choose **Save**.
5. Redeploy your API for the changes to take effect.
CloudFormation
In this example, you use the
[body](https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-restapi.html#cfn-apigateway-restapi-body) property to import an OpenAPI definition file into API Gateway.
```
`AWSTemplateFormatVersion: 2010-09-09
Resources:
Api:
Type: 'AWS::ApiGateway::RestApi'
Properties:
Body:
openapi: 3.0.1
info:
title: Parameter mapping example
version: "2025-01-15T19:21:35Z"
paths:
/:
post:
responses:
"200":
description: 200 response
headers:
item:
schema:
type: string
location:
schema:
type: string
id:
schema:
type: string
x-amazon-apigateway-integration:
type: http
httpMethod: GET
uri: http://petstore-demo-endpoint.execute-api.com/petstore/pets
responses:
default:
statusCode: "200"
responseParameters:
method.response.header.id: integration.response.header.x-app-id
method.response.header.location: integration.response.body.redirect.url
method.response.header.item: integration.response.multivalueheader.item
requestTemplates:
application/json: '{"statusCode": 200}'
passthroughBehavior: when\_no\_templates
timeoutInMillis: 29000
ApiGatewayDeployment:
Type: 'AWS::ApiGateway::Deployment'
DependsOn: Api
Properties:
RestApiId: !Ref Api
ApiGatewayDeployment20250219:
Type: 'AWS::ApiGateway::Deployment'
DependsOn: Api
Properties:
RestApiId: !Ref Api
Stage:
Type: 'AWS::ApiGateway::Stage'
Properties:
DeploymentId: !Ref ApiGatewayDeployment20250219
RestApiId: !Ref Api
StageName: prod
`
```
OpenAPI
The following OpenAPI definition maps the integration response to the method response.
```
`{
"openapi" : "3.0.1",
"info" : {
"title" : "Parameter mapping example",
"version" : "2025-01-15T19:21:35Z"
},
"paths" : {
"/" : {
"post" : {
"responses" : {
"200" : {
"description" : "200 response",
"headers" : {
"item" : {
"schema" : {
"type" : "string"
}
},
"location" : {
"schema" : {
"type" : "string"
}
},
"id" : {
"schema" : {
"type" : "string"
}
}
}
}
},
"x-amazon-apigateway-integration" : {
"type" : "http",
"httpMethod" : "GET",
"uri" : "http://petstore-demo-endpoint.execute-api.com/petstore/pets",
"responses" : {
"default" : {
"statusCode" : "200",
"responseParameters" : {
"method.response.header.id" : "integration.response.header.x-app-id",
"method.response.header.location" : "integration.response.body.redirect.url",
"method.response.header.item" : "integration.response.multivalueheader.item"
}
}
},
"requestTemplates" : {
"application/json" : "{\\"statusCode\\": 200}"
},
"passthroughBehavior" : "when\_no\_templates",
"timeoutInMillis" : 29000
}
}
}
}
}`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Parameter mapping
Parameter mapping source reference
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.