---
url: https://docs.aws.amazon.com/lambda/latest/dg/with-sns.html
title: Invoking Lambda functions with Amazon SNS notifications
word_count: 857
filtered: true
elements_removed: 0
density_score: 0.86
---

Invoking Lambda functions with Amazon SNS notifications - AWS Lambda
Invoking Lambda functions with Amazon SNS notifications - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#with-sns)
[Idempotency utility from Powertools for AWS Lambda](#services-sns-powertools-idempotency)[Adding an Amazon SNS topic trigger for a Lambda function using the console](#sns-trigger-console)[Manually adding an Amazon SNS topic trigger for a Lambda function](#sns-trigger-manual)[Sample SNS event shape](#sns-sample-event)
# Invoking Lambda functions with Amazon SNS notifications
You can use a Lambda function to process Amazon Simple Notification Service (Amazon SNS) notifications. Amazon SNS supports Lambda functions as a
target for messages sent to a topic. You can subscribe your function to topics in the same account or in other AWS
accounts. For a detailed walkthrough, see [Tutorial: Using AWS Lambda with Amazon Simple Notification Service](./with-sns-example.html).
Lambda supports SNS triggers for standard SNS topics only. FIFO topics aren't supported.
Lambda processes SNS messages asynchronously by queuing the messages and handling retries. If Amazon SNS
can't reach Lambda or the message is rejected, Amazon SNS retries at increasing intervals over several hours.
For details, see [Reliability](https://aws.amazon.com/sns/faqs/#Reliability) in the Amazon SNS FAQs.
###### Warning
Lambda asynchronous invocations process each event at least once, and duplicate processing of records can occur.
To avoid potential issues related to duplicate events, we strongly recommend that you make your function code
idempotent. To learn more, see [
How do I make my Lambda function idempotent](https://repost.aws/knowledge-center/lambda-function-idempotent) in the AWS Knowledge Center.
## Idempotency utility from Powertools for AWS Lambda
The idempotency utility from Powertools for AWS Lambda makes your Lambda functions idempotent. It is available for Python, TypeScript, Java, and .NET.
For more information, see [Idempotency utility](https://docs.powertools.aws.dev/lambda/python/latest/utilities/idempotency/) in the *Powertools for AWS Lambda (Python) documentation*, [Idempotency Utility](https://docs.aws.amazon.com/powertools/typescript/2.1.1/utilities/idempotency/) in the *Powertools for AWS Lambda (TypeScript) documentation*, [Idempotency Utility](https://docs.powertools.aws.dev/lambda/java/latest/utilities/idempotency/) in the *Powertools for AWS Lambda (Java) documentation*, and [Idempotency Utility](https://docs.powertools.aws.dev/lambda/dotnet/utilities/idempotency/) in the *Powertools for AWS Lambda (.NET) documentation*.
###### Topics
* [Adding an Amazon SNS topic trigger for a Lambda function using the console](#sns-trigger-console)
* [Manually adding an Amazon SNS topic trigger for a Lambda function](#sns-trigger-manual)
* [Sample SNS event shape](#sns-sample-event)
* [Tutorial: Using AWS Lambda with Amazon Simple Notification Service](./with-sns-example.html)
## Adding an Amazon SNS topic trigger for a Lambda function using the console
To add an SNS topic as a trigger for a Lambda function, the easiest way is to use
the Lambda console. When you add the trigger via the console, Lambda automatically
sets up the necessary permissions and subscriptions to start receiving events from
the SNS topic.
###### To add an SNS topic as a trigger for a Lambda function (console)
1. Open the [Functions page](https://console.aws.amazon.com/lambda/home#/functions) of the Lambda console.
2. Choose the name of a function you want to add the trigger for.
3. Choose **Configuration**, and then choose **Triggers**.
4. Choose **Add trigger**.
5. Under **Trigger configuration**, in the dropdown menu, choose
**SNS**.
6. For **SNS topic**, choose the SNS topic to subscribe to.
## Manually adding an Amazon SNS topic trigger for a Lambda function
To set up an SNS trigger for a Lambda function manually, you need to complete the following
steps:
* Define a resource-based policy for your function to allow SNS to invoke it.
* Subscribe your Lambda function to the Amazon SNS topic.
###### Note
If your SNS topic and your Lambda function are in different AWS accounts, you also
need to grant extra permissions to allow cross-account subscriptions to the SNS topic.
For more information, see [Grant
cross-account permission for Amazon SNS subscription](./with-sns-example.html#with-sns-subscription-grant-permission).
You can use the AWS Command Line Interface (AWS CLI) to complete both of these steps. First, to define
a resource-based policy for a Lambda function that allows SNS invocations, use the following
AWS CLI command. Be sure to replace the value of `--function-name` with your
Lambda function name, and the value of `--source-arn` with your SNS topic ARN.
```
`aws lambda add-permission --function-name `example-function` \\
--source-arn `arn:aws:sns:us-east-1:123456789012:sns-topic-for-lambda` \\
--statement-id function-with-sns --action "lambda:InvokeFunction" \\
--principal sns.amazonaws.com`
```
To subscribe your function to the SNS topic, use the following AWS CLI command. Replace
the value of `--topic-arn` with your SNS topic ARN, and the value of
`--notification-endpoint` with your Lambda function ARN.
```
`aws sns subscribe --protocol lambda \\
--region us-east-1 \\
--topic-arn `arn:aws:sns:us-east-1:123456789012:sns-topic-for-lambda` \\
--notification-endpoint `arn:aws:lambda:us-east-1:123456789012:function:example-function``
```
## Sample SNS event shape
Amazon SNS invokes your function [asynchronously](./invocation-async.html) with an event that contains a
message and metadata.
###### Example Amazon SNS message event
```
`{
"Records": [
{
"EventVersion": "1.0",
"EventSubscriptionArn": "arn:aws:sns:us-east-1:123456789012:sns-lambda:21be56ed-a058-49f5-8c98-aedd2564c486",
"EventSource": "aws:sns",
"Sns": {
"SignatureVersion": "1",
"Timestamp": "2019-01-02T12:45:07.000Z",
"Signature": "tcc6faL2yUC6dgZdmrwh1Y4cGa/ebXEkAi6RibDsvpi+tE/1+82j...65r==",
"SigningCertURL": "https://sns.us-east-1.amazonaws.com/SimpleNotificationService-ac565b8b1a6c5d002d285f9598aa1d9b.pem",
"MessageId": "95df01b4-ee98-5cb9-9903-4c221d41eb5e",
"Message": "Hello from SNS!",
"MessageAttributes": {
"Test": {
"Type": "String",
"Value": "TestString"
},
"TestBinary": {
"Type": "Binary",
"Value": "TestBinary"
}
},
"Type": "Notification",
"UnsubscribeUrl": "https://sns.us-east-1.amazonaws.com/?Action=Unsubscribe&amp;amp;SubscriptionArn=arn:aws:sns:us-east-1:123456789012:test-lambda:21be56ed-a058-49f5-8c98-aedd2564c486",
"TopicArn":"arn:aws:sns:us-east-1:123456789012:sns-lambda",
"Subject": "TestInvoke"
}
}
]
}`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
S3 Batch
Tutorial
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.