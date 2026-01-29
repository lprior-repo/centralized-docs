---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/delete-canary-deployment.html
title: Turn off a canary release
word_count: 394
filtered: true
elements_removed: 0
density_score: 0.83
---

Turn off a canary release - Amazon API Gateway
Turn off a canary release - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#delete-canary-deployment)
[Turn off a canary release using the
API Gateway console](#delete-canary-release-console)[Turn off a canary release using the
AWS CLI](#delete-canary-release-cli)
# Turn off a canary release
To turn off a canary release deployment is to set the [`canarySettings`](https://docs.aws.amazon.com/apigateway/latest/api/API_Stage.html#canarySettings) to null to remove it from the stage.
You can disable a canary release deployment using the API Gateway console, the AWS CLI, or an AWS
SDK.
###### Topics
* [Turn off a canary release using the
API Gateway console](#delete-canary-release-console)
* [Turn off a canary release using the
AWS CLI](#delete-canary-release-cli)
## Turn off a canary release using the
API Gateway console
To use the API Gateway console to turn off a canary release deployment, use the following
steps:
###### To turn off a canary release deployment
1. Sign in to the API Gateway console and choose an existing API in the main
navigation pane.
2. In the main navigation pane, choose **Stages**, and then choose an existing stage.
3. Choose the **Canary** tab.
4. Choose **Delete**.
5. Confirm you want to delete the canary by choosing
**Delete**.
As a result, the [`canarySettings`](https://docs.aws.amazon.com/apigateway/latest/api/API_Stage.html#canarySettings) property becomes `null` and
is removed from the deployment [stage](https://docs.aws.amazon.com/apigateway/latest/api/API_Stage.html). You can verify this using the AWS CLI. For
example, see [Turn off a canary release using the
AWS CLI](#delete-canary-release-cli).
## Turn off a canary release using the
AWS CLI
The following [update-stage](https://docs.aws.amazon.com/cli/latest/reference/apigateway/update-stage.html) command turns off
the canary release deployment:
```
`aws apigateway update-stage \\
--rest-api-id abcd1234 \\
--stage-name canary \\
--patch-operations '[{"op":"remove", "path":"/canarySettings"}]'`
```
The output looks like the following:
```
`{
"stageName": "prod",
"accessLogSettings": {
...
},
"cacheClusterEnabled": false,
"cacheClusterStatus": "NOT\_AVAILABLE",
"deploymentId": "nfcn0x",
"lastUpdatedDate": 1511309280,
"createdDate": 1511152939,
"methodSettings": {
...
}
}`
```
As shown in the output, the [canarySettings](https://docs.aws.amazon.com/apigateway/latest/api/API_Stage.html#canarySettings) property is no longer present in the [stage](https://docs.aws.amazon.com/apigateway/latest/api/API_Stage.html) of a canary-disabled
deployment.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Promote a canary release
Redeploy a REST API
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.