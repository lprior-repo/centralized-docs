---
url: https://docs.aws.amazon.com/step-functions/latest/dg/connect-eventbridge.html
title: Add EventBridge events with Step Functions
word_count: 655
filtered: true
elements_removed: 0
density_score: 0.80
---

Add EventBridge events with Step Functions - AWS Step Functions
Add EventBridge events with Step Functions - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#connect-eventbridge)
[Supported APIs](#connect-eventbridge-apis)[Error handling](#connect-eventbridge-error)[IAM policies](#eventbridge-iam)
# Add EventBridge events with Step Functions
Step Functions provides a service integration API for integrating with Amazon EventBridge. Learn how to
build event-driven applications by sending custom events directly from Step Functions
workflows.
To learn about integrating with AWS services in Step Functions, see [Integrating services](./integrate-services.html) and [Passing parameters to a service API in Step Functions](./connect-parameters.html).
###### Key features of Optimized EventBridge integration
* The execution ARN and the state machine ARN are automatically appended to the
`Resources` field of each
`PutEventsRequestEntry`.
* If the response from `PutEvents` contains a non-zero
`FailedEntryCount` then the `Task` state fails with
the error `EventBridge.FailedEntry`.
To use the `PutEvents` API, you will need to create an EventBridge rule in your
account that matches the specific pattern of the events you will send. For example, you
could:
* Create a Lambda function in your account that receives and prints an event that
matches an EventBridge rule.
* Create an EventBridge rule in your account on the default event bus that matches a
specific event pattern and targets the Lambda function.
For more information, see:
* [Adding Amazon EventBridge events with
PutEvents](https://docs.aws.amazon.com/eventbridge/latest/userguide/eb-putevents.html) in the EventBridge User Guide.
* [Wait for a Callback with Task Token](./connect-to-resource.html#connect-wait-token) in Service
Integration Patterns.
The following includes a `Task` that sends a custom event:
```
`{
"Type": "Task",
"Resource": "arn:aws:states:::events:putEvents",
"Arguments": {
"Entries": [
{
"Detail": {
"Message": "MyMessage"
},
"DetailType": "MyDetailType",
"EventBusName": "MyEventBus",
"Source": "my.source"
}
]
},
"End": true
}`
```
###### Quota for input or result data
When sending or receiving data between services, the maximum input or result for a task is 256 KiB of data as a UTF-8 encoded string. See [Quotas related to state
machine executions](./service-quotas.html#service-limits-state-machine-executions).
## Optimized EventBridge API
Supported EventBridge API and syntax include:
* [`PutEvents`](https://docs.aws.amazon.com/eventbridge/latest/APIReference/API_PutEvents.html)
## Error handling
The `PutEvents` API accepts an array of entries as input, then returns an
array of result entries. As long as the `PutEvents` action was successful,
`PutEvents` will return an HTTP 200 response, even if one or more entries
failed. `PutEvents` returns the number of failed entries in the
`FailedEntryCount` field.
Step Functions checks whether the `FailedEntryCount` is greater than zero. If it is
greater than zero, Step Functions fails the state with the error
`EventBridge.FailedEntry`. This lets you use the built-in error handling
of Step Functions on task states to catch or retry when there are failed entries, rather than
needing to use an additional state to analyze the `FailedEntryCount` from the
response.
###### Note
If you have implemented idempotency and can safely retry on all entries, you can
use Step Functions' retry logic. Step Functions does not remove successful entries from the
`PutEvents` input array before retrying. Instead, it retries with the
original array of entries.
## IAM policies for calling EventBridge
The following example templates show how AWS Step Functions generates IAM policies based on the resources in your state machine definition. For more information, see [How Step Functions generates IAM policies for integrated
services](./service-integration-iam-templates.html) and [Discover service integration patterns in Step Functions](./connect-to-resource.html).
### `PutEvents`
*Static resources*
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Action": [
"events:PutEvents"
],
"Resource": [
"arn:aws:events:`us-east-1`:123456789012:event-bus/`my-project-eventbus`"
],
"Effect": "Allow"
}
]
}`
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
"events:PutEvents"
],
"Resource": "arn:aws:events:\*:\*:event-bus/\*"
}
]
}`
`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Amazon EMR Serverless
AWS Glue
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.