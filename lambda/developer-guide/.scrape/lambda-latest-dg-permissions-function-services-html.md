---
url: https://docs.aws.amazon.com/lambda/latest/dg/permissions-function-services.html
title: Granting Lambda function access to AWS services
word_count: 454
filtered: true
elements_removed: 0
density_score: 0.87
---

Granting Lambda function access to AWS services - AWS Lambda
Granting Lambda function access to AWS services - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#permissions-function-services)
# Granting Lambda function access to AWS services
When you [use an AWS service to invoke your function](./lambda-services.html), you grant
permission in a statement on a resource-based policy. You can apply the statement to the entire function, or limit the statement to a single version or alias.
###### Note
When you add a trigger to your function with the Lambda console, the console updates the function's
resource-based policy to allow the service to invoke it. To grant permissions to other accounts or services that
aren't available in the Lambda console, you can use the AWS CLI.
Add a statement with the [add-permission](https://awscli.amazonaws.com/v2/documentation/api/latest/reference/lambda/add-permission.html) command. The simplest resource-based policy statement
allows a service to invoke a function. The following command grants Amazon Simple Notification Service permission to invoke a function named
`my-function`.
```
``aws lambda add-permission \\
--function-name my-function \\
--action lambda:InvokeFunction \\
--statement-id sns \\
--principal sns.amazonaws.com \\
--output text``
```
You should see the following output:
```
{"Sid":"sns","Effect":"Allow","Principal":{"Service":"sns.amazonaws.com"},"Action":"lambda:InvokeFunction","Resource":"arn:aws:lambda:us-east-2:123456789012:function:my-function"}
```
This lets Amazon SNS call the [Invoke](https://docs.aws.amazon.com/lambda/latest/api/API_Invoke.html) API action on the function, but it doesn't restrict the Amazon SNS topic that
triggers the invocation. To ensure that your function is only invoked by a specific resource, specify the Amazon
Resource Name (ARN) of the resource with the `source-arn` option. The following command only allows
Amazon SNS to invoke the function for subscriptions to a topic named `my-topic`.
```
``aws lambda add-permission \\
--function-name my-function \\
--action lambda:InvokeFunction \\
--statement-id sns-my-topic \\
--principal sns.amazonaws.com \\
--source-arn arn:aws:sns:`us-east-2:123456789012:my-topic```
```
Some services can invoke functions in other accounts. If you specify a source ARN that has your account ID in
it, that isn't an issue. For Amazon S3, however, the source is a bucket whose ARN doesn't have an account ID in it.
It's possible that you could delete the bucket and another account could create a bucket with the same name. Use
the `source-account` option with your account ID to ensure that only resources in your account can
invoke the function.
```
``aws lambda add-permission \\
--function-name my-function \\
--action lambda:InvokeFunction \\
--statement-id s3-account \\
--principal s3.amazonaws.com \\
--source-arn arn:aws:s3:::`amzn-s3-demo-bucket` \\
--source-account `123456789012```
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Resource-based policies
Function access for AWS Organizations
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.