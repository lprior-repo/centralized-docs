---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/updating-api.html
title: Updates to REST APIs that require
word_count: 553
filtered: true
elements_removed: 0
density_score: 0.89
---

Updates to REST APIs that require redeployment - Amazon API Gateway
Updates to REST APIs that require redeployment - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#updating-api)
# Updates to REST APIs that require
redeployment
Maintaining an API amounts to viewing, updating and deleting the existing API setups. You
can maintain an API using the API Gateway console, AWS CLI, CloudFormation, an SDK or the API Gateway REST API. Updating
an API involves modifying certain resource properties or configuration settings of the API.
Resource updates require redeploying the API, where configuration updates do not.
The following table describes API resources that require redeployment of your API when you update them.
|Resource|Notes|
|[ApiKey](https://docs.aws.amazon.com/apigateway/latest/api/API_ApiKey.html)|For applicable properties and supported operations, see [apikey:update](https://docs.aws.amazon.com/apigateway/latest/api/API_UpdateApiKey.html). The update requires redeploying the API.|
|[Authorizer](https://docs.aws.amazon.com/apigateway/latest/api/API_Authorizer.html)|For applicable properties and supported operations, see [authorizer:update](https://docs.aws.amazon.com/apigateway/latest/api/API_UpdateAuthorizer.html). The update requires redeploying the
API.|
|[disableExecuteApiEndpoint](https://docs.aws.amazon.com/apigateway/latest/api/API_UpdateRestApi.html#apigw-UpdateRestApi-response-disableExecuteApiEndpoint)|The update requires modifying any stage on the API such as redeploying the API to a stage.|
|[DocumentationPart](https://docs.aws.amazon.com/apigateway/latest/api/API_DocumentationPart.html)|For applicable properties and supported operations, see [documentationpart:update](https://docs.aws.amazon.com/apigateway/latest/api/API_UpdateDocumentationPart.html). The update requires redeploying the
API.|
|[DocumentationVersion](https://docs.aws.amazon.com/apigateway/latest/api/API_DocumentationVersion.html)|For applicable properties and supported operations, see [documentationversion:update](https://docs.aws.amazon.com/apigateway/latest/api/API_UpdateDocumentationVersion.html). The update requires redeploying
the API.|
|[GatewayResponse](https://docs.aws.amazon.com/apigateway/latest/api/API_GatewayResponse.html)|For applicable properties and supported operations, see [gatewayresponse:update](https://docs.aws.amazon.com/apigateway/latest/api/API_UpdateGatewayResponse.html#remarks). The update requires redeploying the
API.|
|[Integration](https://docs.aws.amazon.com/apigateway/latest/api/API_Integration.html)|
For applicable properties and supported operations, see [integration:update](https://docs.aws.amazon.com/apigateway/latest/api/API_UpdateIntegration.html). The update requires redeploying the
API.
|
|[IntegrationResponse](https://docs.aws.amazon.com/apigateway/latest/api/API_IntegrationResponse.html)|For applicable properties and supported operations, see [integrationresponse:update](https://docs.aws.amazon.com/apigateway/latest/api/API_UpdateIntegrationResponse.html). The update requires redeploying the
API.|
|[Method](https://docs.aws.amazon.com/apigateway/latest/api/API_Method.html)|For applicable properties and supported operations, see [method:update](https://docs.aws.amazon.com/apigateway/latest/api/API_UpdateMethod.html). The update requires redeploying the API.|
|[MethodResponse](https://docs.aws.amazon.com/apigateway/latest/api/API_MethodResponse.html)|For applicable properties and supported operations, see [methodresponse:update](https://docs.aws.amazon.com/apigateway/latest/api/API_UpdateMethodResponse.html). The update requires redeploying the
API.|
|[Model](https://docs.aws.amazon.com/apigateway/latest/api/API_Model.html)|For applicable properties and supported operations, see [model:update](https://docs.aws.amazon.com/apigateway/latest/api/API_UpdateModel.html). The update requires redeploying the API.|
|[RequestValidator](https://docs.aws.amazon.com/apigateway/latest/api/API_RequestValidator.html)|For applicable properties and supported operations, see [requestvalidator:update](https://docs.aws.amazon.com/apigateway/latest/api/API_UpdateRequestValidator.html). The update requires redeploying the
API.|
|[Resource](https://docs.aws.amazon.com/apigateway/latest/api/API_Resource.html)|For applicable properties and supported operations, see [resource:update](https://docs.aws.amazon.com/apigateway/latest/api/API_UpdateResource.html). The update requires redeploying the
API.|
|[RestApi](https://docs.aws.amazon.com/apigateway/latest/api/API_UpdateRestApi.html)|For applicable properties and supported operations, see [restapi:update](https://docs.aws.amazon.com/apigateway/latest/api/API_UpdateRestApi.html). The update requires redeploying the
API. This includes modifying resource policies.|
|[VpcLink](https://docs.aws.amazon.com/apigateway/latest/api/API_VpcLink.html)|For applicable properties and supported operations, see [vpclink:update](https://docs.aws.amazon.com/apigateway/latest/api/API_UpdateVpcLink.html). The update requires redeploying the
API.|
The following table describes API configurations that don't require redeployment of your API when you update them.
|Configuration|Notes|
|[Account](https://docs.aws.amazon.com/apigateway/latest/api/API_GetAccount.html)|
For applicable properties and supported operations, see [account:update](https://docs.aws.amazon.com/apigateway/latest/api/API_UpdateAccount.html). The update does not require redeploying the
API.
|
|[Deployment](https://docs.aws.amazon.com/apigateway/latest/api/API_Deployment.html)|For applicable properties and supported operations, see [deployment:update](https://docs.aws.amazon.com/apigateway/latest/api/API_UpdateDeployment.html). |
|[DomainName](https://docs.aws.amazon.com/apigateway/latest/api/API_DomainName.html)|For applicable properties and supported operations, see [domainname:update](https://docs.aws.amazon.com/apigateway/latest/api/API_UpdateDomainName.html). The update does not require redeploying the
API.|
|[BasePathMapping](https://docs.aws.amazon.com/apigateway/latest/api/API_BasePathMapping.html)|
For applicable properties and supported operations, see [basepathmapping:update](https://docs.aws.amazon.com/apigateway/latest/api/API_UpdateBasePathMapping.html). The update does not require
redeploying the API.
|
|[IP address type](https://docs.aws.amazon.com/apigateway/latest/api/API_CreateRestApi.html)|
The update does not require redeploying the
API.
|
|[Stage](https://docs.aws.amazon.com/apigateway/latest/api/API_Stage.html)|
For applicable properties and supported operations, see [stage:update](https://docs.aws.amazon.com/apigateway/latest/api/API_UpdateStage.html). The update does not require redeploying the
API.
|
|[Usage](https://docs.aws.amazon.com/apigateway/latest/api/API_GetUsage.html)|
For applicable properties and supported operations, see [usage:update](https://docs.aws.amazon.com/apigateway/latest/api/API_UpdateUsage.html). The update does not require redeploying the
API.
|
|[UsagePlan](https://docs.aws.amazon.com/apigateway/latest/api/API_UsagePlan.html)|For applicable properties and supported operations, see [usageplan:update](https://docs.aws.amazon.com/apigateway/latest/api/API_UpdateUsagePlan.html). The update does not require redeploying the
API.|
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Turn off a canary release
Custom domain names
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.