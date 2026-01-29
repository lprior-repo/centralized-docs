---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/set-up-deployments.html
title: Create a deployment for a REST API in API Gateway
word_count: 966
filtered: true
elements_removed: 0
density_score: 0.83
---

Create a deployment for a REST API in API Gateway - Amazon API Gateway
Create a deployment for a REST API in API Gateway - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#set-up-deployments)
[Create a deployment](#create-deployment)[Next steps for your API deployment](#apigateway-deployment-next-steps)
# Create a deployment for a REST API in API Gateway
In API Gateway, a REST API deployment is represented by a [Deployment](https://docs.aws.amazon.com/apigateway/latest/api/API_Deployment.html) resource. It's similar to an
executable of an API that is represented by a [RestApi](https://docs.aws.amazon.com/apigateway/latest/api/API_RestApi.html) resource.
For the client to call your API, you must create a deployment and associate a stage with
it. A stage is represented by a [Stage](https://docs.aws.amazon.com/apigateway/latest/api/API_Stage.html)
resource. It represents a snapshot of the API, including methods, integrations, models,
mapping templates, and Lambda authorizers (formerly known as custom authorizers). When you
update the API, you can redeploy the API by associating a new stage with the existing
deployment. We discuss creating a stage in [Set up a stage for a REST API in API Gateway](./set-up-stages.html).
###### Topics
* [Create a deployment](#create-deployment)
* [Next steps for your API deployment](#apigateway-deployment-next-steps)
## Create a deployment
The following procedures show how to create a deployment for a REST API.
AWS Management Console
You must have created a REST API before deploying it for the first time. For more
information, see [Develop REST APIs in API Gateway](./rest-api-develop.html).
The API Gateway console lets you deploy an API by creating a deployment and associating it with a new or existing stage.
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2.
In the **APIs** navigation pane, choose the API you want to deploy.
3. In the **Resources** pane, choose **Deploy API**.
4. For **Stage**, select from the following:
1. To create a new stage, select **New stage**, and then enter a name in
**Stage name**. Optionally, you can provide a description for the
deployment in **Deployment description**.
2. To choose an existing stage, select the stage name from the dropdown menu. You might want to provide a
description of the new deployment in **Deployment description**.
3. To create a deployment that is not associated with a stage, select **No stage**. Later,
you can associate this deployment with a stage.
4. Choose **Deploy**.
AWS CLI
When you create a deployment, you instantiate the [Deployment](https://docs.aws.amazon.com/apigateway/latest/api/API_Deployment.html) resource.
The following [create-deployment](https://docs.aws.amazon.com/cli/latest/reference/apigateway/create-deployment.html)
command creates a new deployment:
```
` aws apigateway create-deployment --rest-api-id `rest-api-id``
```
You can't call the API until you associate this deployment with a stage. With an existing stage, you can
do this by updating the stage's [deploymentId](https://docs.aws.amazon.com/apigateway/latest/api/API_Stage.html#deploymentId)
property with the newly created deployment ID. The following [update-stage](https://docs.aws.amazon.com/cli/latest/reference/apigateway/update-stage.html) command updates the stage with a new
deployment. In the console, this is called the **Active deployment**.
```
` aws apigateway update-stage \\
--rest-api-id `rest-api-id` \\
--stage-name '`stage-name`' \\
--patch-operations op='replace',path='/deploymentId',value='`deployment-id`'`
```
When you create your deployment, you can also associate it with a new stage at the same time. The
following [create-deployment](https://docs.aws.amazon.com/cli/latest/reference/apigateway/create-deployment.html) command
creates a new deployment and associates it with a new stage called `beta`:
```
` aws apigateway create-deployment \\
--rest-api-id `rest-api-id` \\
--stage-name beta`
```
To redeploy an API, perform the same steps. You can reuse the same
stage.
## Next steps for your API deployment
The following are next steps for your API deployment.
Modify stage settings
After an API is deployed, you can modify the stage settings to enable or disable the API cache, logging,
or request throttling. You can also choose a client certificate for the backend to authenticate API Gateway and
set stage variables to pass deployment context to the API integration at runtime. For more information, see
[Modify stage settings](./set-up-stages.html#how-to-stage-settings)
After modifying stage settings, you must redeploy the API for the changes to take effect.
###### Note
If the updated settings, such as enabling logging, requires a new IAM role, you
can add the required IAM role without redeploying the API. However, it might take a
few minutes before the new IAM role takes effect. Before that happens, traces of
your API calls are not logged even if you have enabled the logging option.
Choose different deployment-stage combinations
Because a deployment represents an API snapshot and a stage defines a path into a snapshot, you
can choose different deployment-stage combinations to control how users call into different versions of the API.
This is useful, for example, when you want to roll back API state to a previous deployment or to merge
a 'private branch' of the API into the public one.
The following procedure shows how to do this using the **Stage Editor** in the API Gateway console.
It is assumed that you must have deployed an API more than once.
1. If you're not already on the **Stages** pane, in the main navigation pane, choose **Stages**.
2. Select the stage you want to update.
3. On the **Deployment history** tab, select the deployment you want the stage to use.
4. Choose **Change active deployment**.
5. Confirm you want to change the active deployment and choose **Change active deployment** in the **Make active deployment** dialog box.
Pass deployment-specific
data to your API.
For a deployment, you can set or modify stage variables to pass deployment-specific
data to the API integration at runtime. You can do this on the **Stage
Variables** tab in the **Stage Editor**. For more
information, see instructions in [Use stage variables for a REST API in API Gateway](./stage-variables.html).
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Deploy REST APIs
Set up a stage
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.