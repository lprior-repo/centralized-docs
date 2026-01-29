---
url: https://docs.aws.amazon.com/apigateway/latest/api/API_CanarySettings.html
title: API CanarySettings.html
word_count: 109
filtered: true
elements_removed: 0
density_score: 0.92
---

CanarySettings - Amazon API Gateway
CanarySettings - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/api/apigw-api.pdf#API_CanarySettings)
[Contents](#API_CanarySettings_Contents)[See Also](#API_CanarySettings_SeeAlso)
## Contents
**
deploymentId
**
The ID of the canary deployment.
Type: String
Required: No
**
percentTraffic
**
The percent (0-100) of traffic diverted to a canary deployment.
Type: Double
Required: No
**
stageVariableOverrides
**
Stage variables overridden for a canary release deployment, including new stage variables introduced in the canary. These stage variables are represented as a string-to-string map between stage variable names and their values.
Type: String to string map
Required: No
**
useStageCache
**
A Boolean flag to indicate whether the canary deployment uses the stage cache or not.
Type: Boolean
Required: No