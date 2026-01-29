---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-set-up-websocket-deployment.html
title: Deploy WebSocket APIs in
word_count: 719
filtered: true
elements_removed: 0
density_score: 0.84
---

Deploy WebSocket APIs in API Gateway - Amazon API Gateway
Deploy WebSocket APIs in API Gateway - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#apigateway-set-up-websocket-deployment)
[Create a WebSocket
API deployment using the AWS CLI](#apigateway-create-websocket-deployment-using-awscli)[Create a
WebSocket API deployment using the API Gateway console](#apigateway-create-websocket-deployment-using-console)
# Deploy WebSocket APIs in
API Gateway
After creating your WebSocket API, you must deploy it to make it available for your users
to invoke.
To deploy an API, you create an [API
deployment](./api-gateway-basic-concept.html#apigateway-definition-api-deployment) and associate it with a [stage](./api-gateway-basic-concept.html#apigateway-definition-api-stage). Each stage is a snapshot of the
API and is made available for client apps to call.
###### Important
Every time you update an API, you must redeploy it. Changes to anything other than stage settings require a
redeployment, including modifications to the following resources:
* Routes
* Integrations
* Authorizers
By default you are limited to 10 stages for each API. We recommend that you re-use stages for your deployments.
To call a deployed WebSocket API, the client sends a message to the API's URL. The
URL is determined by the API's hostname and stage name.
###### Note
API Gateway will support payloads up to 128 KB with a maximum frame size of 32
KB. If a message exceeds 32 KB, it must be split into multiple frames, each
32 KB or smaller.
Using the API's default domain name, the URL of (for example) a WebSocket API in a given
stage (``{stageName}``) is in the following
format:
```
`wss://`{api-id}`.execute-api.`{region}`.amazonaws.com/`{stageName}``
```
To make the WebSocket API's URL more user-friendly, you can create a custom domain name
(e.g., `api.example.com`) to replace the default host name of the API. The
configuration process is the same as for REST APIs. For more information, see [Custom domain name for public
REST APIs in API Gateway](./how-to-custom-domains.html).
Stages enable robust version control of your API. For example, you can deploy an API to a
`test` stage and a `prod` stage, and use the `test`
stage as a test build and use the `prod` stage as a stable build. After the
updates pass the test, you can promote the `test` stage to the `prod`
stage. The promotion can be done by redeploying the API to the `prod`
stage. For more details about stages, see [Set up a stage for a REST API in API Gateway](./set-up-stages.html).
###### Topics
* [Create a WebSocket
API deployment using the AWS CLI](#apigateway-create-websocket-deployment-using-awscli)
* [Create a
WebSocket API deployment using the API Gateway console](#apigateway-create-websocket-deployment-using-console)
## Create a WebSocket
API deployment using the AWS CLI
The following [create-deployment](https://docs.aws.amazon.com/cli/latest/reference/apigatewayv2/create-deployment.html) command creates a deployment:
```
`aws apigatewayv2 --region us-east-1 create-deployment --api-id aabbccddee`
```
The output will look like the following:
```
`{
"DeploymentId": "fedcba",
"DeploymentStatus": "DEPLOYED",
"CreatedDate": "2018-11-15T06:49:09Z"
}`
```
The deployed API is not callable until you associate the deployment with a stage. You
can create a new stage or reuse a stage that you have previously created.
The following [create-stage](https://docs.aws.amazon.com/cli/latest/reference/apigatewayv2/create-stage.html) command creates a new stage and associates it with the deployment:
```
`aws apigatewayv2 --region us-east-1 create-stage --api-id aabbccddee --deployment-id fedcba --stage-name test`
```
The output looks like the following:
```
`{
"StageName": "test",
"CreatedDate": "2018-11-15T06:50:28Z",
"DeploymentId": "fedcba",
"DefaultRouteSettings": {
"MetricsEnabled": false,
"ThrottlingBurstLimit": 5000,
"DataTraceEnabled": false,
"ThrottlingRateLimit": 10000.0
},
"LastUpdatedDate": "2018-11-15T06:50:28Z",
"StageVariables": {},
"RouteSettings": {}
}`
```
You can also reuse an existing stage by updating the stage's `deploymentId` property with the newly
created deployment ID (`deployment-id`). The following [update-stage](https://docs.aws.amazon.com/cli/latest/reference/apigatewayv2/update-stage.html) command updates
the stage's deployment ID:
```
`aws apigatewayv2 update-stage --region `region` \\
--api-id `api-id` \\
--stage-name `stage-name` \\
--deployment-id `deployment-id``
```
## Create a
WebSocket API deployment using the API Gateway console
To use the API Gateway console to create a deployment for a WebSocket API:
1. Sign in to the API Gateway console and choose the API.
2. Choose **Deploy
API**.
3. Choose the desired stage from the dropdown list or enter the name of a new
stage.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Stages
Security policy for WebSocket APIs in API Gateway
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.