---
url: https://docs.aws.amazon.com/lambda/latest/dg/logging-with-s3.html
title: Sending Lambda function logs to Amazon S3
word_count: 963
filtered: true
elements_removed: 0
density_score: 0.85
---

Sending Lambda function logs to Amazon S3 - AWS Lambda
Sending Lambda function logs to Amazon S3 - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#logging-with-s3)
[Pricing](#logging-s3-pricing)[Required permissions for Amazon S3 log destination](#logging-s3-permissions)[Send Lambda logs to Amazon S3](#using-cwl-subscription-filter-lambda-s3)[Sending Lambda function logs to Amazon S3](#logging-s3-setup)[Cross-Account Logs](#cross-account-logging-s3)
# Sending Lambda function logs to Amazon S3
You can configure your Lambda function to send logs directly to Amazon S3 using the Lambda console. This feature provides a
cost-effective solution for long-term log storage and enables powerful analysis options using services like Athena.
###### Note
You can configure Lambda function logs to be sent to Amazon S3 using the Lambda console, AWS CLI, AWS CloudFormation, and all AWS SDKs.
## Pricing
For details on pricing, see [Amazon CloudWatch pricing](https://aws.amazon.com/cloudwatch/pricing/#Vended_Logs).
## Required permissions for Amazon S3 log destination
When using the Lambda console to configure Amazon S3 as your function's log destination, you need:
1. The [required IAM permissions](https://docs.aws.amazon.com/lambda/latest/dg/monitoring-cloudwatchlogs.html#monitoring-cloudwatchlogs-prereqs) to use CloudWatch Logs with Lambda.
2. To [Set up a CloudWatch Logs subscriptions filter to send Lambda function logs to Amazon S3](#using-cwl-subscription-filter-lambda-s3). This filter defines which log events are delivered to your Amazon S3 bucket.
## Set up a CloudWatch Logs subscriptions filter to send Lambda function logs to Amazon S3
To send logs from CloudWatch Logs to Amazon S3, you need to create a subscription filter. This filter defines which log events are delivered to your Amazon S3 bucket. Your Amazon S3 bucket must be in the same Region as your log group.
### To create a subscription filter for Amazon S3
1. Create an Amazon Simple Storage Service (Amazon S3) bucket. We recommend that you use a bucket that was created specifically for CloudWatch Logs. However, if you want to use an existing bucket, skip to step 2.
Run the following command, replacing the placeholder Region with the Region you want to use:
```
`aws s3api create-bucket --bucket amzn-s3-demo-bucket2 --create-bucket-configuration LocationConstraint=region`
```
###### Note
`amzn-s3-demo-bucket2` is an example Amazon S3 bucket name. It is *reserved*. For this procedure to work, you must to replace it with your unique Amazon S3 bucket name.
The following is example output:
```
`{
"Location": "/amzn-s3-demo-bucket2"
}`
```
2. Create the IAM role that grants CloudWatch Logs permission to put data into your Amazon S3 bucket. This policy includes a aws:SourceArn global condition context key to help prevent
the confused deputy security issue. For more information, see [Confused deputy prevention](https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/Subscriptions-confused-deputy.html).
1. Use a text editor to create a trust policy in a file `\~/TrustPolicyForCWL.json` as follows:
```
`{
"Statement": {
"Effect": "Allow",
"Principal": { "Service": "logs.amazonaws.com" },
"Condition": {
"StringLike": {
"aws:SourceArn": "arn:aws:logs:region:123456789012:\*"
}
},
"Action": "sts:AssumeRole"
}
}`
```
2. Use the create-role command to create the IAM role, specifying the trust policy file. Note of the returned Role.Arn value, as you will need it in a later step:
```
`aws iam create-role \\
--role-name CWLtoS3Role \\
--assume-role-policy-document file://\~/TrustPolicyForCWL.json
{
"Role": {
"AssumeRolePolicyDocument": {
"Statement": {
"Action": "sts:AssumeRole",
"Effect": "Allow",
"Principal": {
"Service": "logs.amazonaws.com"
},
"Condition": {
"StringLike": {
"aws:SourceArn": "arn:aws:logs:region:123456789012:\*"
}
}
}
},
"RoleId": "AAOIIAH450GAB4HC5F431",
"CreateDate": "2015-05-29T13:46:29.431Z",
"RoleName": "CWLtoS3Role",
"Path": "/",
"Arn": "arn:aws:iam::123456789012:role/CWLtoS3Role"
}
}`
```
3. Create a permissions policy to define what actions CloudWatch Logs can do on your account. First, use a text editor to create a permissions policy in a file `\~/PermissionsForCWL.json`:
```
`{
"Statement": [
{
"Effect": "Allow",
"Action": ["s3:PutObject"],
"Resource": ["arn:aws:s3:::amzn-s3-demo-bucket2/\*"]
}
]
}`
```
Associate the permissions policy with the role using the following `put-role-policy` command:
```
`aws iam put-role-policy --role-name CWLtoS3Role --policy-name Permissions-Policy-For-S3 --policy-document file://\~/PermissionsForCWL.json`
```
4. Create a `Delivery` log group or use a existing `Delivery` log group.
```
`aws logs create-log-group --log-group-name my-logs --log-group-class DELIVERY --region REGION\_NAME`
```
5. `PutSubscriptionFilter` to set up destination
```
`aws logs put-subscription-filter
--log-group-name my-logs
--filter-name my-lambda-delivery
--filter-pattern ""
--destination-arn arn:aws:s3:::amzn-s3-demo-bucket2
--role-arn arn:aws:iam::123456789012:role/CWLtoS3Role
--region REGION\_NAME`
```
## Sending Lambda function logs to Amazon S3
In the Lambda console, you can send function logs directly to Amazon S3 after creating a new function. To do this, complete these steps:
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
If choosing an existing log group for an Amazon S3 destination, ensure the log group you choose is a `Delivery` log group type.
3. Choose an Amazon S3 bucket to be the destination for your function logs.
4. The CloudWatch `Delivery` log group will appear.
5. Choose **Save**.
###### Note
If the IAM role provided in the console doesn't have the required permissions, then the destination setup will fail. To fix this, refer to [Required permissions for Amazon S3 log destination](#logging-s3-permissions).
## Cross-Account Logging
You can configure Lambda to send logs to an Amazon S3 bucket in a different AWS account. This requires setting up a destination and configuring appropriate permissions in both accounts.
For detailed instructions on setting up cross-account logging, including required IAM roles and policies, see [Setting up a new cross-account subscription](https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/CrossAccountSubscriptions.html) in the CloudWatch Logs documentation.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Log with Firehose
CloudTrail logs
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.