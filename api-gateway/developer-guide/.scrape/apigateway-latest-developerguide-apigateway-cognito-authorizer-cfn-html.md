---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-cognito-authorizer-cfn.html
title: Create an Amazon Cognito authorizer for a REST API using CloudFormation
word_count: 424
filtered: true
elements_removed: 0
density_score: 0.93
---

Create an Amazon Cognito authorizer for a REST API using CloudFormation - Amazon API Gateway
Create an Amazon Cognito authorizer for a REST API using CloudFormation - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#apigateway-cognito-authorizer-cfn)
# Create an Amazon Cognito authorizer for a REST API using CloudFormation
You can use CloudFormation to create an Amazon Cognito user pool and an Amazon Cognito authorizer. The example CloudFormation template does the following:
* Create an Amazon Cognito user pool. The client must first sign the user in to the user pool and obtain an [identity or access token](https://docs.aws.amazon.com/cognito/latest/developerguide/amazon-cognito-user-pools-using-tokens-with-identity-providers.html). If you're using access tokens to authorize API method calls, be sure to
configure the app integration with the user pool to set up the custom scopes that you want on a given
resource server.
* Creates an API Gateway API with a `GET` method.
* Creates an Amazon Cognito authorizer that uses the `Authorization` header as the token source.
```
`AWSTemplateFormatVersion: 2010-09-09
Resources:
UserPool:
Type: AWS::Cognito::UserPool
Properties:
AccountRecoverySetting:
RecoveryMechanisms:
- Name: verified\_phone\_number
Priority: 1
- Name: verified\_email
Priority: 2
AdminCreateUserConfig:
AllowAdminCreateUserOnly: true
EmailVerificationMessage: The verification code to your new account is {####}
EmailVerificationSubject: Verify your new account
SmsVerificationMessage: The verification code to your new account is {####}
VerificationMessageTemplate:
DefaultEmailOption: CONFIRM\_WITH\_CODE
EmailMessage: The verification code to your new account is {####}
EmailSubject: Verify your new account
SmsMessage: The verification code to your new account is {####}
UpdateReplacePolicy: Retain
DeletionPolicy: Retain
CogAuthorizer:
Type: AWS::ApiGateway::Authorizer
Properties:
Name: CognitoAuthorizer
RestApiId:
Ref: Api
Type: COGNITO\_USER\_POOLS
IdentitySource: method.request.header.Authorization
ProviderARNs:
- Fn::GetAtt:
- UserPool
- Arn
Api:
Type: AWS::ApiGateway::RestApi
Properties:
Name: MyCogAuthApi
ApiDeployment:
Type: AWS::ApiGateway::Deployment
Properties:
RestApiId:
Ref: Api
DependsOn:
- CogAuthorizer
- ApiGET
ApiDeploymentStageprod:
Type: AWS::ApiGateway::Stage
Properties:
RestApiId:
Ref: Api
DeploymentId:
Ref: ApiDeployment
StageName: prod
ApiGET:
Type: AWS::ApiGateway::Method
Properties:
HttpMethod: GET
ResourceId:
Fn::GetAtt:
- Api
- RootResourceId
RestApiId:
Ref: Api
AuthorizationType: COGNITO\_USER\_POOLS
AuthorizerId:
Ref: CogAuthorizer
Integration:
IntegrationHttpMethod: GET
Type: HTTP\_PROXY
Uri: http://petstore-demo-endpoint.execute-api.com/petstore/pets
Outputs:
ApiEndpoint:
Value:
Fn::Join:
- ""
- - https://
- Ref: Api
- .execute-api.
- Ref: AWS::Region
- "."
- Ref: AWS::URLSuffix
- /
- Ref: ApiDeploymentStageprod
- /`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Configure cross-account Amazon Cognito authorizer for a REST API
Integrations
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.