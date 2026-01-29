---
url: https://docs.aws.amazon.com/apigateway/latest/api/API_Stage.html
title: Stage
word_count: 436
filtered: true
elements_removed: 0
density_score: 0.80
---

Stage - Amazon API Gateway
Stage - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/api/apigw-api.pdf#API_Stage)
[Contents](#API_Stage_Contents)[See Also](#API_Stage_SeeAlso)
# Stage
Represents a unique identifier for a version of a deployed RestApi that is callable by users.
## Contents
**
accessLogSettings
**
Settings for logging access in this stage.
Type: [AccessLogSettings](./API_AccessLogSettings.html) object
Required: No
**
cacheClusterEnabled
**
Specifies whether a cache cluster is enabled for the stage. To activate a method-level cache, set `CachingEnabled` to `true` for a method.
Type: Boolean
Required: No
**
cacheClusterSize
**
The stage's cache capacity in GB. For more information about choosing a cache size, see [Enabling API caching to enhance responsiveness](https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-caching.html).
Type: String
Valid Values: `0.5 | 1.6 | 6.1 | 13.5 | 28.4 | 58.2 | 118 | 237`
Required: No
**
cacheClusterStatus
**
The status of the cache cluster for the stage, if enabled.
Type: String
Valid Values: `CREATE\_IN\_PROGRESS | AVAILABLE | DELETE\_IN\_PROGRESS | NOT\_AVAILABLE | FLUSH\_IN\_PROGRESS`
Required: No
**
canarySettings
**
Settings for the canary deployment in this stage.
Type: [CanarySettings](./API_CanarySettings.html) object
Required: No
**
clientCertificateId
**
The identifier of a client certificate for an API stage.
Type: String
Required: No
**
createdDate
**
The timestamp when the stage was created.
Type: Timestamp
Required: No
**
deploymentId
**
The identifier of the Deployment that the stage points to.
Type: String
Required: No
**
description
**
The stage's description.
Type: String
Required: No
**
documentationVersion
**
The version of the associated API documentation.
Type: String
Required: No
**
lastUpdatedDate
**
The timestamp when the stage last updated.
Type: Timestamp
Required: No
**
methodSettings
**
A map that defines the method settings for a Stage resource. Keys (designated as `/{method\_setting\_key` below) are method paths defined as `{resource\_path}/{http\_method}` for an individual method override, or `/\\\*/\\\*` for overriding all methods in the stage.
Type: String to [MethodSetting](./API_MethodSetting.html) object map
Required: No
**
stageName
**
The name of the stage is the first path segment in the Uniform Resource Identifier (URI) of a call to API Gateway. Stage names can only contain alphanumeric characters, hyphens, and underscores. Maximum length is 128 characters.
Type: String
Required: No
**
tags
**
The collection of tags. Each tag element is associated with a given resource.
Type: String to string map
Required: No
**
tracingEnabled
**
Specifies whether active tracing with X-ray is enabled for the Stage.
Type: Boolean
Required: No
**
variables
**
A map that defines the stage variables for a Stage resource. Variable names can
have alphanumeric and underscore characters, and the values must match `[A-Za-z0-9-.\_\~:/?#&amp;&amp;=,]+`.
Type: String to string map
Required: No
**
webAclArn
**
The ARN of the WebAcl associated with the Stage.
Type: String
Required: No