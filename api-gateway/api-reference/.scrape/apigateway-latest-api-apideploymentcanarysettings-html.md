---
url: https://docs.aws.amazon.com/apigateway/latest/api/API_DeploymentCanarySettings.html
title: API DeploymentCanarySettings.html
word_count: 107
filtered: true
elements_removed: 0
density_score: 0.93
---

DeploymentCanarySettings - Amazon API Gateway
DeploymentCanarySettings - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/api/apigw-api.pdf#API_DeploymentCanarySettings)
[Contents](#API_DeploymentCanarySettings_Contents)[See Also](#API_DeploymentCanarySettings_SeeAlso)
## Contents
**
percentTraffic
**
The percentage (0.0-100.0) of traffic routed to the canary deployment.
Type: Double
Required: No
**
stageVariableOverrides
**
A stage variable overrides used for the canary release deployment. They can override existing stage variables or add new stage variables for the canary release deployment. These stage variables are represented as a string-to-string map between stage variable names and their values.
Type: String to string map
Required: No
**
useStageCache
**
A Boolean flag to indicate whether the canary release deployment uses the stage cache or not.
Type: Boolean
Required: No