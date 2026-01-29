---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-override-request-response-parameters.html
title: Override your API's request and response parameters and status codes for REST APIs in API Gateway
word_count: 1038
filtered: true
elements_removed: 0
density_score: 0.75
---

Override your API's request and response parameters and status codes for REST APIs in API Gateway - Amazon API Gateway
Override your API's request and response parameters and status codes for REST APIs in API Gateway - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#apigateway-override-request-response-parameters)
[Example 1: Override the status code based on the integration
body](#apigateway-override-request-response-examples)[Example 2: Override the request header and create new
headers](#apigateway-override-request-response-examples-2)
# Override your API's request and response parameters and status codes for REST APIs in API Gateway
You can use mapping template transformations to override any type of request parameter, response header, or response status
code. You use a mapping template to do the following:
* Perform many-to-one parameter mappings
* Override parameters after standard API Gateway mappings have been applied
* Conditionally map parameters based on body content or other parameter values
* Programmatically create new parameters
* Override status codes returned by your integration endpoint
Overrides are final. An override may only be applied to each parameter once. If you try to override the same
parameter multiple times, API Gateway returns a `5XX` response. If you must override the same
parameter multiple times throughout the template, we recommend creating a variable and applying the override at
the end of the template. The template is applied only after the entire template is parsed.
## Example 1: Override the status code based on the integration
body
The following example use the [example API](./api-gateway-create-api-from-example.html) to
override the status code based on the integration response body.
AWS Management Console
###### To override a status code based on the integration response body
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. Choose **Create API**.
3. For **REST API**, choose **Build**.
4. For **API details**, choose **Example API**.
5. Choose **Create API**.
API Gateway creates an example pet store API. To retrieve information about a pet, you use the API method request of `GET
/pets/{petId}`, where `{petId}` is a path parameter corresponding to an ID number for a pet.
In this example, you override the `GET` method's response code to `400` when an
error condition is detected.
6. In the **Resources** tree, choose the `GET` method under
`/{petId}`.
7. First, you test the current implementation of the API.
Choose the **Test** tab. You might need to choose the right arrow button to show the
tab.
8. For **petId**, enter `-1`, and then choose
**Test**.
The **Response body** indicates an out-of-range error:
```
`{
"errors": [
{
"key": "GetPetRequest.petId",
"message": "The value is out of range."
}
]
}`
```
In addition, the last line under **Logs** ends with: `Method completed with status:
200`.
The integration was completed successfully, but there was an error. Now you'll override the status
code based on the integration response.
9. On the **Integration response** tab, for the **Default - Response**,
choose **Edit**.
10. Choose **Mapping templates**.
11. Choose **Add mapping template**.
12. For **Content type**, enter `application/json`.
13. For **Template body**, enter the following:
```
`#set($inputRoot = $input.path('$'))
$input.json("$")
#end`
```
This mapping template uses the `$context.responseOverride.status` variable to override the
status code to `400` if the integration response contains the string `error`.
14. Choose **Save**.
15. Choose the **Test** tab.
16. For **petId**, enter `-1`.
17. In the results, the **Response Body** indicates an out-of-range error:
```
`{
"errors": [
{
"key": "GetPetRequest.petId",
"message": "The value is out of range."
}
]
}`
```
However, the last line under **Logs** now ends with: `Method completed with
status: 400`.
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
title: PetStore Example 1
description: Example pet store API.
version: "2025-01-14T00:13:18Z"
paths:
/pets/{petId}:
get:
parameters:
- name: petId
in: path
required: true
schema:
type: string
responses:
"200":
description: 200 response
x-amazon-apigateway-integration:
httpMethod: GET
uri: http://petstore.execute-api.us-east-1.amazonaws.com/petstore/pets/{petId}
responses:
default:
statusCode: "200"
responseTemplates:
application/json: |-
#end
requestParameters:
integration.request.path.petId: method.request.path.petId
passthroughBehavior: when\_no\_match
type: http
components:
schemas:
Pet:
type: object
properties:
id:
type: integer
type:
type: string
price:
type: number
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
The following OpenAPI definition creates the `GET pets/{petId}` resource and overrides the
status code based on the integration body.
```
`{
"openapi" : "3.0.1",
"info" : {
"title" : "PetStore Example 1",
"description" : "Example pet store API.",
"version" : "2025-01-14T00:13:18Z"
},
"paths" : {
"/pets/{petId}" : {
"get" : {
"parameters" : [ {
"name" : "petId",
"in" : "path",
"required" : true,
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
"uri" : "http://petstore.execute-api.us-east-1.amazonaws.com/petstore/pets/{petId}",
"responses" : {
"default" : {
"statusCode" : "200",
"responseTemplates" : {
"application/json" : "#set($inputRoot = $input.path('$'))\\n$input.json(\\"$\\")\\n#if($inputRoot.toString().contains(\\"error\\"))\\n#set($context.responseOverride.status = 400)\\n#end"
}
}
},
"requestParameters" : {
"integration.request.path.petId" : "method.request.path.petId"
},
"passthroughBehavior" : "when\_no\_match",
"type" : "http"
}
}
}
},
"components" : {
"schemas" : {
"Pet" : {
"type" : "object",
"properties" : {
"id" : {
"type" : "integer"
},
"type" : {
"type" : "string"
},
"price" : {
"type" : "number"
}
}
}
}
}
}`
```
###### To override a method's
request header by creating a new header
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. Choose the example API you created in the previous tutorial. The name of the API should be **PetStore**.
3. In the **Resources** tree, choose the `GET` method under
`/pet`.
4. On the **Method request** tab, for **Method request settings**, choose
**Edit**.
5. Choose **HTTP request headers**, and then choose **Add
header**.
6. For **Name**, enter `header1`.
7. Choose **Add header**, and then create a second header called
`header2`.
8. Choose **Save**.
Now, you combine these headers into one header value using a mapping template.
9. On the **Integration request** tab, for **Integration request
settings**, choose **Edit**.
10. For **Request body passthrough**, select **When there are no templates defined
(recommended)**.
11. Choose **Mapping templates**, and then do the following:
1. Choose **Add mapping template**.
2. For **Content type**, enter `application/json`.
3. For **Template body**, enter the following:
```
`#set($header1Override = "pets")