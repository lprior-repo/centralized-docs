---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/create-canary-deployment.html
title: Create a canary release deployment
word_count: 1291
filtered: true
elements_removed: 0
density_score: 0.78
---

Create a canary release deployment - Amazon API Gateway
Create a canary release deployment - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#create-canary-deployment)
[Create a canary deployment using the API Gateway console](#create-canary-deployment-using-console)[Create a canary deployment
using the AWS CLI](#create-canary-deployment-using-cli)
# Create a canary release deployment
You create a canary release deployment when deploying the API with [canary
settings](https://docs.aws.amazon.com/apigateway/latest/api/API_CreateDeployment.html#canarySettings) as an additional input to the [deployment creation](https://docs.aws.amazon.com/apigateway/latest/api/API_CreateDeployment.html)
operation.
You can also create a canary release deployment from an existing non-canary deployment
by making a [`stage:update`](https://docs.aws.amazon.com/apigateway/latest/api/API_UpdateStage.html) request to add the canary settings on the
stage.
When creating a non-canary release deployment, you can specify a non-existing stage
name. API Gateway creates one if the specified stage does not exist. However, you cannot
specify any non-existing stage name when creating a canary release deployment. You will
get an error and API Gateway will not create any canary release deployment.
You can create a canary release deployment in API Gateway using the API Gateway console, the AWS CLI,
or an AWS SDK.
###### Topics
* [Create a canary deployment using the API Gateway console](#create-canary-deployment-using-console)
* [Create a canary deployment
using the AWS CLI](#create-canary-deployment-using-cli)
## Create a canary deployment using the API Gateway console
To use the API Gateway console to create a canary release deployment, follow the
instructions below:
###### To create the initial
canary release deployment
1. Sign in to the API Gateway console.
2. Choose an existing REST API or create a new REST API.
3. In the main navigation pane, choose **Resources**, and then choose **Deploy API**.
Follow the on-screen
instructions in **Deploy API** to deploy the API to a new
stage.
So far, you have deployed the API to a production release stage. Next, you
configure canary settings on the stage and, if needed, also enable caching,
set stage variables, or configure API execution or access logs.
4. To enable API caching or associate an AWS WAF web ACL with the stage, in the **Stage details** section, choose **Edit**.
For more information, see [Cache settings for REST APIs in API Gateway](./api-gateway-caching.html) or [To associate an AWS WAF
web ACL with an API Gateway API stage using the API Gateway console](./apigateway-control-access-aws-waf.html#apigateway-control-access-aws-waf-console).
5. To configure execution or access logging, in the **Logs and tracing** section, choose
**Edit** and follow the on-screen instructions. For more information, see [Set up CloudWatch logging for REST APIs in API Gateway](./set-up-logging.html).
6. To set stage variables, choose the **Stage variables**
tab and follow the on-screen
instructions to add or modify stage variables. For more information, see
[Use stage variables for a REST API in API Gateway](./stage-variables.html).
7. Choose the **Canary** tab, and then choose **Create
canary**. You might need to choose the right arrow button to show the **Canary** tab.
8. Under **Canary settings**, for **Canary**, enter the percentage of requests to be diverted to the
canary.
9. If desired, select **Stage cache** to
turn on caching for the canary release. The cache is not
available for the canary release until API caching is enabled.
10. To override existing stage variables, for **Canary override**, enter a new stage variable value.
After the canary release is initialized on the deployment stage, you change the
API and want to test the changes. You can redeploy the API to the same stage so that
both the updated version and the base version are accessible through the same stage.
The following steps describe how to do that.
###### To deploy the latest API
version to a canary
1. With each update of the API, choose **Deploy API**.
2. In **Deploy API**, choose the stage that contains a canary from the
**Deployment stage** dropdown list.
3. (Optional) Enter a description for **Deployment
description**.
4. Choose **Deploy** to push the latest API version to the
canary release.
5. If desired, reconfigure the stage settings, logs, or canary settings, as
describe in [To create the initial
canary release deployment](#to-create-canary-release-on-new-deployment).
As a result, the canary release points to the latest version while the production
release still points to the initial version of the API. The [**canarySettings**](https://docs.aws.amazon.com/apigateway/latest/api/API_Stage.html#canarySettings) now has a new
**deploymentId** value, whereas the stage still has the initial
[**deploymentId**](https://docs.aws.amazon.com/apigateway/latest/api/API_Stage.html#deploymentId) value. Behind the scenes, the
console calls [**stage:update**](https://docs.aws.amazon.com/apigateway/latest/api/API_UpdateStage.html).
###### To create a canary deployment for a new stage
1. Use the following [create-deployment](https://docs.aws.amazon.com/cli/latest/reference/apigateway/create-deployment.html) command
to create a deployment with two stage variables, but without a canary:
```
`aws apigateway create-deployment \\
--variables sv0=val0,sv1=val1 \\
--rest-api-id abcd1234 \\
--stage-name 'prod'`
```
The output will look like the following:
```
`{
"id": "du4ot1",
"createdDate": 1511379050
}`
```
2. Use the following [create-deployment](https://docs.aws.amazon.com/cli/latest/reference/apigateway/create-deployment.html) command
to create a canary deployment on the `prod` stage:
```
`aws apigateway create-deployment \\
--rest-api-id abcd1234 \\
--canary-settings percentTraffic=10.5,stageVariableOverrides={sv1='val2',sv2='val3'},useStageCache=false \\
--stage-name 'prod'`
```
The output will look like the following:
```
`{
"id": "a6rox0",
"createdDate": 1511379433
}`
```
The resulting deployment `id` identifies the test version of the API
for the canary release. As a result, the associated stage is canary-enabled.
3. (Optional) Use the following [get-stage](https://docs.aws.amazon.com/cli/latest/reference/apigateway/get-stage.html) command to view the stage representation:
```
`aws apigateway get-stage --rest-api-id acbd1234 --stage-name prod`
```
The following shows a representation of the `Stage` as the output of
the command:
```
`{
"stageName": "prod",
"variables": {
"sv0": "val0",
"sv1": "val1"
},
"cacheClusterEnabled": false,
"cacheClusterStatus": "NOT\_AVAILABLE",
"deploymentId": "du4ot1",
"lastUpdatedDate": 1511379433,
"createdDate": 1511379050,
"canarySettings": {
"percentTraffic": 10.5,
"deploymentId": "a6rox0",
"useStageCache": false,
"stageVariableOverrides": {
"sv2": "val3",
"sv1": "val2"
}
},
"methodSettings": {}
}`
```
In this example, the base version of the API will use the stage variables of
`{"sv0":val0", "sv1":val1"}`, while the test version uses the stage
variables of `{"sv1":val2", "sv2":val3"}`. Both the production release
and canary release use the same stage variable of `sv1`, but with
different values, `val1` and `val2`, respectively. The stage
variable of `sv0` is used solely in the production release and the stage
variable of `sv2` is used in the canary release only.
You can also create a canary release deployment from an existing regular deployment by updating the stage to
enable a canary.
###### To create a canary release deployment from an existing deployment
1. Use the [create-deployment](https://docs.aws.amazon.com/cli/latest/reference/apigateway/create-deployment.html) command to
create a deployment without a canary:
```
`aws apigateway create-deployment \\
--variables sv0=val0,sv1=val1 \\
--rest-api-id abcd1234 \\
--stage-name 'beta'`
```
2. Use the [update-stage](https://docs.aws.amazon.com/cli/latest/reference/apigateway/update-stage.html) command to update
the stage to enable a canary:
```
`aws apigateway update-stage \\
--rest-api-id abcd1234 \\
--stage-name 'beta' \\
--patch-operations '[{
"op": "replace",
"value": "0.0",
"path": "/canarySettings/percentTraffic"
}, {
"op": "copy",
"from": "/canarySettings/stageVariableOverrides",
"path": "/variables"
}, {
"op": "copy",
"from": "/canarySettings/deploymentId",
"path": "/deploymentId"
}]'`
```
The output will look like the following:
```
`{
"stageName": "beta",
"variables": {
"sv0": "val0",
"sv1": "val1"
},
"cacheClusterEnabled": false,
"cacheClusterStatus": "NOT\_AVAILABLE",
"deploymentId": "**cifeiw**",
"lastUpdatedDate": 1511381930,
"createdDate": 1511380879,
"canarySettings": {
"percentTraffic": 10.5,
"deploymentId": "**cifeiw**",
"useStageCache": false,
"stageVariableOverrides": {
"sv2": "val3",
"sv1": "val2"
}
},
"methodSettings": {}
}`
```
Because you enabled a canary on an existing version of the API, both the
production release (`Stage`) and canary release
(`canarySettings`) point to the same deployment. After you change the API and deploy
it to this stage again, the new version will be in the canary release, while the
base version remains in the production release. This is manifested in the stage
evolution when the `deploymentId` in the canary release is updated to the
new deployment `id` and the `deploymentId` in the production
release remains unchanged.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Set up a canary release deployment
Update a canary release
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.