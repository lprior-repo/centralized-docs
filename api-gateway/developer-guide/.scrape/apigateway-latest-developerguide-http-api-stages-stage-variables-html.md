---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/http-api-stages.stage-variables.html
title: Use stage variables for HTTP APIs in API Gateway
word_count: 440
filtered: true
elements_removed: 0
density_score: 0.81
---

Use stage variables for HTTP APIs in API Gateway - Amazon API Gateway
Use stage variables for HTTP APIs in API Gateway - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#http-api-stages.stage-variables)
[Example – Use a stage variable to customize the HTTP integration endpoint](#http-api-stages.stage-variables-examples)
# Use stage variables for HTTP APIs in API Gateway
Stage variables are key-value pairs that you can define for a stage of an
HTTP API. They act like environment variables and can be used in your API
setup.
Stage variables are not intended to be used for sensitive data, such as credentials. To pass sensitive data to
integrations, use an AWS Lambda authorizer. You can pass sensitive data to integrations in the output of the Lambda
authorizer. To learn more, see [Lambda authorizer
response format](./http-api-lambda-authorizer.html#http-api-lambda-authorizer.payload-format-response).
## Example – Use a stage variable to customize the HTTP integration endpoint
For example, you can define a stage variable, and then set its value as an HTTP
endpoint for an HTTP proxy integration. Later, you can reference the endpoint by using
the associated stage variable name. By doing this, you can use the same API setup with a
different endpoint at each stage. Similarly, you can use stage variables to specify a
different AWS Lambda function integration for each stage of your API.
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
Lambda function manually. The following [add-permission](https://docs.aws.amazon.com/cli/latest/reference/lambda/add-permission.html) command configures the permission for the Lambda function:
```
`aws lambda add-permission --function-name arn:aws:lambda:XXXXXX:your-lambda-function-name --source-arn arn:aws:execute-api:us-east-1:YOUR\_ACCOUNT\_ID:api\_id/\*/HTTP\_METHOD/resource --principal apigateway.amazonaws.com --statement-id apigateway-access --action lambda:InvokeFunction`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Stages
API Gateway stage
variables reference for HTTP APIs in API Gateway
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.