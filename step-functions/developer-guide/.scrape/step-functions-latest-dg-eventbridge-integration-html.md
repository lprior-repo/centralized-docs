---
url: https://docs.aws.amazon.com/step-functions/latest/dg/eventbridge-integration.html
title: Automating Step Functions event delivery with EventBridge
word_count: 1614
filtered: true
elements_removed: 0
density_score: 0.82
---

Automating Step Functions event delivery with EventBridge - AWS Step Functions
Automating Step Functions event delivery with EventBridge - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#eventbridge-integration)
[Step Functions events](#supported-events)[Delivering Step Functions
events](#eventbridge-using-events-rules)[Triggering Step Functions state machines](#eventbridge-stepfunctions-as-target)[Events detail reference](#events-detail-reference)
# Automating Step Functions event delivery with EventBridge
With EventBridge, you can select events from Step Functions standard workflows,
to send to other services for additional processing. This
technique provides a flexible way to loosely connect components and monitor your resources.
Amazon EventBridge is a serverless service that connects application
components together to build scalable event-driven applications.
Event-driven architecture is a style of building loosely-coupled software systems that work
together by emitting and responding to *events*. Events represent a change in state, or an update.
By using EventBridge to deliver Step Functions events to other services, you can monitor
your standard workflows without continuously calling the [DescribeExecution](https://docs.aws.amazon.com/step-functions/latest/apireference/API_DescribeExecution.html) API to get the status. Status changes in state machine executions
are sent to EventBridge automatically. You can use those events to target services.
For example, events might invoke a AWS Lambda function, publish a message to Amazon Simple Notification Service
(Amazon SNS) topic, or even run another SFN workflow.
###### How event delivery works
Step Functions generates and sends events to the default EventBridge *event bus *which is automatically provisioned in every AWS account. An event bus is a router that receives events and delivers them to zero
or more destinations, or *targets*. Targets are other AWS services. You can specify rules for the event bus that compare events against the rule's *event pattern*. When the event matches a pattern, the event bus sends the event to the specified target(s). The following diagram shows this process:
![AWS services send events to EventBridge where rules match events and send them to targets.](https://docs.aws.amazon.com/images/step-functions/latest/dg/images/eventbridge-integration_shared_conceptual.png)
###### Standard versus Express workflows
Only standard workflows emit events to EventBridge. To monitor the execution of express workflows, you can use CloudWatch Logs. See [Logging in CloudWatch Logs](./cw-logs.html).
## Step Functions events
Step Functions sends the following events to the default EventBridge event bus automatically. Events that match a rule's event pattern are delivered to the specified targets
on a [best-effort basis](https://docs.aws.amazon.com/eventbridge/latest/userguide/eb-service-event.html#eb-service-event-delivery-level). Events might be
delivered out of order.
For more information, see [EventBridge events](https://docs.aws.amazon.com/eventbridge/latest/userguide/eb-events.html) in the *Amazon EventBridge User Guide.*
|Event detail type|Description|
|
[Execution Status Change](#event-detail-execution-status-change)
|
Represents a change in the status of a state machine execution.
|
## Delivering Step Functions events using
EventBridge
To have the EventBridge default event bus send Step Functions events to a target, you must create
a rule. Each rule contains an event pattern, which EventBridge matches against each event received on the event bus. If the event data matches the specified event pattern, EventBridge delivers that event to the rule's target(s).
For comprehensive instructions on creating event bus rules, see [Creating rules that
react to events](https://docs.aws.amazon.com/eventbridge/latest/userguide/eb-create-rule.html) in the *EventBridge User Guide*.
You can also create an event bus rule for a specific state machine from the Step Functions console:
* On the **Details** page of a state machine, choose
**Actions**, and then choose **Create *EventBridge*
rule**.
The EventBridge console opens to the **Create rule** page, with the state machine selected as the event source for the rule.
* Follow the procedure detailed in [Creating rules that
react to events](https://docs.aws.amazon.com/eventbridge/latest/userguide/eb-create-rule.html) in the *EventBridge User
Guide*.
### Creating event patterns that match Step Functions events
Each event pattern is a JSON object that contains:
* A `source` attribute that identifies the service sending the event.
For Step Functions events, the source is `aws.states`.
* (Optional): A `detail-type` attribute that contains an array of the
event types to match.
* (Optional): A `detail` attribute containing any other event data on
which to match.
For example, the following event pattern matches against all Execution Status Change events from Step Functions:
```
`{
"source": ["aws.states"],
"detail-type": ["Step Functions Execution Status Change"]
}`
```
While the following example matches against a specific execution associated with a
specific state machine, when that execution fails or times out:
```
`{
"source": ["aws.states"],
"detail-type": ["Step Functions Execution Status Change"],
"detail": {
"status": ["FAILED", "TIMED\_OUT"],
"stateMachineArn": ["arn:aws:states:`region`:`account-id`:stateMachine:state-machine"],
"executionArn": ["arn:aws:states:`region`:`account-id`:execution:state-machine-name:execution-name"]
}
}`
```
For more information on writing event patterns, see [Event patterns](https://docs.aws.amazon.com/eventbridge/latest/userguide/eb-event-patterns.html)
in the *EventBridge User Guide*.
## Triggering Step Functions state machines using events
You can also specify a Step Functions state machine as a target for EventBridge event bus rule. This enables you to
trigger an execution of a Step Functions workflow in response to an event from another AWS service.
For more information, see [Amazon EventBridge
targets](https://docs.aws.amazon.com/eventbridge/latest/userguide/eb-targets.html) in the *Amazon EventBridge User Guide*.
## Step Functions events detail reference
All events from AWS services have a common set of fields containing metadata about
the event, such as the AWS service that is the source of the event, the time the event
was generated, the account and region in which the event took place, and others. For
definitions of these general fields, see [Event structure
reference](https://docs.aws.amazon.com/eventbridge/latest/ref/overiew-event-structure.html) in the *Amazon EventBridge User Guide*.
In addition, each event has a `detail` field that contains data specific to
that particular event.
When using EventBridge to select and manage Step Functions events, it's useful to keep the
following in mind:
* The `source` field for all events from Step Functions is set to
`aws.states`.
* The `detail-type` field specifies the event type.
For example, `Step Functions Execution Status Change`.
* The `detail` field contains the data that is specific to that
particular event.
For information on constructing event patterns that enable rules to match Step Functions
events, see [Event patterns](https://docs.aws.amazon.com/eventbridge/latest/userguide/eb-event-patterns.html) in the *Amazon EventBridge User Guide*.
For more information on events and how EventBridge processes them, see [Amazon EventBridge
events](https://docs.aws.amazon.com/eventbridge/latest/userguide/eb-events.html) in the *Amazon EventBridge User Guide*.
### Execution Status Change
Represents a change in the status of a state machine execution.
The `source` and `detail-type` fields are included below because they contain specific values for Step Functions events. For
definitions of the other metadata fields that are included in all events, see [Event structure
reference](https://docs.aws.amazon.com/eventbridge/latest/ref/overiew-event-structure.html) in the *Amazon EventBridge User Guide*.
#### Event structure
```
`{
. . .,
"detail-type": "Step Functions Execution Status Change",
"source"": "aws.states",
. . .,
"detail"": {
"executionArn"" : "string",
"input" : "string",
"inputDetails" : {
"included" : "boolean"
},
"name" : "string",
"output" : "string",
"outputDetails" : {
"included" : "boolean"
},
"startDate" : "integer",
"stateMachineArn" : "string",
"stopDate" : "integer",
"status" : "RUNNING | SUCCEEDED | FAILED | TIMED\_OUT | ABORTED | PENDING\_REDRIVE"
}
}`
```
#### Remarks
An Execution Status Change event can contain an input property in its definition. For
some events, an Execution Status Change event can also contain an output property in its
definition.
* If the combined escaped input and escaped output sent to EventBridge exceeds
248 KiB, then the input will be excluded. Similarly, if the
escaped output exceeds 248 KiB, then the output will be
excluded. This is a result of events quotas.
* You can determine whether a payload has been truncated with the
`inputDetails` and `outputDetails` properties. For more
information, see the [`CloudWatchEventsExecutionDataDetails` Data Type](https://docs.aws.amazon.com/step-functions/latest/apireference/API_CloudWatchEventsExecutionDataDetails.html).
* For Standard Workflows, use [DescribeExecution](https://docs.aws.amazon.com/step-functions/latest/apireference/API_DescribeExecution.html) to see the full input and output.
`DescribeExecution` is not available for Express Workflows. If you want to see
the full input/output, you can:
* Wrap your Express Workflow with a Standard Workflow.
* Use Amazon S3 ARNs. For information about using ARNs, see [Using Amazon S3 ARNs instead of passing large payloads in Step Functions](./sfn-best-practices.html#avoid-exec-failures).
###### Example Execution Status Change: execution started
```
`{
"version": "0",
"id": "315c1398-40ff-a850-213b-158f73e60175",
"detail-type": "Step Functions Execution Status Change",
"source": "aws.states",
"account": "`account-id`",
"time": "2019-02-26T19:42:21Z",
"region": "us-east-2",
"resources": [
"arn:aws:states:us-east-2:`account-id`:execution:state-machine-name:execution-name"
],
"detail": {
"executionArn": "arn:aws:states:us-east-2:`account-id`:execution:state-machine-name:execution-name",
"stateMachineArn": "arn:aws::states:us-east-2:`account-id`:stateMachine:state-machine",
"name": "execution-name",
"status": "RUNNING",
"startDate": 1551225271984,
"stopDate": null,
"input": "{}",
"inputDetails": {
"included": true
},
"output": null,
"outputDetails": null
}
}`
```
###### Example Execution Status Change: execution succeeded
```
`{
"version": "0",
"id": "315c1398-40ff-a850-213b-158f73e60175",
"detail-type": "Step Functions Execution Status Change",
"source": "aws.states",
"account": "`account-id`",
"time": "2019-02-26T19:42:21Z",
"region": "us-east-2",
"resources": [
"arn:aws:states:us-east-2:`account-id`:execution:state-machine-name:execution-name"
],
"detail": {
"executionArn": "arn:aws:states:us-east-2:`account-id`:execution:state-machine-name:execution-name",
"stateMachineArn": "arn:aws:states:us-east-2:`account-id`:stateMachine:state-machine",
"name": "execution-name",
"status": "SUCCEEDED",
"startDate": 1547148840101,
"stopDate": 1547148840122,
"input": "{}",
"inputDetails": {
"included": true
},
"output": "\\"Hello World!\\"",
"outputDetails": {
"included": true
}
}
}`
```
###### Example Execution Status Change:
execution failed
```
`{
"version": "0",
"id": "315c1398-40ff-a850-213b-158f73e60175",
"detail-type": "Step Functions Execution Status Change",
"source": "aws.states",
"account": "`account-id`",
"time": "2019-02-26T19:42:21Z",
"region": "us-east-2",
"resources": [
"arn:aws:states:us-east-2:`account-id`:execution:state-machine-name:execution-name"
],
"detail": {
"executionArn": "arn:aws:states:us-east-2:`account-id`:execution:state-machine-name:execution-name",
"stateMachineArn": "arn:aws:states:us-east-2:`account-id`:stateMachine:state-machine",
"name": "execution-name",
"status": "FAILED",
"startDate": 1551225146847,
"stopDate": 1551225151881,
"input": "{}",
"inputDetails": {
"included": true
},
"output": null,
"outputDetails": null
}
}`
```
###### Example Execution Status Change: timed-out
```
`{
"version": "0",
"id": "315c1398-40ff-a850-213b-158f73e60175",
"detail-type": "Step Functions Execution Status Change",
"source": "aws.states",
"account": "`account-id`",
"time": "2019-02-26T19:42:21Z",
"region": "us-east-2",
"resources": [
"arn:aws:states:us-east-2:`account-id`:execution:state-machine-name:execution-name"
],
"detail": {
"executionArn": "arn:aws:states:us-east-2:`account-id`:execution:state-machine-name:execution-name",
"stateMachineArn": "arn:aws:states:us-east-2:`account-id`:stateMachine:state-machine",
"name": "execution-name",
"status": "TIMED\_OUT",
"startDate": 1551224926156,
"stopDate": 1551224927157,
"input": "{}",
"inputDetails": {
"included": true
},
"output": null,
"outputDetails": null
`
```
###### Example Execution Status Change:
aborted
```
`{
"version": "0",
"id": "315c1398-40ff-a850-213b-158f73e60175",
"detail-type": "Step Functions Execution Status Change",
"source": "aws.states",
"account": "`account-id`",
"time": "2019-02-26T19:42:21Z",
"region": "us-east-2",
"resources": [
"arn:aws:states:us-east-2:`account-id`:execution:state-machine-name:execution-name"
],
"detail": {
"executionArn": "arn:aws:states:us-east-2:`account-id`:execution:state-machine-name:execution-name",
"stateMachineArn": "arn:aws:states:us-east-2:`account-id`:stateMachine:state-machine",
"name": "execution-name",
"status": "ABORTED",
"startDate": 1551225014968,
"stopDate": 1551225017576,
"input": "{}",
"inputDetails": {
"included": true
},
"output": null,
"outputDetails": null
}
}`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Metrics in CloudWatch
API calls in CloudTrail
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.