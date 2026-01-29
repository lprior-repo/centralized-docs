---
url: https://docs.aws.amazon.com/apigateway/latest/api/patch-operations.html
title: Patch Operations
word_count: 2666
filtered: true
elements_removed: 0
density_score: 0.94
---

Patch Operations - Amazon API Gateway
Patch Operations - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/api/apigw-api.pdf#patch-operations)
[UpdateAccount](#UpdateAccount-Patch)[UpdateApiKey](#UpdateApiKey-Patch)[UpdateAuthorizer](#UpdateAuthorizer-Patch)[UpdateBasePathMapping](#UpdateBasePathMapping-Patch)[UpdateClientCertificate](#UpdateClientCertificate-Patch)[UpdateDeployment](#UpdateDeployment-Patch)[UpdateDocumentationPart](#UpdateDocumentationPart-Patch)[UpdateDocumentationVersion](#UpdateDocumentationVersion-Patch)[UpdateDomainName](#UpdateDomainName-Patch)[UpdateGatewayResponse](#UpdateGatewayResponse-Patch)[UpdateIntegration](#UpdateIntegration-Patch)[UpdateIntegrationResponse](#UpdateIntegrationResponse-Patch)[UpdateMethod](#UpdateMethod-Patch)[UpdateMethodResponse](#UpdateMethodResponse-Patch)[UpdateModel](#UpdateModel-Patch)[UpdateRequestValidator](#UpdateRequestValidator-Patch)[UpdateResource](#UpdateResource-Patch)[UpdateRestApi](#UpdateRestApi-Patch)[UpdateStage](#UpdateStage-Patch)[UpdateUsage](#UpdateUsage-Patch)[UpdateUsagePlan](#UpdateUsagePlan-Patch)[UpdateVpcLink](#UpdateVpcLink-Patch)
# Patch Operations
This section lists information about the supported patch operations to update resources.
## UpdateAccount
The following table shows the supported and unsupported `op` operations for
`UpdateAccount`.
|Path|op:add|op:replace|op:remove|op:copy|
|
`/cloudwatchRoleArn`
|
Not supported
|
Supported
|
Not supported
|
Not supported
|
|
`/features`
|
Supported
|
Not supported
|
Supported, but not for the `UsagePlans` feature
|
Not supported
|
## UpdateApiKey
The following table shows the supported and unsupported `op` operations for
`UpdateApiKey`.
|Path|op:add|op:replace|op:remove|op:copy|
|
`/customerId`
|
Not supported
|
Supported
|
Not supported
|
Not supported
|
|
`/description`
|
Not supported
|
Supported
|
Not supported
|
Not supported
|
|
`/enabled`
|
Not supported
|
Supported
|
Not supported
|
Not supported
|
|
`/labels`
|
Supported
|
Not supported
|
Supported
|
Not supported
|
|
`/name`
|
Not supported
|
Supported
|
Not supported
|
Not supported
|
|
`/stages`
|
Supported
|
Not supported
|
Supported
|
Not supported
|
## UpdateAuthorizer
The following table shows the supported and unsupported `op` operations for
`UpdateAuthorizer`.
|Path|op:add|op:replace|op:remove|op:copy|
|
`/authorizerUri`
|
Not supported
|
Supported
|
Not supported
|
Not supported
|
|
`/authorizerCredentials`
|
Not supported
|
Supported
|
Not supported
|
Not supported
|
|
`/authorizerResultTtlInSeconds`
|
Not supported
|
Supported
|
Not supported
|
Not supported
|
|
`/authType`
|
Not supported
|
Supported
|
Not supported
|
Not supported
|
|
`/identitySource`
|
Supported
|
Supported
|
Supported
|
Not supported
|
|
`/identityValidationExpression`
|
Not supported
|
Supported
|
Not supported
|
Not supported
|
|
`/name`
|
Not supported
|
Supported
|
Not supported
|
Not supported
|
|
`/providerARNs`
|
Supported
|
Not supported
|
Supported
|
Not supported
|
|
`/type`
|
Not supported
|
Supported
|
Not supported
|
Not supported
|
## UpdateBasePathMapping
The following table shows the supported and unsupported `op` operations for
`UpdateBasePathMapping`.
|Path|op:add|op:replace|op:remove|op:copy|
|
`/basepath`
|
Not supported
|
Supported
|
Not supported
|
Not supported
|
|
`/restapiId`
|
Not supported
|
Supported
|
Not supported
|
Not supported
|
|
`/stage`
|
Not supported
|
Supported
|
Not supported
|
Not supported
|
## UpdateClientCertificate
The following table shows the supported and unsupported `op` operations for
`UpdateClientCertificate`.
|Path|op:add|op:replace|op:remove|op:copy|
|
`/description`
|
Not supported
|
Supported
|
Not supported
|
Not supported
|
## UpdateDeployment
The following table shows the supported and unsupported `op` operations for
`UpdateDeployment`.
|Path|op:add|op:replace|op:remove|op:copy|
|
`/description`
|
Not supported
|
Supported
|
Not supported
|
Not supported
|
## UpdateDocumentationPart
The following table shows the supported and unsupported `op` operations for
`UpdateDocumentationPart`.
|Path|op:add|op:replace|op:remove|op:copy|
|
`/properties`
|
Not supported
|
Supported
|
Not supported
|
Not supported
|
## UpdateDocumentationVersion
The following table shows the supported and unsupported `op` operations for
`UpdateDocumentationVersion`.
|Path|op:add|op:replace|op:remove|op:copy|
|
`/description`
|
Not supported
|
Supported
|
Not supported
|
Not supported
|
## UpdateDomainName
The following table shows the supported and unsupported `op` operations for
`UpdateDomainName`.
|Path|op:add|op:replace|op:remove|op:copy|
|
`/certificateName`
|
Supported for adding an edge certificate while updating a regional
domain name to an edge-optimized one. This operation cannot be included
with the `remove` operation in the same request.
|
Supported for rotating an edge certificate of an edge-optimized domain
name.
|
Supported for removing an edge certificate while updating an
edge-optimized domain name to a regional one. This operation cannot be
included with the `add` operation in the same request.
|
Not supported
|
|
`/certificateArn`
|
Supported for adding an edge certificate while updating a regional
domain name to an edge-optimized one. This operation cannot be included
with the `remove` operation in the same request.
|
Supported for rotating an edge certificate of an edge-optimized domain
name.
|
Supported for removing an edge certificate while updating an
edge-optimized domain name to a regional one. This operation cannot be
included with the `add` operation in the same request.
|
Not supported
|
|
`/endpointConfiguration/types`
|
Supported for updates between edge-optimized and regional endpoints.
This operation cannot be included with the `remove` operation
in the same request.
|
Not supported
|
Supported for updates between edge-optimized and regional endpoints.
This operation cannot be included with the `add` operation in
the same request.
|
Not supported
|
|
`/endpointConfiguration/ipAddressType`
|
Not supported
|
Supported. Only `dualstack` and `ipv4` are supported.
|
Not supported
|
Not supported
|
|
`/endpointAccessMode`
|
Not supported
|
Supported only for custom domain names for public APIs.
|
Not supported
|
Not supported
|
|
`/managementPolicy`
|
Not supported
|
Supported only for custom domain names for private APIs.
|
Not supported
|
Not supported
|
|
`/mutualTlsAuthentication/truststoreUri`
|
Supported
|
Supported
|
Supported
|
Not supported
|
|
`/mutualTlsAuthentication/truststoreVersion`
|
Supported
|
Supported
|
Supported
|
Not supported
|
|
`/policy`
|
Not supported
|
Supported only for custom domain names for private APIs.
|
Not supported
|
Not supported
|
|
`/regionalCertificateName`
|
Supported for adding a regional certificate. This operation cannot be
included with the `remove` operation in the same
request.
|
Supported for rotating a regional certificate of a regional domain
name.
|
Supported for removing a regional certificate of an inactive API
endpoint. This operation cannot be included with the `add`
operation in the same request.
|
Not supported
|
|
`/regionalCertificateArn`
|
Supported for adding a regional certificate. This operation cannot be
included with the `remove` operation in the same
request.
|
Supported for rotating a regional certificate of a regional domain
name.
|
Supported for removing a regional certificate of an inactive API
endpoint. This operation cannot be included with the `add`
operation in the same request.
|
Not supported
|
|
`/routingMode`
|
Not supported
|
Supported
|
Not supported
|
Not supported
|
|
`/securityPolicy`
|
Not supported
|
Supported for migrating a domain name.
|
Not supported
|
Not supported
|
|
`/ownershipVerificationCertificateArn`
|
Supported
|
Supported
|
Supported
|
Not supported
|
## UpdateGatewayResponse
The following table shows the supported and unsupported `op` operations for
`UpdateGatewayResponse`.
|Path|op:add|op:replace|op:remove|op:copy|
|
`/statusCode`
|
Not supported
|
Supported
|
Not supported
|
Not supported
|
|
`/responseParameters`
|
Supported
|
Supported
|
Supported
|
Not supported
|
|
`/responseTemplates`
|
Supported
|
Supported
|
Supported
|
Not supported
|
## UpdateIntegration
The following table shows the supported and unsupported `op` operations for
`UpdateIntegration`.
|Path|op:add|op:replace|op:remove|op:copy|
|
`/cacheKeyParameters`
|
Supported
|
Supported
|
Supported
|
Not supported
|
|
`/cacheNamespace`
|
Not supported
|
Supported
|
Not supported
|
Not supported
|
|
`/connectionId`
|
Not supported
|
Supported
|
Not supported
|
Not supported
|
|
`/connectionType`
|
Not supported
|
Supported
|
Not supported
|
Not supported
|
|
`/contentHandling`
|
Not supported
|
Supported
|
Not supported
|
Not supported
|
|
`/httpMethod`
|
Not supported
|
Supported, except for `MOCK` integrations
|
Not supported
|
Not supported
|
|
`/integrationTarget`
|
Not supported
|
Supported
|
Not supported
|
Not supported
|
|
`/passthroughBehavior`
|
Not supported
|
Supported
|
Not supported
|
Not supported
|
|
`/requestParameters`
|
Supported
|
Supported
|
Supported
|
Not supported
|
|
`/requestTemplates`
|
Supported
|
Supported
|
Supported
|
Not supported
|
|
`/responseTransferMode`
|
Not supported
|
Supported
|
Not supported
|
Not supported
|
|
`/timeoutInMillis`
|
Not supported
|
Supported
|
Not supported
|
Not supported
|
|
`/tlsConfig/insecureSkipVerification`
|
Not supported
|
Supported
|
Not supported
|
Not supported
|
|
`/type`
|
Not supported
|
Not supported
|
Not supported
|
Not supported
|
|
`/uri`
|
Not supported
|
Supported, except for `MOCK` integrations
|
Not supported
|
Not supported
|
## UpdateIntegrationResponse
The following table shows the supported and unsupported `op` operations for
`UpdateIntegrationResponse`.
|Path|op:add|op:replace|op:remove|op:copy|
|
`/contentHandling`
|
Not supported
|
Supported
|
Not supported
|
Not supported
|
|
`/responseParameters`
|
Supported
|
Supported
|
Supported
|
Not supported
|
|
`/responseTemplates`
|
Supported
|
Supported
|
Supported
|
Not supported
|
|
`/selectionPattern`
|
Not supported
|
Supported
|
Not supported
|
Not supported
|
## UpdateMethod
The following table shows the supported and unsupported `op` operations for
`UpdateMethod`.
|Path|op:add|op:replace|op:remove|op:copy|
|
`/authorizationScopes`
|
Supported
|
Not supported
|
Supported
|
Not supported
|
|
`/authorizationType`
|
Not supported
|
Supported
|
Not supported
|
Not supported
|
|
`/authorizerId`
|
Not supported
|
Supported
|
Not supported
|
Not supported
|
|
`/apiKeyRequired`
|
Not supported
|
Supported
|
Not supported
|
Not supported
|
|
`/operationName`
|
Not supported
|
Supported
|
Not supported
|
Not supported
|
|
`/requestParameters`
|
Supported
|
Supported, except for `MOCK` integrations
|
Supported
|
Not supported
|
|
`/requestModels`
|
Supported
|
Supported
|
Supported
|
Not supported
|
|
`/requestValidatorId`
|
Not supported
|
Supported
|
Not supported
|
Not supported
|
## UpdateMethodResponse
The following table shows the supported and unsupported `op` operations for
`UpdateMethodResponse`.
|Path|op:add|op:replace|op:remove|op:copy|
|
`/responseModels`
|
Supported
|
Supported
|
Supported
|
Not supported
|
|
`/responseParameters`
|
Supported
|
Supported
|
Supported
|
Not supported
|
## UpdateModel
The following table shows the supported and unsupported `op` operations for
`UpdateModel`.
|Path|op:add|op:replace|op:remove|op:copy|
|
`/description`
|
Not supported
|
Supported
|
Not supported
|
Not supported
|
|
`/schema`
|
Not supported
|
Supported
|
Not supported
|
Not supported
|
## UpdateRequestValidator
The following table shows the supported and unsupported `op` operations for
`UpdateRequestValidator`.
|Path|op:add|op:replace|op:remove|op:copy|
|
`/name`
|
Not supported
|
Supported
|
Not supported
|
Not supported
|
|
`/validateRequestBody`
|
Not supported
|
Supported
|
Not supported
|
Not supported
|
|
`/validateRequestParameters`
|
Not supported
|
Supported
|
Not supported
|
Not supported
|
## UpdateResource
The following table shows the supported and unsupported `op` operations for
`UpdateResource`.
|Path|op:add|op:replace|op:remove|op:copy|
|
`/parentId`
|
Not supported
|
Supported
|
Not supported
|
Not supported
|
|
`/pathPart`
|
Not supported
|
Supported
|
Not supported
|
Not supported
|
## UpdateRestApi
The following table shows the supported and unsupported `op` operations for
`UpdateRestApi`.
|Path|op:add|op:replace|op:remove|op:copy|
|
`/apiKeySource`
|
Not supported
|
Supported
|
Not supported
|
Not supported
|
|
`/binaryMediaTypes`
|
Supported
|
Supported
|
Supported
|
Not supported
|
|
`/description`
|
Supported
|
Supported
|
Supported
|
Not supported
|
|
`/disableExecuteApiEndpoint`
|
Not supported
|
Supported
|
Not supported
|
Not supported
|
|
`/endpointConfiguration/types/{type}`
|
Not supported
|
Supported. Type must be `REGIONAL`, `EDGE`, or
`PRIVATE`.
|
Not supported
|
Not supported
|
|
`/endpointConfiguration/vpcEndpointIds`
|
Supported only for `PRIVATE` endpoint type.
|
Not supported
|
Supported only for `PRIVATE` endpoint type.
|
Not supported
|
|
`/endpointConfiguration/ipAddressType`
|
Not supported
|
Supported. Only `dualstack` and `ipv4` are supported.
|
Not supported
|
Not supported
|
|
`/endpointAccessMode`
|
Not supported
|
Supported
|
Not supported
|
Not supported
|
|
`/minimumCompressionSize \*`
|
Not supported
|
Supported
|
Not supported
|
Not supported
|
|
`/name`
|
Not supported
|
Supported
|
Not supported
|
Not supported
|
|
`/policy`
|
Not supported
|
Supported
|
Not supported
|
Not supported
|
|
`/securityPolicy`
|
Not supported
|
Supported
|
Not supported
|
Not supported
|
\* To enable compression, apply a `replace` operation with the accompanying
value property set to a non-negative integer between 0 and 10485760. To disable compression,
apply a `replace` operation with the `value` property set to null or
omit the `value` property.
## UpdateStage
The following table shows the supported and unsupported `op` operations for
`UpdateStage`.
|Path|op:add|op:replace|op:remove|op:copy|
|
`/accessLogSettings`
|
Not supported
|
Not supported
|
Supported
|
Not supported
|
|
`/accessLogSettings/\*`
|
Not supported
|
Supported
|
Not supported
|
Not supported
|
|
`/accessLogSettings/destinationArn`
|
Supported
|
Supported
|
Supported
|
Not supported
|
|
`/accessLogSettings/format`
|
Supported
|
Supported
|
Supported
|
Not supported
|
|
`/cacheClusterEnabled`
|
Not supported
|
Supported
|
Not supported
|
Not supported
|
|
`/cacheClusterSize`
|
Not supported
|
Supported
|
Not supported
|
Not supported
|
|
`/canarySettings`
|
Not supported
|
Not supported
|
Supported
|
Not supported
|
|
`/canarySettings/deploymentId`
|
Not supported
|
Supported
|
Not supported
|
Not supported
|
|
`/canarySettings/percentTraffic`
|
Not supported
|
Supported
|
Not supported
|
Not supported
|
|
`/canarySettings/stageVariableOverrides`
|
Not supported
|
Supported
|
Not supported
|
Not supported
|
|
`/canarySettings/useStageCache`
|
Not supported
|
Supported
|
Not supported
|
Not supported
|
|
`/clientCertificateId`
|
Not supported
|
Supported
|
Not supported
|
Not supported
|
|
`/deploymentId`
|
Not supported
|
Supported
|
Not supported
|
Supported
|
|
`/description`
|
Not supported
|
Supported
|
Not supported
|
Not supported
|
|
`/documentationVersion`
|
Not supported
|
Supported
|
Not supported
|
Not supported
|
|
`/methodSettings`
|
Not supported
|
Supported
|
Not supported
|
Not supported
|
|
`/variables`
|
Not supported
|
Supported
|
Supported
|
Not supported
|
|
`/variables/\*`
|
Not supported
|
Supported
|
Not supported
|
Not supported
|
|
`/tracingEnabled`
|
Not supported
|
Supported
|
Not supported
|
Not supported
|
|
`/{resourcePath}/{httpMethod}/metrics/enabled`
|
Not supported
|
Supported
|
Not supported
|
Not supported
|
|
`/{resourcePath}/{httpMethod}/logging/dataTrace`
|
Not supported
|
Supported
|
Not supported
|
Not supported
|
|
`/{resourcePath}/{httpMethod}/logging/loglevel`
|
Not supported
|
Supported
|
Not supported
|
Not supported
|
|
`/{resourcePath}/{httpMethod}/throttling/burstLimit`
|
Not supported
|
Supported
|
Not supported
|
Not supported
|
|
`/{resourcePath}/{httpMethod}/throttling/rateLimit`
|
Not supported
|
Supported
|
Not supported
|
Not supported
|
|
`/{resourcePath}/{httpMethod}/caching/enabled`
|
Not supported
|
Supported
|
Not supported
|
Not supported
|
|
`/{resourcePath}/{httpMethod}/caching/dataEncrypted`
|
Not supported
|
Supported
|
Not supported
|
Not supported
|
|
`/{resourcePath}/{httpMethod}/caching/ttlInSeconds`
|
Not supported
|
Supported
|
Not supported
|
Not supported
|
|
`/{resourcePath}/{httpMethod}/caching/requireAuthorizationForCacheControl`
|
Not supported
|
Supported
|
Not supported
|
Not supported
|
|
`/{resourcePath}/{httpMethod}/caching/unauthorizedCacheControlHeaderStrategy`
|
Not supported
|
Supported
|
Not supported
|
Not supported
|
## UpdateUsage
The following table shows the supported and unsupported `op` operations for
`UpdateUsage`.
|Path|op:add|op:replace|op:remove|op:copy|
|
`/remaining`
|
Not supported
|
Supported
|
Not supported
|
Not supported
|
## UpdateUsagePlan
The following table shows the supported and unsupported `op` operations for
`UpdateUsagePlan`.
|Path|op:add|op:replace|op:remove|op:copy|
|
`/apiStages`
|
Supported
|
Not supported
|
Supported
|
Not supported
|
|
`/apiStages/{apidId:stageName}/throttle/{resourcePath}/{httpMethod}`
|
Not supported
|
Not supported
|
Supported
|
Not supported
|
|
`/apiStages/{apidId:stageName}/throttle/{resourcePath}/{httpMethod}/rateLimit`
|
Supported
|
Supported
|
Not supported
|
Not supported
|
|
`/apiStages/{apidId:stageName}/throttle/{resourcePath}/{httpMethod}/burstLimit`
|
Supported
|
Supported
|
Not supported
|
Not supported
|
|
`/apiStages/{apidId:stageName}/throttle`
|
Supported
|
Supported
|
Supported
|
Not supported
|
|
`/description`
|
Not supported
|
Supported
|
Not supported
|
Not supported
|
|
`/name`
|
Not supported
|
Supported
|
Not supported
|
Not supported
|
|
`/productCode`
|
Supported
|
Supported
|
Supported
|
Not supported
|
|
`/quota`
|
Not supported
|
Not supported
|
Supported
|
Not supported
|
|
`/quota/limit`
|
Supported
|
Supported
|
Not supported
|
Not supported
|
|
`/quota/offset`
|
Supported
|
Supported
|
Not supported
|
Not supported
|
|
`/quota/period`
|
Supported
|
Supported
|
Not supported
|
Not supported
|
|
`/throttle`
|
Not supported
|
Not supported
|
Supported
|
Not supported
|
|
`/throttle/burstLimit`
|
Supported
|
Supported
|
Not supported
|
Not supported
|
|
`/throttle/rateLimit`
|
Supported
|
Supported
|
Not supported
|
Not supported
|
## UpdateVpcLink
The following table shows the supported and unsupported `op` operations for
`UpdateVpcLink`.
|Path|op:add|op:replace|op:remove|op:copy|
|
`/name`
|
Not supported
|
Supported
|
Not supported
|
Not supported
|
|
`/description`
|
Not supported
|
Supported
|
Not supported
|
Not supported
|
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Common Errors
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.