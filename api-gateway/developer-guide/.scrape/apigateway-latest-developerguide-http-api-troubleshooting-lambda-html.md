---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/http-api-troubleshooting-lambda.html
title: Troubleshooting issues with HTTP API Lambda
word_count: 684
filtered: true
elements_removed: 0
density_score: 0.87
---

Troubleshooting issues with HTTP API Lambda integrations - Amazon API Gateway
Troubleshooting issues with HTTP API Lambda integrations - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#http-api-troubleshooting-lambda)
[Issue: My API with a Lambda integration
returns {"message":"Internal Server Error"}](#http-api-troubleshooting-lambda-internal-server-error)
# Troubleshooting issues with HTTP API Lambda
integrations
The following provides troubleshooting advice for errors and issues that you might
encounter when using [AWS Lambda integrations](./http-api-develop-integrations-lambda.html) with HTTP APIs.
## Issue: My API with a Lambda integration
returns `{"message":"Internal Server Error"}`
To troubleshoot the internal server error, add the
`$context.integrationErrorMessage`
[logging variable](./http-api-logging-variables.html) to your log format,
and view your HTTP API's logs. To achieve this, do the following:
###### To create a log group by using the AWS Management Console
1. Open the CloudWatch console at
[https://console.aws.amazon.com/cloudwatch/](https://console.aws.amazon.com/cloudwatch/).
2. Choose **Log groups**.
3. Choose **Create log group**.
4. Enter a log group name, and then choose **Create**.
5. Note the Amazon Resource Name (ARN) for your log group. The ARN format is
arn:aws:logs:`region`:
`account-id`:log-group:`log-group-name`.
You need the log group ARN to enable access logging for your
HTTP API.
###### To add the `$context.integrationErrorMessage` logging variable
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. Choose your HTTP API.
3. Under **Monitor**, choose
**Logging**.
4. Select a stage of your API.
5. Choose **Edit**, and then enable access logging.
6. For **Log destination**, enter the ARN of the log group that you created in the previous step.
7. For **Log format**, choose **CLF**. API Gateway creates an example log format.
8. Add `$context.integrationErrorMessage` to the end of the log
format.
9. Choose **Save**.
###### To view your API's logs
1. Generate logs. Use a browser or `curl` to invoke your API.
```
``$`curl https://`api-id`.execute-api.`us-west-2`.amazonaws.com/`route``
```
2. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
3. Choose your HTTP API.
4. Under **Monitor**, choose
**Logging**.
5. Select the stage of your API for which you enabled logging.
6. Choose **View logs in CloudWatch**.
7. Choose the latest log stream to view your HTTP API's logs.
8. Your log entry should look similar to the following:
![CloudWatch Logs log entry showing the integration error message from Lambda.](https://docs.aws.amazon.com/images/apigateway/latest/developerguide/images/troubleshoot-http-api-logs.png)
Because we added `$context.integrationErrorMessage` to the log format, we
see an error message in our logs that summarizes the problem.
Your logs might include a different error message that indicates that there's a problem with your Lambda
function code. In that case, check your Lambda function code, and verify that your Lambda function returns a
response in the [required format](./http-api-develop-integrations-lambda.html#http-api-develop-integrations-lambda.response). If your logs
don't include an error message, add `$context.error.message` and
`$context.error.responseType` to your log format for more information to help troubleshoot.
In this case, the logs show that API Gateway didn't have the required permissions to invoke
the Lambda function.
When you create a Lambda integration in the API Gateway console, API Gateway automatically configures permissions to invoke
the Lambda function. When you create a Lambda integration by using the AWS CLI, CloudFormation, or an SDK, you must grant
permissions for API Gateway to invoke the function. The following [add-permission](https://docs.aws.amazon.com/cli/latest/reference/lambda/add-permission.html) commands grant permission for different HTTP API routes to invoke a Lambda
function.
###### Example – For the `$default` stage and `$default` route of an HTTP API
```
`aws lambda add-permission \\
--function-name `my-function` \\
--statement-id `apigateway-invoke-permissions` \\
--action lambda:InvokeFunction \\
--principal apigateway.amazonaws.com \\
--source-arn "arn:aws:execute-api:`us-west-2`:`123456789012`:`api-id`/`\\$default`/`\\$default`"`
```
###### Example – For the `prod` stage and `test` route of an HTTP API
```
`aws lambda add-permission \\
--function-name `my-function` \\
--statement-id `apigateway-invoke-permissions` \\
--action lambda:InvokeFunction \\
--principal apigateway.amazonaws.com \\
--source-arn "arn:aws:execute-api:`us-west-2`:`123456789012`:`api-id`/`prod`/\*/`test`"`
```
[Confirm the function policy](https://docs.aws.amazon.com/lambda/latest/dg/access-control-resource-based.html) in the **Permissions** tab of
the Lambda console.
Try invoking your API again. You should see your Lambda function's response.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Troubleshooting
JWT authorizers
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.