---
url: https://docs.aws.amazon.com/step-functions/latest/dg/connect-sqs.html
title: Send messages to an Amazon SQS queue with Step Functions
word_count: 497
filtered: true
elements_removed: 0
density_score: 0.81
---

Send messages to an Amazon SQS queue with Step Functions - AWS Step Functions
Send messages to an Amazon SQS queue with Step Functions - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#connect-sqs)
[Supported APIs](#connect-sqs-api)[IAM policies](#sqs-iam)
# Send messages to an Amazon SQS queue with Step Functions
You can to send messages to an Amazon SQS queue using the following Amazon SQS API actions and
example `Task` state code for Step Functions workflows.
To learn about integrating with AWS services in Step Functions, see [Integrating services](./integrate-services.html) and [Passing parameters to a service API in Step Functions](./connect-parameters.html).
To learn more about receiving messages in Amazon SQS, see [Receive and
Delete Your Message](https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/step-receive-delete-message.html) in the *Amazon Simple Queue Service Developer Guide*.
The following sample includes a `Task` state (JSONata) that sends an Amazon Simple Queue Service (Amazon SQS)
message with optional **MessageAttributes**:
```
`{
"StartAt": "Send to SQS",
"States": {
"Send to SQS": {
"Type": "Task",
"Resource": "arn:aws:states:::sqs:sendMessage",
"Arguments": {
"QueueUrl": "https://sqs.us-east-1.amazonaws.com/`account-id`/myQueue",
"MessageBody": "{% $states.input.message %}",
"MessageAttributes": {
"my\_attribute\_no\_1": {
"DataType": "String",
"StringValue": "`attribute1`"
},
"my\_attribute\_no\_2": {
"DataType": "String",
"StringValue": "`attribute2`"
}
}
},
"End": true
}
}
}`
```
The following state machine includes a `Task` state that publishes to an Amazon SQS queue, and
then waits for the task token to be returned. See [Wait for a Callback with Task Token](./connect-to-resource.html#connect-wait-token).
```
`{
"StartAt":"Send message to SQS",
"States":{
"Send message to SQS":{
"Type":"Task",
"Resource":"arn:aws:states:::sqs:sendMessage.waitForTaskToken",
"Arguments":{
"QueueUrl":"https://sqs.us-east-1.amazonaws.com/`account-id`/myQueue",
"MessageBody":{
"Input" : "{% $states.input.message %}",
"MyTaskToken" : "{% $states.context.Task.Token %}"
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
## IAM policies for calling Amazon SQS
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
"sqs:SendMessage"
],
"Resource": [
"arn:aws:sqs:`us-east-1`:`123456789012`:myQueueName"
]
}
]
}
`
`
```
*Dynamic resources*
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Action": [
"sqs:SendMessage"
],
"Resource": "\*"
}
]
}`
`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Amazon SNS
AWS Step Functions
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.