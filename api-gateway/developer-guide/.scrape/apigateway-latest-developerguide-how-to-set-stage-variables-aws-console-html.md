---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/how-to-set-stage-variables-aws-console.html
title: Set up stage variables for REST APIs in API Gateway
word_count: 1997
filtered: true
elements_removed: 0
density_score: 0.81
---

Set up stage variables for REST APIs in API Gateway - Amazon API Gateway
Set up stage variables for REST APIs in API Gateway - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#how-to-set-stage-variables-aws-console)
[Prerequisites](#how-to-set-stage-variables-aws-console-prerequisites)[Invoke an HTTP endpoint through an API with a stage
variable](#how-to-set-stage-variables-aws-console-http-endpoint)[Pass
stage-specific metadata into an HTTP backend](#how-to-set-stage-variables-aws-console-stage-metadata)[Invoke a Lambda function through an API
with a stage variable](#how-to-set-stage-variables-aws-console-lambda-function)[Pass stage-specific metadata to a
Lambda function through a stage variable](#pass-version-info-to-lambda-backend-with-stage-variable)
# Set up stage variables for REST APIs in API Gateway
This section shows how to set up various stage variables for two deployment stages of a sample API by using
the Amazon API Gateway console. To understand how to use stage variables in API Gateway, we recommend that you follow all
procedures in this section.
## Prerequisites
Before you begin, make sure the following prerequisites are met:
* You must have an API available in API Gateway. Follow the instructions in [Develop REST APIs in API Gateway](./rest-api-develop.html).
* You must have deployed the API at least once. Follow the instructions in [Deploy REST APIs in API Gateway](./how-to-deploy-api.html).
* You must have created the first stage for a deployed API. Follow the instructions in [Create a new stage](./set-up-stages.html#how-to-create-stage-console).
## Invoke an HTTP endpoint through an API with a stage
variable
This procedure describes how to create a stage variable for an HTTP endpoint and two stages for your API. In
addition, you create the stage variables, `url`, `stageName`, and
`function` that are used in the following procedures in this section.
###### To invoke an HTTP endpoint through an API with a stage
variable
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. Create an API, and then create a `GET` method on the API's root resource. Set the integration type to
**HTTP** and set the **Endpoint URL** to `http://${stageVariables.url}`.
3. Deploy the API to a new stage named `beta`.
4. In the main navigation pane, choose **Stages**, and then select the **beta** stage.
5. On the **Stage variables** tab, choose **Edit**.
6. Choose **Add stage variable**.
7. For **Name**, enter `url`. For **value**, enter
`httpbin.org/get`.
8. Choose **Add stage variable**, and then do the following:
For **Name**, enter `stageName`. For **value**, enter
`beta`.
9. Choose **Add stage variable**, and then do the following:
For **Name**, enter `function`. For **value**, enter
`HelloWorld`.
10. Choose **Save**.
11. Now create a second stage. From the **Stages** navigation pane, choose
**Create stage**. For **Stage
name**, enter `prod`. Select a recent deployment from
**Deployment**, and then choose **Create stage**.
12. As with the **beta** stage, set the same three stage variables
(**url**, **stageName**, and **function**) to different
values (`petstore-demo-endpoint.execute-api.com/petstore/pets`, `prod`, and
`HelloEveryone`), respectively.
13. In the **Stages** navigation pane, choose **beta**. Under
**Stage details**, choose the copy icon to copy your API's invoke URL, and then enter
your API's invoke URL in a web browser. This starts the **beta** stage `GET`
request on the root resource of the API.
###### Note
The **Invoke URL** link points to the root resource of the API in its
**beta** stage. Entering the URL in a web browser calls the
**beta** stage `GET` method on the root resource. If methods are defined on
child resources and not on the root resource itself, entering the URL in a web browser
returns a `{"message":"Missing Authentication Token"}` error response. In this case, you must
append the name of a specific child resource to the **Invoke URL** link.
14. The response you get from the **beta** stage `GET` request is shown next.
You can also verify the result by using a browser to navigate to
**http://httpbin.org/get**. This value was assigned to the `url` variable in the
**beta** stage. The two responses are identical.
15. In the **Stages** navigation pane, choose the **prod** stage. Under
**Stage details**, choose the copy icon to copy your API's invoke URL, and then enter
your API's invoke URL in a web browser. This starts the
**prod** stage `GET` request on the root resource of the API.
16. The response you get from the **prod** stage `GET` request is shown next.
You can verify the result by using a browser to navigate to
**http://petstore-demo-endpoint.execute-api.com/petstore/pets**. This value was assigned
to the `url` variable in the **prod** stage. The two responses are identical.
## Pass
stage-specific metadata into an HTTP backend
This procedure describes how to use a stage variable value in a query parameter expression to pass
stage-specific metadata into an HTTP backend. We will use the `stageName` stage variable declared in
the previous procedure.
###### To pass stage-specific metadata into an HTTP backend
1. In the **Resource** navigation pane, choose the **GET** method.
To add
a query string parameter to the method's URL, choose the **Method request** tab, and then in the
**Method request settings** section, choose **Edit**.
2. Choose **URL query string parameters** and do the following:
1. Choose **Add query string**.
2. For **Name**, enter `stageName`.
3. Keep **Required** and **Caching** turned off.
4. Choose **Save**.
5. Choose the **Integration request** tab, and then in the **Integration request settings** section, choose **Edit**.
6. For **Endpoint URL**, append `?stageName=${stageVariables.stageName}` to the previously defined URL
value, so the entire **Endpoint URL** is `http://${stageVariables.url}?stageName=${stageVariables.stageName}`.
7. Choose **Deploy API** and select the **beta** stage.
8. In the main navigation pane, choose **Stages**. In the **Stages**
navigation pane, choose **beta**. Under **Stage details**, choose the
copy icon to copy your API's invoke URL, and then enter your API's invoke URL in a web browser.
###### Note
We use the beta stage here because the HTTP endpoint (as specified by the `url` variable
"http://httpbin.org/get") accepts query parameter expressions and returns them as the `args`
object in its response.
9. You get the following response. Notice that `beta`, assigned to the `stageName` stage
variable, is passed in the backend as the `stageName` argument.
![Response from the API's GET method with an HTTP endpoint using the url stage variable.](https://docs.aws.amazon.com/images/apigateway/latest/developerguide/images/stageVariables-new-console-invoke-beta-stage-with-url-and-stageName-response.png)
## Invoke a Lambda function through an API
with a stage variable
This procedure describes how to use a stage variable to call a Lambda function as a backend of your API. You
use the `function` stage variable declared in [Invoke an HTTP endpoint through an API with a stage
variable](#how-to-set-stage-variables-aws-console-http-endpoint).
When setting a Lambda function as the value of a stage variable, use the function's local name, possibly
including its alias or version specification, as in `HelloWorld`,
`HelloWorld:1` or `HelloWorld:alpha`. Do not use the function's
ARN (for example, `arn:aws:lambda:us-east-1:123456789012:function:HelloWorld`). The API Gateway
console assumes the stage variable value for a Lambda function as the unqualified function name and expands the
given stage variable into an ARN.
###### To invoke a Lambda function through an API
with a stage variable
1. Create a Lambda function named `HelloWorld` using the default Node.js runtime. The code must contain the following:
```
`export const handler = function(event, context, callback) {
if (event.stageName)
callback(null, 'Hello, World! I\\'m calling from the ' + event.stageName + ' stage.');
else
callback(null, 'Hello, World! I\\'m not sure where I\\'m calling from...');
};`
```
For more information on how to create a Lambda function, see [Getting started with the REST API console](./getting-started-rest-new-console.html#getting-started-rest-new-console-create-function).
2. In the **Resources** pane, select **Create resource**, and then do the following:
1. For **Resource path**, select **/**.
2. For **Resource name**, enter `lambdav1`.
3. Choose **Create resource**.
4. Choose the **/lambdav1** resource, and then choose **Create method**.
Then, do the following:
1. For **Method type**, select **GET**.
2. For **Integration type**, select **Lambda function**.
3. Keep **Lambda proxy integration** turned off.
4. For **Lambda function**, enter `${stageVariables.function}`.
![Create a GET method integrated with a Lambda function as specified by the function stage variable.](https://docs.aws.amazon.com/images/apigateway/latest/developerguide/images/stageVariables-new-console-create-lambda-get-method.png)
###### Tip
When prompted with the **Add permission command**, copy the [add-permission](https://docs.aws.amazon.com/cli/latest/reference/lambda/add-permission.html) command. Run the command on
each Lambda function that will be assigned to the `function` stage variable. For example, if
the `$stageVariables.function` value is `HelloWorld`, run the following AWS CLI
command:
```
`aws lambda add-permission --function-name arn:aws:lambda:us-east-1:`account-id`:function:HelloWorld --source-arn arn:aws:execute-api:us-east-1:`account-id`:`api-id`/\*/GET/lambdav1 --principal apigateway.amazonaws.com --statement-id `statement-id-guid` --action lambda:InvokeFunction`
```
Failing to do so results in a `500 Internal Server Error` response when invoking the
method. Replace `${stageVariables.function}` with the Lambda function name that is
assigned to the stage variable.
![AWS CLI command to add permission to the Lambda function to be invoked by the method you created.](https://docs.aws.amazon.com/images/apigateway/latest/developerguide/images/stageVariables-new-console-add-permission-to-lambda-function.png)
5. Choose **Create method**.
6. Deploy the API to both the **prod** and **beta** stages.
7. In the main navigation pane, choose **Stages**. In the **Stages**
navigation pane, choose **beta**. Under **Stage details**, choose the
copy icon to copy your API's invoke URL, and then enter your API's invoke URL in a web browser. Append `/lambdav1` to the URL before you press enter.
You get the following response.
```
`"Hello, World! I'm not sure where I'm calling from..."`
```
## Pass stage-specific metadata to a
Lambda function through a stage variable
This procedure describes how to use a stage variable to pass stage-specific configuration metadata into a
Lambda function. You create a `POST` method and an input mapping template to generate payload using
the `stageName` stage variable you declared earlier.
###### To pass stage-specific metadata to a
Lambda function through a stage variable
1. Choose the **/lambdav1** resource, and then choose **Create method**.
Then, do the following:
1. For **Method type**, select **POST**.
2. For **Integration type**, select **Lambda function**.
3. Keep **Lambda proxy integration** turned off.
4. For **Lambda function**, enter `${stageVariables.function}`.
5. When prompted with the **Add permission command**, copy the the [add-permission](https://docs.aws.amazon.com/cli/latest/reference/lambda/add-permission.html) command. Run the command on each
Lambda function that will be assigned to the `function` stage variable.
6. Choose **Create method**.
7. Choose the **Integration request** tab, and then in the **Integration request settings** section, choose **Edit**.
8. Choose **Mapping templates**, and then choose **Add mapping template**.
9. For **Content type**, enter `application/json`.
10. For **Template body**, enter the following template:
```
`#set($inputRoot = $input.path('$'))
{
"stageName" : "$stageVariables.stageName"
}`
```
###### Note
In a mapping template, a stage variable must be referenced within quotes (as in
`"$stageVariables.stageName"` or `"${stageVariables.stageName}"`). In other places, it
must be referenced without quotes (as in `${stageVariables.function}`).
11. Choose **Save**.
12. Deploy the API to both the **beta** and **prod** stages.
13. To use a REST API client to pass stage-specific metadata, do the following:
1. In the **Stages**
navigation pane, choose **beta**. Under **Stage details**, choose the
copy icon to copy your API's invoke URL, and then enter your API's invoke URL in the input field of a REST API client. Append `/lambdav1` before you submit your request.
You get the following response.
```
`"Hello, World! I'm calling from the beta stage."`
```
2. In the **Stages**
navigation pane, choose **prod**.
Under **Stage details**, choose the
copy icon to copy your API's invoke URL, and then enter your API's invoke URL in the input field of a REST API client. Append `/lambdav1` before you submit your request.
You get the following response.
```
`"Hello, World! I'm calling from the prod stage."`
```
3. To use the **Test** feature to pass stage-specific metadata, do the following:
1. In the **Resources** navigation pane, choose the **Test** tab. You might need to choose the right arrow button to show the tab.
2. For **function**, enter `HelloWorld`.
3. For **stageName**, enter `beta`.
4. Choose **Test**. You do not need to add a body to your `POST` request.
You get the following response.
```
`"Hello, World! I'm calling from the beta stage."`
```
5. You can repeat the previous steps to test the **Prod** stage. For **stageName**, enter `Prod`.
You get the following response.
```
`"Hello, World! I'm calling from the prod stage."`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Use stage variables
Stage variables reference
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.