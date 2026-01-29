---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/api-key-usage-plan-cfn.html
title: Create and configure API keys and usage plans with CloudFormation
word_count: 467
filtered: true
elements_removed: 0
density_score: 0.93
---

Create and configure API keys and usage plans with CloudFormation - Amazon API Gateway
Create and configure API keys and usage plans with CloudFormation - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#api-key-usage-plan-cfn)
# Create and configure API keys and usage plans with CloudFormation
You can use CloudFormation to require API keys on API methods and create a usage plan for an API. The example CloudFormation template does the following:
* Creates an API Gateway API with `GET` and `POST` methods.
* Requires an API key for the `GET` and `POST` methods. This API receives keys from the `X-API-KEY` header of each incoming request.
* Creates an API key.
* Creates a usage plan to specify a monthly quota of 1,000 request each month, a throttling rate limit of 100
request each second, and a throttling burst limit of 200 request each second.
* Specifies a method-level throttling rate limit of 50 requests each second and a method-level throttling
burst limit of 100 requests per second for the `GET` method.
* Associates the API stage and API key with the usage plan.
```
`AWSTemplateFormatVersion: 2010-09-09
Parameters:
StageName:
Type: String
Default: v1
Description: Name of API stage.
KeyName:
Type: String
Default: MyKeyName
Description: Name of an API key
Resources:
Api:
Type: 'AWS::ApiGateway::RestApi'
Properties:
Name: keys-api
ApiKeySourceType: HEADER
PetsResource:
Type: 'AWS::ApiGateway::Resource'
Properties:
RestApiId: !Ref Api
ParentId: !GetAtt Api.RootResourceId
PathPart: 'pets'
PetsMethodGet:
Type: 'AWS::ApiGateway::Method'
Properties:
RestApiId: !Ref Api
ResourceId: !Ref PetsResource
HttpMethod: GET
ApiKeyRequired: true
AuthorizationType: NONE
Integration:
Type: HTTP\_PROXY
IntegrationHttpMethod: GET
Uri: http://petstore-demo-endpoint.execute-api.com/petstore/pets/
PetsMethodPost:
Type: 'AWS::ApiGateway::Method'
Properties:
RestApiId: !Ref Api
ResourceId: !Ref PetsResource
HttpMethod: POST
ApiKeyRequired: true
AuthorizationType: NONE
Integration:
Type: HTTP\_PROXY
IntegrationHttpMethod: GET
Uri: http://petstore-demo-endpoint.execute-api.com/petstore/pets/
ApiDeployment:
Type: 'AWS::ApiGateway::Deployment'
DependsOn:
- PetsMethodGet
Properties:
RestApiId: !Ref Api
StageName: !Sub '${StageName}'
UsagePlan:
Type: AWS::ApiGateway::UsagePlan
DependsOn:
- ApiDeployment
Properties:
Description: Example usage plan with a monthly quota of 1000 calls and method-level throttling for /pets GET
ApiStages:
- ApiId: !Ref Api
Stage: !Sub '${StageName}'
Throttle:
"/pets/GET":
RateLimit: 50.0
BurstLimit: 100
Quota:
Limit: 1000
Period: MONTH
Throttle:
RateLimit: 100.0
BurstLimit: 200
UsagePlanName: "My Usage Plan"
ApiKey:
Type: AWS::ApiGateway::ApiKey
Properties:
Description: API Key
Name: !Sub '${KeyName}'
Enabled: True
UsagePlanKey:
Type: AWS::ApiGateway::UsagePlanKey
Properties:
KeyId: !Ref ApiKey
KeyType: API\_KEY
UsagePlanId: !Ref UsagePlan
Outputs:
ApiRootUrl:
Description: Root Url of the API
Value: !Sub 'https://${Api}.execute-api.${AWS::Region}.amazonaws.com/${StageName}'`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Maintain a usage plan for REST APIs in API Gateway
Configure a method to use API keys with an OpenAPI definition
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.