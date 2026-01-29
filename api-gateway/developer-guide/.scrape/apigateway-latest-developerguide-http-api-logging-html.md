---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/http-api-logging.html
title: Configure logging for HTTP APIs in API Gateway
word_count: 584
filtered: true
elements_removed: 0
density_score: 0.87
---

Configure logging for HTTP APIs in API Gateway - Amazon API Gateway
Configure logging for HTTP APIs in API Gateway - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#http-api-logging)
[Permissions to activate logging](#http-api-logging.permissions)[ Create a log group and activate logging for HTTP APIs](#http-api-enable-logging)[Example log formats](#http-api-enable-logging.examples)
# Configure logging for HTTP APIs in API Gateway
You can turn on logging to write logs to CloudWatch Logs. You can use [logging variables](./http-api-logging-variables.html) to customize the
content of your logs.
To improve your security posture, we recommend that you write logs to CloudWatch Logs for all stages of your HTTP API.
You might need to do this to comply with various compliance frameworks. For more information, see [Amazon API Gateway controls](https://docs.aws.amazon.com/securityhub/latest/userguide/apigateway-controls.html) in
the *AWS Security Hub User Guide*.
To turn on logging for an HTTP API, you must do the following.
1. Ensure that your user has the required permissions to activate logging.
2. Create a CloudWatch Logs log group.
3. Provide the ARN of the CloudWatch Logs log group for a stage of your API.
## Permissions to activate logging
To turn on logging for an API, your user must have the following permissions.
JSON
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Action": [
"logs:DescribeLogGroups",
"logs:DescribeLogStreams",
"logs:GetLogEvents",
"logs:FilterLogEvents"
],
"Resource": "arn:aws:logs:`us-east-2`:`123456789012`:log-group:\*"
},
{
"Effect": "Allow",
"Action": [
"logs:CreateLogDelivery",
"logs:PutResourcePolicy",
"logs:UpdateLogDelivery",
"logs:DeleteLogDelivery",
"logs:CreateLogGroup",
"logs:DescribeResourcePolicies",
"logs:GetLogDelivery",
"logs:ListLogDeliveries"
],
"Resource": "\*"
}
]
}
`
`
```
## Create a log group and activate logging for HTTP APIs
You can create a log group and activate access logging using the AWS Management Console or the AWS CLI.
AWS Management Console
1. Create a log group.
To learn how to create a log group using the console, see [Create a Log Group in Amazon CloudWatch Logs User Guide](https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/Working-with-log-groups-and-streams.html).
2. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
3. Choose an HTTP API.
4. Under the **Monitor** tab in the primary navigation panel, choose **Logging**.
5. Select a stage to activate logging and choose **Select**.
6. Choose **Edit** to activate access logging.
7. Turn on **Access logging**, enter a CloudWatch Logs, and select a log format.
8. Choose **Save**.
AWS CLI
The following [create-log-group](https://docs.aws.amazon.com/cli/latest/reference/logs/create-log-group.html) command creates a log group:
```
`aws logs create-log-group --log-group-name `my-log-group``
```
You need the Amazon Resource Name (ARN) for your log group to turn on logging. The ARN format is
arn:aws:logs:`region`:`account-id`:log-group:`log-group-name`.
The following [update-stage](https://docs.aws.amazon.com/cli/latest/reference/apigatewayv2/update-stage.html) command turns on logging for the `$default` stage of an HTTP API:
```
`aws apigatewayv2 update-stage --api-id `abcdef` \\
--stage-name '`$default`' \\
--access-log-settings '{"DestinationArn": "arn:aws:logs:`region`:`account-id`:log-group:`log-group-name`", "Format": "$context.identity.sourceIp - - [$context.requestTime] \\"$context.httpMethod $context.routeKey $context.protocol\\" $context.status $context.responseLength $context.requestId"}'`
```
## Example log formats
Examples of some common access log formats are available in the API Gateway console and are
listed as follows.
* `CLF` ([Common Log
Format](https://httpd.apache.org/docs/current/logs.html#common)):
```
`$context.identity.sourceIp - - [$context.requestTime] "$context.httpMethod $context.routeKey $context.protocol" $context.status $context.responseLength $context.requestId $context.extendedRequestId`
```
* `JSON`:
```
`{ "requestId":"$context.requestId", "ip": "$context.identity.sourceIp", "requestTime":"$context.requestTime", "httpMethod":"$context.httpMethod","routeKey":"$context.routeKey", "status":"$context.status","protocol":"$context.protocol", "responseLength":"$context.responseLength", "extendedRequestId": "$context.extendedRequestId" }`
```
* `XML`:
```
`&lt;request id="$context.requestId"&gt; &lt;ip&gt;$context.identity.sourceIp&lt;/ip&gt; &lt;requestTime&gt;$context.requestTime&lt;/requestTime&gt; &lt;httpMethod&gt;$context.httpMethod&lt;/httpMethod&gt; &lt;routeKey&gt;$context.routeKey&lt;/routeKey&gt; &lt;status&gt;$context.status&lt;/status&gt; &lt;protocol&gt;$context.protocol&lt;/protocol&gt; &lt;responseLength&gt;$context.responseLength&lt;/responseLength&gt; &lt;extendedRequestId&gt;$context.extendedRequestId&lt;/extendedRequestId&gt; &lt;/request&gt;`
```
* `CSV` (comma-separated values):
```
`$context.identity.sourceIp,$context.requestTime,$context.httpMethod,$context.routeKey,$context.protocol,$context.status,$context.responseLength,$context.requestId,$context.extendedRequestId`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Metrics
Logging variables
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.