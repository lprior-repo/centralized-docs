---
url: https://docs.aws.amazon.com/step-functions/latest/dg/procedure-cloud-trail.html
title: Recording Step Functions API calls with AWS CloudTrail
word_count: 1330
filtered: true
elements_removed: 0
density_score: 0.87
---

Recording Step Functions API calls with AWS CloudTrail - AWS Step Functions
Recording Step Functions API calls with AWS CloudTrail - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#procedure-cloud-trail)
[Data events in CloudTrail](#cloudtrail-data-events)[Management events in CloudTrail](#cloudtrail-management-events)[Event examples](#cloudtrail-event-examples)
# Recording Step Functions API calls with AWS CloudTrail
AWS Step Functions is integrated with [AWS CloudTrail](https://docs.aws.amazon.com/awscloudtrail/latest/userguide/cloudtrail-user-guide.html), a service that provides a record of actions taken by a user, role, or an
AWS service. CloudTrail captures all
API calls for Step Functions as events. The calls captured include calls from the Step Functions console
and code calls to the Step Functions API operations. Using the information collected by CloudTrail, you can
determine the request that was made to Step Functions, the IP address from which the request was
made, when it was made, and additional details.
Every event or log entry contains information about who generated the request. The identity
information helps you determine the following:
* Whether the request was made with root user or user credentials.
* Whether the request was made on behalf of an IAM Identity Center user.
* Whether the request was made with temporary security credentials for a role or federated
user.
* Whether the request was made by another AWS service.
CloudTrail is active in your AWS account when you create the account and you automatically have
access to the CloudTrail **Event history**. The CloudTrail **Event
history** provides a viewable, searchable, downloadable, and immutable record of the
past 90 days of recorded management events in an AWS Region. For more information, see [Working
with CloudTrail Event history](https://docs.aws.amazon.com/awscloudtrail/latest/userguide/view-cloudtrail-events.html) in the *AWS CloudTrail User Guide*. There are no CloudTrail
charges for viewing the **Event history**.
For an ongoing record of events in your AWS account past 90 days, create a trail or a
[CloudTrail
Lake](https://docs.aws.amazon.com/awscloudtrail/latest/userguide/cloudtrail-lake.html) event data store.
**CloudTrail trails**
A *trail* enables CloudTrail to deliver log files to an Amazon S3 bucket. All trails created using the AWS Management Console are multi-Region. You can create a single-Region or a multi-Region trail by using the AWS CLI. Creating a multi-Region trail is recommended because you capture activity in all AWS Regions in your account. If you create a single-Region trail, you can view only the events logged in the trail's AWS Region. For more information about trails, see [Creating a trail for your AWS account](https://docs.aws.amazon.com/awscloudtrail/latest/userguide/cloudtrail-create-and-update-a-trail.html) and [Creating a trail for an organization](https://docs.aws.amazon.com/awscloudtrail/latest/userguide/creating-trail-organization.html) in the *AWS CloudTrail User Guide*.
You can deliver one copy of your ongoing management events to your Amazon S3 bucket at no charge from CloudTrail by creating a trail, however, there are Amazon S3 storage charges. For more information about CloudTrail pricing, see [AWS CloudTrail Pricing](https://aws.amazon.com/cloudtrail/pricing/). For information about Amazon S3 pricing, see [Amazon S3 Pricing](https://aws.amazon.com/s3/pricing/).
**CloudTrail Lake event data stores**
*CloudTrail Lake* lets you run SQL-based queries on your events. CloudTrail Lake converts existing events in row-based JSON format to [ Apache ORC](https://orc.apache.org/) format. ORC is a columnar storage format that is optimized for fast retrieval of data. Events are aggregated into *event data stores*, which are immutable collections of events based on criteria that you select by applying [advanced event selectors](https://docs.aws.amazon.com/awscloudtrail/latest/userguide/cloudtrail-lake-concepts.html#adv-event-selectors). The selectors that you apply to an event data store control which events persist and are available for you to query. For more information about CloudTrail Lake, see [Working with AWS CloudTrail Lake](https://docs.aws.amazon.com/awscloudtrail/latest/userguide/cloudtrail-lake.html) in the *AWS CloudTrail User Guide*.
CloudTrail Lake event data stores and queries incur costs. When you create an event data store, you choose the [pricing option](https://docs.aws.amazon.com/awscloudtrail/latest/userguide/cloudtrail-lake-manage-costs.html#cloudtrail-lake-manage-costs-pricing-option) you want to use for the event data store. The pricing option determines the cost for ingesting and storing events, and the default and maximum retention period for the event data store. For more information about CloudTrail pricing, see [AWS CloudTrail Pricing](https://aws.amazon.com/cloudtrail/pricing/).
## Data events in CloudTrail
[Data events](https://docs.aws.amazon.com/awscloudtrail/latest/userguide/logging-data-events-with-cloudtrail.html#logging-data-events) provide information about the resource operations performed on or in a
resource (for example, reading or writing to an Amazon S3
object). These are also known as data
plane operations. Data events are often high-volume activities. By default, CloudTrail doesn’t log
data events. The CloudTrail **Event history** doesn't record data events.
Additional charges apply for data events. For more information about CloudTrail pricing, see
[AWS CloudTrail Pricing](https://aws.amazon.com/cloudtrail/pricing/).
You can log data events for the Step Functions resource types by using the CloudTrail console, AWS CLI,
or CloudTrail API operations. For more information about how to log data events, see [Logging data events with the AWS Management Console](https://docs.aws.amazon.com/awscloudtrail/latest/userguide/logging-data-events-with-cloudtrail.html#logging-data-events-console) and [Logging data events with the AWS Command Line Interface](https://docs.aws.amazon.com/awscloudtrail/latest/userguide/logging-data-events-with-cloudtrail.html#creating-data-event-selectors-with-the-AWS-CLI) in the
*AWS CloudTrail User Guide*.
The following table lists the Step Functions resource types for which you can log data events.
The **Data event type** column shows the value to
choose from the **Data event type** list on the CloudTrail console. The **resources.type value** column shows the `resources.type`
value, which you would specify when configuring advanced event selectors using the AWS CLI or
CloudTrail APIs. The **Data APIs logged to CloudTrail** column shows the API
calls logged to CloudTrail for the resource type.
You can configure advanced event selectors to filter on the `eventName`,
`readOnly`, and `resources.ARN` fields to log only those events that
are important to you. For more information about these fields, see [AdvancedFieldSelector](https://docs.aws.amazon.com/awscloudtrail/latest/APIReference/API_AdvancedFieldSelector.html) in the
*AWS CloudTrail API Reference*.
|Data event type|resources.type value|Data APIs logged to CloudTrail|
|**Step Functions state machine**|`AWS::StepFunctions::StateMachine`|
* InvokeHTTPEndpoint
* StartSyncExecution
|
|**Step Functions activity**|`AWS::StepFunctions::Activity`|
* GetActivityTask
|
## Management events in CloudTrail
[Management events](https://docs.aws.amazon.com/awscloudtrail/latest/userguide/logging-management-events-with-cloudtrail.html#logging-management-events) provide information about management operations that are performed on resources in your AWS account. These are also known as control plane operations. By default, CloudTrail logs management events.
**State Machine**
* [CreateStateMachine](https://docs.aws.amazon.com/step-functions/latest/apireference/API_CreateStateMachine.html)
* [ListStateMachines](https://docs.aws.amazon.com/step-functions/latest/apireference/API_ListStateMachines.html)
* [DescribeStateMachine](https://docs.aws.amazon.com/step-functions/latest/apireference/API_DescribeStateMachine.html)
* [UpdateStateMachine](https://docs.aws.amazon.com/step-functions/latest/apireference/API_UpdateStateMachine.html)
* [DeleteStateMachine](https://docs.aws.amazon.com/step-functions/latest/apireference/API_DeleteStateMachine.html)
* [ValidateStateMachineDefinition](https://docs.aws.amazon.com/step-functions/latest/apireference/API_ValidateStateMachineDefinition.html)
* [TestState](https://docs.aws.amazon.com/step-functions/latest/apireference/API_TestState.html)
**State Machine Alias**
* [CreateStateMachineAlias](https://docs.aws.amazon.com/step-functions/latest/apireference/API_CreateStateMachineAlias.html)
* [ListStateMachineAliases](https://docs.aws.amazon.com/step-functions/latest/apireference/API_ListStateMachineAliases.html)
* [DescribeStateMachineAlias](https://docs.aws.amazon.com/step-functions/latest/apireference/API_DescribeStateMachineAlias.html)
* [UpdateStateMachineAlias](https://docs.aws.amazon.com/step-functions/latest/apireference/API_UpdateStateMachineAlias.html)
* [DeleteStateMachineAlias](https://docs.aws.amazon.com/step-functions/latest/apireference/API_DeleteStateMachineAlias.html)
**State Machine Version**
* [ListStateMachineVersions](https://docs.aws.amazon.com/step-functions/latest/apireference/API_ListStateMachineVersions.html)
* [PublishStateMachineVersion](https://docs.aws.amazon.com/step-functions/latest/apireference/API_PublishStateMachineVersion.html)
* [DeleteStateMachineVersion](https://docs.aws.amazon.com/step-functions/latest/apireference/API_DeleteStateMachineVersion.html)
**Executions**
* [StartExecution](https://docs.aws.amazon.com/step-functions/latest/apireference/API_StartExecution.html)
* [StartSyncExecution](https://docs.aws.amazon.com/step-functions/latest/apireference/API_StartSyncExecution.html)
* [RedriveExecution](https://docs.aws.amazon.com/step-functions/latest/apireference/API_RedriveExecution.html)
* [ListExecutions](https://docs.aws.amazon.com/step-functions/latest/apireference/API_ListExecutions.html)
* [DescribeExecution](https://docs.aws.amazon.com/step-functions/latest/apireference/API_DescribeExecution.html)
* [GetExecutionHistory](https://docs.aws.amazon.com/step-functions/latest/apireference/API_GetExecutionHistory.html)
* [DescribeStateMachineForExecution](https://docs.aws.amazon.com/step-functions/latest/apireference/API_DescribeStateMachineForExecution.html)
* [StopExecution](https://docs.aws.amazon.com/step-functions/latest/apireference/API_StopExecution.html)
**Activity**
* [CreateActivity](https://docs.aws.amazon.com/step-functions/latest/apireference/API_CreateActivity.html)
* [ListActivities](https://docs.aws.amazon.com/step-functions/latest/apireference/API_ListActivities.html)
* [DescribeActivity](https://docs.aws.amazon.com/step-functions/latest/apireference/API_DescribeActivity.html)
* [DeleteActivity](https://docs.aws.amazon.com/step-functions/latest/apireference/API_DeleteActivity.html)
* [GetActivityTask](https://docs.aws.amazon.com/step-functions/latest/apireference/API_GetActivityTask.html)
**Task Token**
* [SendTaskSuccess](https://docs.aws.amazon.com/step-functions/latest/apireference/API_SendTaskSuccess.html)
* [SendTaskHeartbeat](https://docs.aws.amazon.com/step-functions/latest/apireference/API_SendTaskHeartbeat.html)
* [SendTaskFailure](https://docs.aws.amazon.com/step-functions/latest/apireference/API_SendTaskFailure.html)
**MapRun**
* [ListMapRuns](https://docs.aws.amazon.com/step-functions/latest/apireference/API_ListMapRuns.html)
* [DescribeMapRun](https://docs.aws.amazon.com/step-functions/latest/apireference/API_DescribeMapRun.html)
* [UpdateMapRun](https://docs.aws.amazon.com/step-functions/latest/apireference/API_UpdateMapRun.html)
**Tags**
* [ListTagsForResource](https://docs.aws.amazon.com/step-functions/latest/apireference/API_ListTagsForResource.html)
* [TagResource](https://docs.aws.amazon.com/step-functions/latest/apireference/API_TagResource.html)
* [UntagResource](https://docs.aws.amazon.com/step-functions/latest/apireference/API_UntagResource.html)
## Event examples
An event represents a single request from any source and includes information about the requested API operation, the date and time of the operation, request parameters, and so on. CloudTrail log files aren't an ordered stack trace of the public API calls, so events don't appear in any specific order.
The following example shows a CloudTrail **data event** that demonstrates `InvokeHTTPEndpoint`.
```
`{
"eventVersion": "1.09",
"userIdentity": {
"accountId": "`account-id`",
"invokedBy": "states.amazonaws.com"
},
"eventTime": "2024-05-01T01:23:45Z",
"eventSource": "states.amazonaws.com",
"eventName": "InvokeHTTPEndpoint",
"awsRegion": "us-east-1",
"sourceIPAddress": "states.amazonaws.com",
"userAgent": "states.amazonaws.com",
"requestParameters": null,
"responseElements": null,
"eventID": "a1b2c3d4-5678-90ab-cdef-EXAMPLEaaaaa",
"readOnly": false,
"resources": [
{
"accountId": "`account-id`",
"type": "AWS::StepFunctions::StateMachine",
"ARN": "arn:aws:states:`region`:`account-id`:stateMachine:ExampleStateMachine"
}
],
"eventType": "AwsServiceEvent",
"managementEvent": false,
"recipientAccountId": "`account-id`",
"serviceEventDetails": {
"httpMethod": "GET",
"httpEndpoint": "https://example.com"
},
"eventCategory": "Data"
}`
```
The following example shows a CloudTrail **management event** that demonstrates the `CreateStateMachine` operation.
```
`{
"eventVersion": "1.08",
"userIdentity": {
"type": "IAMUser",
"principalId": "AIDAJYDLDBVBI4EXAMPLE",
"arn": "arn:aws:iam::`account-id`:user/test-user",
"accountId": "`account-id`",
"accessKeyId": "AKIAIOSFODNN7EXAMPLE",
"userName": "test-user"
},
"eventTime": "2024-05-01T01:23:45Z",
"eventSource": "states.amazonaws.com",
"eventName": "CreateStateMachine",
"awsRegion": "`region`",
"sourceIPAddress": "`AWS Internal`",
"userAgent": "`AWS Internal`",
"requestParameters": {
"name": "MyStateMachine",
"definition": "HIDDEN\_DUE\_TO\_SECURITY\_REASONS",
"roleArn": "arn:aws:iam::`account-id`:role/MyStateMachineRole",
"type": "STANDARD",
"loggingConfiguration": {
"level": "OFF",
"includeExecutionData": false
},
"tags": [],
"tracingConfiguration": {
"enabled": false
},
"publish": false
},
"responseElements": {
"stateMachineArn": "arn:aws:states:`region`:`account-id`:stateMachine:MyStateMachine",
"creationDate": "May 1, 2024 1:23:45 AM"
},
"requestID": "a1b2c3d4-5678-90ab-cdef-EXAMPLEaaaaa",
"eventID": "a1b2c3d4-5678-90ab-cdef-EXAMPLE11111",
"readOnly": false,
"eventType": "AwsApiCall",
"managementEvent": true,
"recipientAccountId": "`account-id`",
"eventCategory": "Management"
}`
```
For information about CloudTrail record contents, see [CloudTrail
record contents](https://docs.aws.amazon.com/awscloudtrail/latest/userguide/cloudtrail-event-reference-record-contents.html) in the *AWS CloudTrail User Guide*.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Automate event delivery
Logging in CloudWatch Logs
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.