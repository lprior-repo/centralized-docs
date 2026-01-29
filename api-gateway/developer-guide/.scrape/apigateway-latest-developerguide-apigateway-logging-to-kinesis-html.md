---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-logging-to-kinesis.html
title: Log REST API calls to Amazon Data Firehose in API Gateway
word_count: 504
filtered: true
elements_removed: 0
density_score: 0.83
---

Log REST API calls to Amazon Data Firehose in API Gateway - Amazon API Gateway
Log REST API calls to Amazon Data Firehose in API Gateway - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#apigateway-logging-to-kinesis)
[Firehose log formats for API Gateway](#apigateway-kinesis-log-formats)[Permissions for Firehose
logging](#set-up-kinesis-access-logging-permissions)[Set up Firehose access
logging by using the API Gateway console](#set-up-kinesis-access-logging-using-console)
# Log REST API calls to Amazon Data Firehose in API Gateway
To help debug issues related to client access to your API, you can log API calls to
Amazon Data Firehose. For more information about Firehose, see [What Is Amazon Data
Firehose?](https://docs.aws.amazon.com/firehose/latest/dev/what-is-this-service.html).
For access logging, you can only enable CloudWatch or Firehose—you can't enable both.
However, you can enable CloudWatch for execution logging and Firehose for access logging.
###### Topics
* [Firehose log formats for API Gateway](#apigateway-kinesis-log-formats)
* [Permissions for Firehose
logging](#set-up-kinesis-access-logging-permissions)
* [Set up Firehose access
logging by using the API Gateway console](#set-up-kinesis-access-logging-using-console)
## Firehose log formats for API Gateway
Firehose logging uses the same format as [CloudWatch logging](https://docs.aws.amazon.com/apigateway/latest/developerguide/set-up-logging.html).
## Permissions for Firehose
logging
When Firehose access logging is enabled on a stage, API Gateway creates a service-linked role
in your account if the role doesn't exist already. The role is named
`AWSServiceRoleForAPIGateway` and has the
`APIGatewayServiceRolePolicy` managed policy attached to it. For more
information about service-linked roles, see [Using Service-Linked
Roles](https://docs.aws.amazon.com/IAM/latest/UserGuide/using-service-linked-roles.html).
###### Note
The name of your Firehose stream must be
`amazon-apigateway-`{your-stream-name}``.
## Set up Firehose access
logging by using the API Gateway console
To set up API logging, you must have deployed the API to a stage. You must also have
created a Firehose stream.
1. Sign in to the API Gateway console at [https://console.aws.amazon.com/apigateway](https://console.aws.amazon.com/apigateway).
2. Do one of the following:
1. Choose an existing API, and then choose a stage.
2. Create an API and deploy it to a stage.
3. In the main navigation pane, choose **Stages**.
4. In the **Logs and tracing** section, choose
**Edit**.
5. To enable access logging to a Firehose stream:
1. Turn on
**Custom access logging**.
2. For **Access log destination ARN**, enter the ARN of a Firehose stream. The ARN
format is
`arn:aws:firehose:`{region}`:`{account-id}`:deliverystream/amazon-apigateway-`{your-stream-name}``.
###### Note
The name of your Firehose stream must be
`amazon-apigateway-`{your-stream-name}``.
3. For **Log format**, enter a log format. You can choose **CLF**,
**JSON**, **XML**, or **CSV**. To learn more about example log formats, see [CloudWatch log formats for API Gateway](./set-up-logging.html#apigateway-cloudwatch-log-formats).
4. Choose **Save changes**.
API Gateway is now ready to log requests to your API to Firehose. You don't need to redeploy
the API when you update the stage settings, logs, or stage variables.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
CloudWatch logs
Variables for access logging for API Gateway
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.