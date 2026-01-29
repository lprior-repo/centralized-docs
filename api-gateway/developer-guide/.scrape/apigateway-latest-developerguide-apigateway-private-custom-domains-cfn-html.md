---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-private-custom-domains-cfn.html
title: Create a custom domain name for private APIs using CloudFormation
word_count: 457
filtered: true
elements_removed: 0
density_score: 0.75
---

Create a custom domain name for private APIs using CloudFormation - Amazon API Gateway
Create a custom domain name for private APIs using CloudFormation - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#apigateway-private-custom-domains-cfn)
# Create a custom domain name for private APIs using CloudFormation
The following example CloudFormation template creates a private API and a private custom domain name, maps the private
API to the custom domain name, and then creates a domain name access association. You need to provide your own VPC
endpoint, domain name, and certificate ARN.
The following considerations might impact your use of CloudFormation to create a private
custom domain name:
* You can't reject a domain name access association using CloudFormation. To reject a domain name access association,
use the AWS CLI.
* Use the `AWS::ApiGateway::DomainNameV2` CloudFormation property to create a private custom domain
name.
* Use the `AWS::ApiGateway:BasePathMappingV2` CloudFormation property to create a base path mapping.
```
`AWSTemplateFormatVersion: 2010-09-09
Parameters:
EndpointID:
Type: String
Default: `vpce-abcd1234567efg`
Description: A VPC endpoint with enableDnsHostnames and enableDnsSupport set to true.
DomainName:
Type: String
Default: `private.example.com`
Description: A domain name that you own.
CertificateArn:
Type: String
Default: `arn:aws:acm:us-west-2:123456789:certificate/abcd-000-1234-0000-000000abcd`
Description: An ACM certificate that covers the domain name.
Resources:
PrivateApi:
Type: 'AWS::ApiGateway::RestApi'
Properties:
EndpointConfiguration:
Types:
- PRIVATE
VpcEndpointIds:
- !Ref EndpointID
Name: private-api
Policy:
Statement:
- Action: 'execute-api:Invoke'
Effect: Allow
Principal: '\*'
Resource: 'execute-api:/\*'
- Action: 'execute-api:Invoke'
Condition:
StringNotEquals:
'aws:SourceVpce': !Ref EndpointID
Effect: Deny
Principal: '\*'
Resource: 'execute-api:/\*'
Version: 2012-10-17
PrivateApiDeployment:
Type: 'AWS::ApiGateway::Deployment'
Properties:
RestApiId: !Ref PrivateApi
Description: Private API deployment
DependsOn:
- PrivateApiMethod
PrivateApiStage:
Type: 'AWS::ApiGateway::Stage'
Properties:
RestApiId: !Ref PrivateApi
DeploymentId: !Ref PrivateApiDeployment
StageName: prod
PrivateApiMethod:
Type: 'AWS::ApiGateway::Method'
Properties:
HttpMethod: ANY
ResourceId: !GetAtt PrivateApi.RootResourceId
RestApiId: !Ref PrivateApi
AuthorizationType: NONE
Integration:
Type: MOCK
RequestTemplates:
application/json: "{\\"statusCode\\": 200}"
IntegrationResponses:
- StatusCode: '200'
MethodResponses:
- StatusCode: '200'
PrivateDomainName:
Type: AWS::ApiGateway::DomainNameV2
Properties:
DomainName: !Ref DomainName
CertificateArn: !Ref CertificateArn
EndpointConfiguration:
Types:
- PRIVATE
SecurityPolicy: TLS\_1\_2
Policy:
Statement:
- Action: 'execute-api:Invoke'
Effect: Allow
Principal: '\*'
Resource: 'execute-api:/\*'
- Action: 'execute-api:Invoke'
Condition:
StringNotEquals:
'aws:SourceVpce': !Ref EndpointID
Effect: Deny
Principal: '\*'
Resource: 'execute-api:/\*'
Version: 2012-10-17
PrivateBasePathMapping:
Type: AWS::ApiGateway::BasePathMappingV2
DependsOn:
- PrivateApiStage
Properties:
BasePath: prod
DomainNameArn: !GetAtt PrivateDomainName.DomainNameArn
RestApiId: !Ref PrivateApi
Stage: prod
DomainNameAccessAssociation:
Type: AWS::ApiGateway::DomainNameAccessAssociation
Properties:
DomainNameArn: !GetAtt PrivateDomainName.DomainNameArn
AccessAssociationSource: !Ref EndpointID
AccessAssociationSourceType: VPCE`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
API consumer: Delete your domain name access association with a private custom domain name
Invoke a private
API
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.