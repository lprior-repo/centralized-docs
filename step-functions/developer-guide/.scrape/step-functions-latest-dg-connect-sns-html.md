---
url: https://docs.aws.amazon.com/step-functions/latest/dg/connect-sns.html
title: Publish messages to an Amazon SNS topic with Step Functions
word_count: 581
filtered: true
elements_removed: 0
density_score: 0.81
---

Publish messages to an Amazon SNS topic with Step Functions - AWS Step Functions
Publish messages to an Amazon SNS topic with Step Functions - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#connect-sns)
[Supported APIs](#connect-sns-api)[IAM policies](#sns-iam)
# Publish messages to an Amazon SNS topic with Step Functions
Learn how to use Step Functions to publish messages to an Amazon SNS topic. This page lists the supported Amazon SNS API actions and
provides example `Task` states to publish message to Amazon SNS.
To learn about integrating with AWS services in Step Functions, see [Integrating services](./integrate-services.html) and [Passing parameters to a service API in Step Functions](./connect-parameters.html).
###### Key features of Optimized Amazon SNS integration
There are no specific optimizations for the [Request Response](./connect-to-resource.html#connect-default) or [Wait for a Callback with Task Token](./connect-to-resource.html#connect-wait-token) integration patterns.
The following includes a `Task` state that publishes to an Amazon Simple Notification Service (Amazon SNS)
topic.
```
`{
"StartAt": "Publish to SNS",
"States": {
"Publish to SNS": {
"Type": "Task",
"Resource": "arn:aws:states:::sns:publish",
"Arguments": {
"TopicArn": "arn:aws:sns:`region`:`account-id`:myTopic",
"Message": "{% states.input.message %}",
"MessageAttributes": {
"my\_attribute\_no\_1": {
"DataType": "String",
"StringValue": "value of my\_attribute\_no\_1"
},
"my\_attribute\_no\_2": {
"DataType": "String",
"StringValue": "value of my\_attribute\_no\_2"
}
}
},
"End": true
}
}
}`
```
**Passing dynamic values**. You can modify the above example to dynamically pass an attribute from this JSON payload:
```
`{
"message": "Hello world",
"SNSDetails": {
"attribute1": "some value",
"attribute2": "some other value",
}
}`
```
The following sets values using JSONata expressions for the `StringValue` fields:
```
`"MessageAttributes": {
"my\_attribute\_no\_1": {
"DataType": "String",
"StringValue": "{% $states.input.SNSDetails.attribute1 %}"
},
"my\_attribute\_no\_2": {
"DataType": "String",
"StringValue": "{% $states.input.SNSDetails.attribute2 %}"
}`
```
The following includes a `Task` state that publishes to an Amazon SNS topic, and
then waits for the task token to be returned. See [Wait for a Callback with Task Token](./connect-to-resource.html#connect-wait-token).
```
`{
"StartAt":"Send message to SNS",
"States":{
"Send message to SNS":{
"Type":"Task",
"Resource":"arn:aws:states:::sns:publish**.waitForTaskToken**",
"Arguments":{
"TopicArn":"arn:aws:sns:`region`:`account-id`:myTopic",
"Message":{
"Input":"{% states.input.message %}",
"TaskToken": "{% $states.context.Task.Token %}"
}
},
"End":true
}
}
}`
```
###### Parameters in Step Functions are expressed in PascalCase
Even if the native service API is in camelCase, for example the API action `startSyncExecution`, you specify parameters in PascalCase, such as: `StateMachineArn`.
###### Quota for input or result data
When sending or receiving data between services, the maximum input or result for a task is 256 KiB of data as a UTF-8 encoded string. See [Quotas related to state
machine executions](./service-quotas.html#service-limits-state-machine-executions).
## IAM policies for calling Amazon SNS
The following example templates show how AWS Step Functions generates IAM policies based on the resources in your state machine definition. For more information, see [How Step Functions generates IAM policies for integrated
services](./service-integration-iam-templates.html) and [Discover service integration patterns in Step Functions](./connect-to-resource.html).
*Static resources*
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Action": [
"sns:Publish"
],
"Resource": [
"arn:aws:sns:`us-east-1`:`123456789012`:myTopicName"
]
}
]
}
`
`
```
*Resources based on a Path, or publishing to `TargetArn` or
`PhoneNumber`*
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Action": [
"sns:Publish"
],
"Resource": "\*"
}
]
}`
`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Amazon SageMaker AI
Amazon SQS
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.