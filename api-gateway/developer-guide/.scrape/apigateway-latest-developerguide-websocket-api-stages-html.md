---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/websocket-api-stages.html
title: Create stages for WebSocket APIs in API Gateway
word_count: 649
filtered: true
elements_removed: 0
density_score: 0.86
---

Create stages for WebSocket APIs in API Gateway - Amazon API Gateway
Create stages for WebSocket APIs in API Gateway - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#websocket-api-stages)
[Stage variables](#websocket-api-stages.stage-variables)[Stage variables reference](#websocket-api-stages.stage-variables-reference)
# Create stages for WebSocket APIs in API Gateway
An API stage is a logical reference to a lifecycle state of your API (for example,
`dev`, `prod`, `beta`, or `v2`). API stages
are identified by their API ID and stage name, and they're included in the URL you use to
invoke the API. Each stage is a named reference to a deployment of the API and is made
available for client applications to call.
A deployment is a snapshot of your API configuration. After you deploy an API to a stage,
it’s available for clients to invoke. You must deploy an API for changes to take effect.
## Stage variables
Stage variables are key-value pairs that you can define for a stage of a
WebSocket API. They act like environment variables and can be used in your API
setup.
For example, you can define a stage variable, and then set its value as an HTTP
endpoint for an HTTP proxy integration. Later, you can reference the endpoint by using
the associated stage variable name. By doing this, you can use the same API setup with a
different endpoint at each stage. Similarly, you can use stage variables to specify a
different AWS Lambda function integration for each stage of your API.
###### Note
Stage variables are not intended to be used for sensitive data, such as credentials. To pass sensitive data to
integrations, use an AWS Lambda authorizer. You can pass sensitive data to integrations in the output of the Lambda
authorizer. To learn more, see [Lambda authorizer
response format](./http-api-lambda-authorizer.html#http-api-lambda-authorizer.payload-format-response).
### Examples
To use a stage variable to customize the HTTP integration endpoint, you must first
set the name and value of the stage variable (for example, `url`) with a
value of `example.com`. Next, set up an HTTP proxy integration. Instead
of entering the endpoint's URL, you can tell API Gateway to use the stage variable value,
`http://${stageVariables.url}`. This value tells API Gateway to
substitute your stage variable `${}` at runtime, depending on the stage
of your API.
You can reference stage variables in a similar way to specify a Lambda function
name or an AWS role ARN.
When specifying a Lambda function name as a stage variable value, you must configure the permissions on the
Lambda function manually. The following [add-permission](https://docs.aws.amazon.com/cli/latest/reference/lambda/add-permission.html) command adds the required permissions:
```
`aws lambda add-permission --function-name arn:aws:lambda:XXXXXX:your-lambda-function-name --source-arn arn:aws:execute-api:us-east-1:YOUR\_ACCOUNT\_ID:api\_id/\*/HTTP\_METHOD/resource --principal apigateway.amazonaws.com --statement-id apigateway-access --action lambda:InvokeFunction`
```
### HTTP
integration URIs
You can use a stage variable as part of an HTTP integration URI, as shown in
the following examples.
* A full URI without protocol – `http://${stageVariables.&lt;&lt;variable\_name&gt;&gt;}`
* A full domain – `http://${stageVariables.&lt;&lt;variable\_name&gt;&gt;}/resource/operation`
* A subdomain – `http://${stageVariables.&lt;&lt;variable\_name&gt;&gt;}.example.com/resource/operation`
* A path – `http://example.com/${stageVariables.&lt;&lt;variable\_name&gt;&gt;}/bar`
* A query string – `http://example.com/foo?q=${stageVariables.&lt;&lt;variable\_name&gt;&gt;}`
### Lambda functions
You can use a stage variable in place of a Lambda function name or alias, as
shown in the following examples.
* `arn:aws:apigateway:&lt;&lt;region&gt;&gt;:lambda:path/2015-03-31/functions/arn:aws:lambda:&lt;&lt;region&gt;&gt;:&lt;&lt;account\_id&gt;&gt;:function:${stageVariables.&lt;&lt;function\_variable\_name&gt;&gt;}/invocations`
* `arn:aws:apigateway:&lt;&lt;region&gt;&gt;:lambda:path/2015-03-31/functions/arn:aws:lambda:&lt;&lt;region&gt;&gt;:&lt;&lt;account\_id&gt;&gt;:function:&lt;&lt;function\_name&gt;&gt;:${stageVariables.&lt;&lt;version\_variable\_name&gt;&gt;}/invocations`
###### Note
To use a stage variable for a Lambda function, the function must be in the same account as the API. Stage
variables don't support cross-account Lambda functions.
### AWS integration credentials
You can use a stage variable as part of an AWS user or role credential ARN,
as shown in the following example.
* `arn:aws:iam::&lt;&lt;account\_id&gt;&gt;:${stageVariables.&lt;&lt;variable\_name&gt;&gt;}`
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Publish
Deploy a WebSocket API
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.