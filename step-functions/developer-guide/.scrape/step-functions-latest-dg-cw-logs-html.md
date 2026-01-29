---
url: https://docs.aws.amazon.com/step-functions/latest/dg/cw-logs.html
title: Using CloudWatch Logs to log execution history in Step Functions
word_count: 1419
filtered: true
elements_removed: 0
density_score: 0.91
---

Using CloudWatch Logs to log execution history in Step Functions - AWS Step Functions
Using CloudWatch Logs to log execution history in Step Functions - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#cw-logs)
[Configure logging](#monitoring-logging-configure)[CloudWatch Logs payloads](#cloudwatch-payload)[IAM Policies for logging to CloudWatch Logs](#cloudwatch-iam-policy)[Event log levels](#cloudwatch-log-level)[Troubleshooting](#cloudwatch-log-troubleshooting)
# Using CloudWatch Logs to log execution history in Step Functions
Standard Workflows record execution history in AWS Step Functions, although you can optionally
configure logging to Amazon CloudWatch Logs.
Unlike Standard Workflows, Express Workflows don't record execution history in AWS Step Functions. To
see execution history and results for an Express Workflow, you must configure logging to
Amazon CloudWatch Logs. Publishing logs doesn't block or slow down executions.
###### Log delivery guarantees
Amazon CloudWatch Logs are delivered on a best-effort basis. The completeness and timeliness of log entries are not guaranteed. If you require guaranteed workflow history in Express Workflows, we recommend that you implement workflow steps to record data in an appropriate data storage service such as Amazon DynamoDB. Alternatively, you might consider using **Standard Workflows** for guaranteed execution history.
###### Pricing information
When you configure logging, [CloudWatch Logs charges](https://aws.amazon.com/cloudwatch/pricing) will apply and you will be billed at the vended logs rate. For more information, see **Vended Logs** under the **Logs** tab on the CloudWatch Pricing page.
## Configure logging
When you create a Standard Workflow using the Step Functions console, that state machine will **not** be configured to send logs to CloudWatch Logs. When you create an Express Workflow using the Step Functions console, that state machine will by default
be configured to send logs to CloudWatch Logs.
For Express workflows, Step Functions can create a role with the necessary AWS Identity and Access Management (IAM) policy
for CloudWatch Logs. If you create a Standard Workflow, or an Express Workflow using the API, CLI, or
CloudFormation, Step Functions will not enable logging by default, and you will need ensure your role has the
necessary permissions.
For each execution started from the console, Step Functions provides a link to CloudWatch Logs, configured
with the correct filter to fetch log events specific for that execution.
You can optionally configure customer managed AWS KMS keys to encrypt your logs. See [Data at rest encryption](./encryption-at-rest.html) for details and permission settings.
To configure logging, you can pass the [LoggingConfiguration](https://docs.aws.amazon.com/step-functions/latest/apireference/API_LoggingConfiguration.html) parameter when using [CreateStateMachine](https://docs.aws.amazon.com/step-functions/latest/apireference/API_CreateStateMachine.html) or [UpdateStateMachine](https://docs.aws.amazon.com/step-functions/latest/apireference/API_UpdateStateMachine.html). You can further analyze your data in CloudWatch Logs by using CloudWatch Logs
Insights. For more information see [Analyzing Log Data with CloudWatch
Logs Insights](https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/AnalyzingLogData.html).
## CloudWatch Logs payloads
Execution history events may contain either input or output properties in their
definitions. If escaped
input or escaped output sent to CloudWatch Logs exceeds 248 KiB, it will be truncated as a result of
CloudWatch Logs quotas.
* You can determine whether a payload has been truncated by reviewing the
`inputDetails` and `outputDetails` properties. For more
information, see the [`HistoryEventExecutionDataDetails` Data Type](https://docs.aws.amazon.com/step-functions/latest/apireference/API_HistoryEventExecutionDataDetails.html).
* For Standard Workflows, you can see the full execution history by using [`GetExecutionHistory`](https://docs.aws.amazon.com/step-functions/latest/apireference/API_GetExecutionHistory.html).
* `GetExecutionHistory` is not available for Express Workflows. If you want to
see the full input and output, you can use Amazon S3 ARNs. For more information, see [Using Amazon S3 ARNs instead of passing large payloads in Step Functions](./sfn-best-practices.html#avoid-exec-failures).
## IAM Policies for logging to CloudWatch Logs
You will also need to configure your state machine's execution IAM role to have the proper permission to log to CloudWatch Logs as shown in the following example.
######
IAM policy example
The following is an example policy you can use to configure your permissions. As shown in the following example, you need to specify **\*** in the `Resource` field. CloudWatch API actions, such as CreateLogDelivery and DescribeLogGroups, do not support [Resource types defined by Amazon CloudWatch Logs](https://docs.aws.amazon.com/service-authorization/latest/reference/list_amazoncloudwatchlogs.html#amazoncloudwatchlogs-resources-for-iam-policies). For more information, see [Actions defined by Amazon CloudWatch Logs](https://docs.aws.amazon.com/service-authorization/latest/reference/list_amazoncloudwatchlogs.html#amazoncloudwatchlogs-actions-as-permissions).
* For information about CloudWatch resources, see [CloudWatch Logs resources and operations](https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/iam-access-control-overview-cwl.html#CWL_ARN_Format) in the *Amazon CloudWatch User Guide*.
* For information about the permissions you need to set up sending logs to CloudWatch Logs, see [User permissions](https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/AWS-logs-and-resource-policy.html#AWS-logs-infrastructure-CWL) in the section titled *Logs sent to CloudWatch Logs*.
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Action": [
"logs:CreateLogDelivery",
"logs:CreateLogStream",
"logs:GetLogDelivery",
"logs:UpdateLogDelivery",
"logs:DeleteLogDelivery",
"logs:ListLogDeliveries",
"logs:PutLogEvents",
"logs:PutResourcePolicy",
"logs:DescribeResourcePolicies",
"logs:DescribeLogGroups"
],
"Resource": "\*"
}
]
}`
`
```
## Log levels for Step Functions execution events
Log levels range from `ALL` to `ERROR` to `FATAL` to `OFF`. All event types are logged for `ALL`, no event types are logged when set to `OFF`. For `ERROR` and `FATAL`, see the following table.
For more information about the execution data displayed for Express Workflow executions based on these **Log levels**, see
[Standard and Express console experience differences](./concepts-view-execution-details.html#console-exp-differences).
|Event Type|`ALL`|`ERROR`|`FATAL`|`OFF`|
|
ChoiceStateEntered
|Logged|*Not logged*|*Not logged*|*Not logged*|
|
ChoiceStateExited
|Logged|*Not logged*|*Not logged*|*Not logged*|
|
ExecutionAborted
|Logged|Logged|Logged|*Not logged*|
|
ExecutionFailed
|Logged|Logged|Logged|*Not logged*|
|
ExecutionStarted
|Logged|*Not logged*|*Not logged*|*Not logged*|
|
ExecutionSucceeded
|Logged|*Not logged*|*Not logged*|*Not logged*|
|
ExecutionTimedOut
|Logged|Logged|Logged|*Not logged*|
|
FailStateEntered
|Logged|Logged|*Not logged*|*Not logged*|
|
LambdaFunctionFailed
|Logged|Logged|*Not logged*|*Not logged*|
|LambdaFunctionScheduled|Logged|*Not logged*|*Not logged*|*Not logged*|
|
LambdaFunctionScheduleFailed
|Logged|Logged|*Not logged*|*Not logged*|
|
LambdaFunctionStarted
|Logged|*Not logged*|*Not logged*|*Not logged*|
|
LambdaFunctionStartFailed
|Logged|Logged|*Not logged*|*Not logged*|
|
LambdaFunctionSucceeded
|Logged|*Not logged*|*Not logged*|*Not logged*|
|
LambdaFunctionTimedOut
|Logged|Logged|*Not logged*|*Not logged*|
|
MapIterationAborted
|Logged|Logged|*Not logged*|*Not logged*|
|
MapIterationFailed
|Logged|Logged|*Not logged*|*Not logged*|
|
MapIterationStarted
|Logged|*Not logged*|*Not logged*|*Not logged*|
|
MapIterationSucceeded
|Logged|*Not logged*|*Not logged*|*Not logged*|
|
MapRunAborted
|Logged|Logged|*Not logged*|*Not logged*|
|
MapRunFailed
|Logged|Logged|*Not logged*|*Not logged*|
|
MapStateAborted
|Logged|Logged|*Not logged*|*Not logged*|
|
MapStateEntered
|Logged|*Not logged*|*Not logged*|*Not logged*|
|
MapStateExited
|Logged|*Not logged*|*Not logged*|*Not logged*|
|
MapStateFailed
|Logged|Logged|*Not logged*|*Not logged*|
|
MapStateStarted
|Logged|*Not logged*|*Not logged*|*Not logged*|
|
MapStateSucceeded
|Logged|*Not logged*|*Not logged*|*Not logged*|
|
ParallelStateAborted
|Logged|Logged|*Not logged*|*Not logged*|
|
ParallelStateEntered
|Logged|*Not logged*|*Not logged*|*Not logged*|
|
ParallelStateExited
|Logged|*Not logged*|*Not logged*|*Not logged*|
|ParallelStateFailed|Logged|Logged|*Not logged*|*Not logged*|
|
ParallelStateStarted
|Logged|*Not logged*|*Not logged*|*Not logged*|
|
ParallelStateSucceeded
|Logged|*Not logged*|*Not logged*|*Not logged*|
|
PassStateEntered
|Logged|*Not logged*|*Not logged*|*Not logged*|
|
PassStateExited
|Logged|*Not logged*|*Not logged*|*Not logged*|
|
SucceedStateEntered
|Logged|*Not logged*|*Not logged*|*Not logged*|
|
SucceedStateExited
|Logged|*Not logged*|*Not logged*|*Not logged*|
|
TaskFailed
|Logged|Logged|*Not logged*|*Not logged*|
|
TaskScheduled
|Logged|*Not logged*|*Not logged*|*Not logged*|
|TaskStarted|Logged|*Not logged*|*Not logged*|*Not logged*|
|
TaskStartFailed
|Logged|Logged|*Not logged*|*Not logged*|
|
TaskStateAborted
|Logged|Logged|*Not logged*|*Not logged*|
|
TaskStateEntered
|Logged|*Not logged*|*Not logged*|*Not logged*|
|TaskStateExited|Logged|*Not logged*|*Not logged*|*Not logged*|
|TaskSubmitFailed|Logged|Logged|*Not logged*|*Not logged*|
|TaskSubmitted|Logged|*Not logged*|*Not logged*|*Not logged*|
|TaskSucceeded|Logged|*Not logged*|*Not logged*|*Not logged*|
|TaskTimedOut|Logged|Logged|*Not logged*|*Not logged*|
|WaitStateAborted|Logged|Logged|*Not logged*|*Not logged*|
|WaitStateEntered|Logged|*Not logged*|*Not logged*|*Not logged*|
|WaitStateExited|Logged|*Not logged*|*Not logged*|*Not logged*|
## Troubleshooting logging to CloudWatch Logs
If your state machine cannot send logs to CloudWatch Logs or you receive the error:
"`AccessDeniedException : The state machine IAM Role is not authorized to access
the Log Destination`", try the following steps:
1. Verify your state machine's execution role has permission to log to CloudWatch Logs.
When you call [CreateStateMachine](https://docs.aws.amazon.com/step-functions/latest/apireference/API_CreateStateMachine.html) or
[UpdateStateMachine](https://docs.aws.amazon.com/step-functions/latest/apireference/API_UpdateStateMachine.html) API endpoints, make sure the IAM role specified in the `roleArn` parameter provides the necessary permissions, shown in the preceding IAM policy example.
2. Verify the CloudWatch Logs resource policy does not exceed the 5,120 character limit.
If the policy exceeds the character limit, prefix your log group names with
`/aws/vendedlogs/states` to grant permissions to your state machines
and avoid the limit.
When you create a log group in the Step Functions console, the suggested log group
names are already prefixed with `/aws/vendedlogs/states`. For more
information on logging best practices, see [Avoiding CloudWatch resource policy size limits](./sfn-best-practices.html#bp-cwl).
3. Verify the number of CloudWatch Logs log resource policies in the account is less than
**ten**.
CloudWatch Logs has a quota of ten resource policies per region, per account. If you try
to enable logging on a state machine that already has ten resource policies, the
state machine will not be created nor updated, and you will receive an error.
For more information about logging quotas, see [CloudWatch Logs quotas](https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/cloudwatch_limits_cwl.html)
To verify the problem, check the number of resource policies using the CLI
command:
[`aws logs describe-resource-policies`](https://awscli.amazonaws.com/v2/documentation/api/latest/reference/logs/describe-resource-policies.html)
To resolve the problem, modify your existing resource policies.
First, back up the existing policies. Then, join similar actions or resources
into a new policy and use the following CLI command to create a new delivery
source in the account:
[`aws logs put-delivery-source`](https://awscli.amazonaws.com/v2/documentation/api/latest/reference/logs/put-delivery-source.html)
After backing up and updating the policies, remove any unused policies with
the following command:
[`aws logs delete-resource-policy --policy-name
&lt;PolicyNameToBeDeleted&gt;`](https://docs.aws.amazon.com/cli/latest/reference/logs/delete-resource-policy.html)
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
API calls in CloudTrail
Trace data in X-Ray
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.