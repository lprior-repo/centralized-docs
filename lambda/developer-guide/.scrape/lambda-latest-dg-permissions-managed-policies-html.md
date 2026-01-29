---
url: https://docs.aws.amazon.com/lambda/latest/dg/permissions-managed-policies.html
title: Working with AWS managed policies in the execution role
word_count: 697
filtered: true
elements_removed: 0
density_score: 0.87
---

Working with AWS managed policies in the execution role - AWS Lambda
Working with AWS managed policies in the execution role - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#permissions-managed-policies)
# Working with AWS managed policies in the execution role
The following AWS managed policies provide permissions that are required to use Lambda features.
|Change|Description|Date|
|
**[
AWSLambdaMSKExecutionRole](https://console.aws.amazon.com/iam/home#policies/arn:aws:iam::aws:policy/service-role/AWSLambdaMSKExecutionRole)** – Lambda added the [kafka:DescribeClusterV2](https://docs.aws.amazon.com/MSK/2.0/APIReference/v2-clusters-clusterarn.html#v2-clusters-clusterarnget) permission to this policy.
|
`AWSLambdaMSKExecutionRole` grants permissions to read and access records from an Amazon Managed Streaming for Apache Kafka (Amazon MSK)
cluster, manage elastic network interfaces (ENIs), and write to CloudWatch Logs.
|
June 17, 2022
|
|
**[
AWSLambdaBasicExecutionRole](https://console.aws.amazon.com/iam/home#policies/arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole)** – Lambda started tracking changes to this policy.
|
`AWSLambdaBasicExecutionRole` grants permissions to upload logs to CloudWatch.
|
February 14, 2022
|
|
**[
AWSLambdaDynamoDBExecutionRole](https://console.aws.amazon.com/iam/home#policies/arn:aws:iam::aws:policy/service-role/AWSLambdaDynamoDBExecutionRole)** – Lambda started tracking changes to this policy.
|
`AWSLambdaDynamoDBExecutionRole` grants permissions to read records from an Amazon DynamoDB stream and write to CloudWatch Logs.
|
February 14, 2022
|
|
**[
AWSLambdaKinesisExecutionRole](https://console.aws.amazon.com/iam/home#policies/arn:aws:iam::aws:policy/service-role/AWSLambdaKinesisExecutionRole)** – Lambda started tracking changes to this policy.
|
`AWSLambdaKinesisExecutionRole` grants permissions to read events from an Amazon Kinesis data stream and write to CloudWatch Logs.
|
February 14, 2022
|
|
**[
AWSLambdaMSKExecutionRole](https://console.aws.amazon.com/iam/home#policies/arn:aws:iam::aws:policy/service-role/AWSLambdaMSKExecutionRole)** – Lambda started tracking changes to this policy.
|
`AWSLambdaMSKExecutionRole` grants permissions to read and access records from an Amazon Managed Streaming for Apache Kafka (Amazon MSK)
cluster, manage elastic network interfaces (ENIs), and write to CloudWatch Logs.
|
February 14, 2022
|
|
**[
AWSLambdaSQSQueueExecutionRole](https://console.aws.amazon.com/iam/home#policies/arn:aws:iam::aws:policy/service-role/AWSLambdaSQSQueueExecutionRole)** – Lambda started tracking changes to this policy.
|
`AWSLambdaSQSQueueExecutionRole` grants permissions to read a message from an Amazon Simple Queue Service (Amazon SQS) queue and write to CloudWatch Logs.
|
February 14, 2022
|
|
**[
AWSLambdaVPCAccessExecutionRole](https://console.aws.amazon.com/iam/home#policies/arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole)** – Lambda started tracking changes to this policy.
|
`AWSLambdaVPCAccessExecutionRole` grants permissions to manage ENIs within an Amazon VPC and write to CloudWatch Logs.
|
February 14, 2022
|
|
**[
AWSXRayDaemonWriteAccess](https://console.aws.amazon.com/iam/home#policies/arn:aws:iam::aws:policy/AWSXRayDaemonWriteAccess)** – Lambda started tracking changes to this policy.
|
`AWSXRayDaemonWriteAccess` grants permissions to upload trace data to X-Ray.
|
February 14, 2022
|
|
**[
CloudWatchLambdaInsightsExecutionRolePolicy](https://console.aws.amazon.com/iam/home#policies/arn:aws:iam::aws:policy/CloudWatchLambdaInsightsExecutionRolePolicy)** – Lambda started tracking changes to this policy.
|
`CloudWatchLambdaInsightsExecutionRolePolicy` grants permissions to write runtime metrics to CloudWatch Lambda Insights.
|
February 14, 2022
|
|
**[
AmazonS3ObjectLambdaExecutionRolePolicy](https://console.aws.amazon.com/iam/home#policies/arn:aws:iam::aws:policy/service-role/AmazonS3ObjectLambdaExecutionRolePolicy)** – Lambda started tracking changes to this policy.
|
`AmazonS3ObjectLambdaExecutionRolePolicy` grants permissions to interact with Amazon Simple Storage Service
(Amazon S3) object Lambda and to write to CloudWatch Logs.
|
February 14, 2022
|
For some features, the Lambda console attempts to add missing permissions to your execution role in a customer
managed policy. These policies can become numerous. To avoid creating extra policies, add the relevant AWS
managed policies to your execution role before enabling features.
When you use an [event source mapping](./invocation-eventsourcemapping.html) to invoke your
function, Lambda uses the execution role to read event data. For example, an event source mapping for Kinesis reads
events from a data stream and sends them to your function in batches.
When a service assumes a role in your account, you can include the `aws:SourceAccount` and `aws:SourceArn` global
condition context keys in your role trust policy to limit access to the role to only requests that are generated by expected resources. For more
information, see [Cross-service
confused deputy prevention for AWS Security Token Service](https://docs.aws.amazon.com/IAM/latest/UserGuide/confused-deputy.html#cross-service-confused-deputy-prevention).
In addition to the AWS managed policies, the Lambda console provides templates for creating a custom policy
with permissions for additional use cases. When you create a function in the Lambda console, you can choose to
create a new execution role with permissions from one or more templates. These templates are also applied
automatically when you create a function from a blueprint, or when you configure options that require access to
other services. Example templates are available in this guide's [GitHub
repository](https://github.com/awsdocs/aws-lambda-developer-guide/tree/master/iam-policies).
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Update execution role
Source function ARN
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.