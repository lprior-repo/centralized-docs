---
url: https://docs.aws.amazon.com/lambda/latest/dg/logging-with-firehose.html
title: Sending Lambda function logs to Firehose
word_count: 484
filtered: true
elements_removed: 0
density_score: 0.90
---

Sending Lambda function logs to Firehose - AWS Lambda
Sending Lambda function logs to Firehose - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#logging-with-firehose)
[Pricing](#logging-firehose-pricing)[Required permissions for Firehose log destination](#logging-firehose-permissions)[Sending Lambda function logs to Firehose](#logging-firehose-setup)[Cross-Account Logs](#cross-account-logging-firehose)
# Sending Lambda function logs to Firehose
The Lambda console now offers the option to send function logs to Firehose. This enables real-time streaming of your logs to various destinations supported by Firehose,
including third-party analytics tools and custom endpoints.
###### Note
You can configure Lambda function logs to be sent to Firehose using the Lambda console, AWS CLI, AWS CloudFormation, and all AWS SDKs.
## Pricing
For details on pricing, see [Amazon CloudWatch pricing](https://aws.amazon.com/cloudwatch/pricing/#Vended_Logs).
## Required permissions for Firehose log destination
When using the Lambda console to configure Firehose as your function's log destination, you need:
1. The [required IAM permissions](https://docs.aws.amazon.com/lambda/latest/dg/monitoring-cloudwatchlogs.html#monitoring-cloudwatchlogs-prereqs) to use CloudWatch Logs with Lambda.
2. To [set up subscription filters with Firehose](https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/SubscriptionFilters.html#FirehoseExample). This filter defines which log events are delivered to your Firehose stream.
## Sending Lambda function logs to Firehose
In the Lambda console, you can send function logs directly to Firehose after creating a new function. To do this, complete these steps:
1. Sign in to the AWS Management Console and open the Lambda console.
2. Choose your function's name.
3. Choose the **Configuration** tab.
4. Choose the **Monitoring and operations tools** tab.
5. In the "Logging configuration" section, choose **Edit**.
6. In the "Log content" section, select a log format.
7. In the "Log destination" section, complete the following steps:
1. Select a destination service.
2. Choose to **Create a new log group** or use an **Existing log group**.
###### Note
If choosing an existing log group for a Firehose destination, ensure the log group you choose is a `Delivery` log group type.
3. Choose a Firehose stream.
4. The CloudWatch `Delivery` log group will appear.
5. Choose **Save**.
###### Note
If the IAM role provided in the console doesn't have the required permission, then the destination setup will fail. To fix this, refer to Required permissions for Firehose log destination to provide the required permissions.
## Cross-Account Logging
You can configure Lambda to send logs to Firehose delivery stream in a different AWS account. This requires setting up a destination and configuring appropriate permissions in both accounts.
For detailed instructions on setting up cross-account logging, including required IAM roles and policies, see [Setting up a new cross-account subscription](https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/CrossAccountSubscriptions.html) in the CloudWatch Logs documentation.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
View function logs
Log with Amazon S3
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.