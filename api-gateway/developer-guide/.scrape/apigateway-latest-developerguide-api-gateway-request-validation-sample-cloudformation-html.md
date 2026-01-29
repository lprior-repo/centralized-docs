---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-request-validation-sample-cloudformation.html
title: AWS CloudFormation template of a sample API with basic request validation
word_count: 511
filtered: true
elements_removed: 0
density_score: 0.76
---

AWS CloudFormation template of a sample API with basic request validation - Amazon API Gateway
AWS CloudFormation template of a sample API with basic request validation - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#api-gateway-request-validation-sample-cloudformation)
# AWS CloudFormation template of a sample API with basic request validation
The following CloudFormation example template definition defines a sample API with request validation
enabled. The API is a subset of the [PetStore
API](http://petstore-demo-endpoint.execute-api.com/petstore/pets). It exposes a `POST` method to add a pet to the
`pets` collection and a `GET` method to query pets by a
specified type.
There are two request validators declared:
**`GETValidator`**
This validator is enabled on the `GET` method. It allows API Gateway to verify that the required
query parameter (`q1`) is included and not blank in the incoming request.
**`POSTValidator`**
This validator is enabled on the `POST` method. It allows API Gateway to verify that payload
request format adheres to the specified `RequestBodyModel` when the content type is
`application/json` if no matching content type is found, request validation is not performed.
To use the same model regardless of the content type, specify `$default`.
`RequestBodyModel` contains an additional model, `RequestBodyModelId`, to define the pet ID.
```
`AWSTemplateFormatVersion: 2010-09-09
Parameters:
StageName:
Type: String
Default: v1
Description: Name of API stage.
Resources:
Api:
Type: 'AWS::ApiGateway::RestApi'
Properties:
Name: ReqValidatorsSample
RequestBodyModelId:
Type: 'AWS::ApiGateway::Model'
Properties:
RestApiId: !Ref Api
ContentType: application/json
Description: Request body model for Pet ID.
Schema:
$schema: 'http://json-schema.org/draft-04/schema#'
title: RequestBodyModelId
properties:
id:
type: integer
RequestBodyModel:
Type: 'AWS::ApiGateway::Model'
Properties:
RestApiId: !Ref Api
ContentType: application/json
Description: Request body model for Pet type, name, price, and ID.
Schema:
$schema: 'http://json-schema.org/draft-04/schema#'
title: RequestBodyModel
required:
- price
- name
- type
type: object
properties:
id:
"$ref": !Sub
- 'https://apigateway.amazonaws.com/restapis/${Api}/models/${RequestBodyModelId}'
- Api: !Ref Api
RequestBodyModelId: !Ref RequestBodyModelId
price:
type: number
minimum: 25
maximum: 500
name:
type: string
type:
type: string
enum:
- "dog"
- "cat"
- "fish"
GETValidator:
Type: AWS::ApiGateway::RequestValidator
Properties:
Name: params-only
RestApiId: !Ref Api
ValidateRequestBody: False
ValidateRequestParameters: True
POSTValidator:
Type: AWS::ApiGateway::RequestValidator
Properties:
Name: body-only
RestApiId: !Ref Api
ValidateRequestBody: True
ValidateRequestParameters: False
ValidationResource:
Type: 'AWS::ApiGateway::Resource'
Properties:
RestApiId: !Ref Api
ParentId: !GetAtt Api.RootResourceId
PathPart: 'validation'
ValidationMethodGet:
Type: 'AWS::ApiGateway::Method'
Properties:
RestApiId: !Ref Api
ResourceId: !Ref ValidationResource
HttpMethod: GET
AuthorizationType: NONE
RequestValidatorId: !Ref GETValidator
RequestParameters:
method.request.querystring.q1: true
Integration:
Type: HTTP\_PROXY
IntegrationHttpMethod: GET
Uri: http://petstore-demo-endpoint.execute-api.com/petstore/pets/
ValidationMethodPost:
Type: 'AWS::ApiGateway::Method'
Properties:
RestApiId: !Ref Api
ResourceId: !Ref ValidationResource
HttpMethod: POST
AuthorizationType: NONE
RequestValidatorId: !Ref POSTValidator
RequestModels:
application/json : !Ref RequestBodyModel
Integration:
Type: HTTP\_PROXY
IntegrationHttpMethod: POST
Uri: http://petstore-demo-endpoint.execute-api.com/petstore/pets/
ApiDeployment:
Type: 'AWS::ApiGateway::Deployment'
DependsOn:
- ValidationMethodGet
- RequestBodyModel
Properties:
RestApiId: !Ref Api
StageName: !Sub '${StageName}'
Outputs:
ApiRootUrl:
Description: Root Url of the API
Value: !Sub 'https://${Api}.execute-api.${AWS::Region}.amazonaws.com/${StageName}'
`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Set up basic request validation
in API Gateway
Data transformations
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.