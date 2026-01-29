---
url: https://docs.aws.amazon.com/step-functions/latest/dg/sfn-best-practices.html
title: sfn best practices.html
word_count: 2809
filtered: true
elements_removed: 0
density_score: 0.84
---

Best practices for Step Functions - AWS Step Functions
Best practices for Step Functions - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#sfn-best-practices)
[Optimizing with Express Workflows](#cost-opt-exp-workflows)[Tagging resources](#concepts-tagging)[Using timeouts to avoid stuck executions](#sfn-stuck-execution)[Using Amazon S3 to pass large data](#avoid-exec-failures)[Avoiding execution history quota](#bp-history-limit)[Handling Lambda exceptions](#bp-lambda-serviceexception)[Avoiding latency for activity tasks](#bp-activity-pollers)[Avoid log policy resource limits](#bp-cwl)
###### Managing state and transforming data
Learn about [Passing data between states with variables](./workflow-variables.html) and [Transforming data with JSONata](./transforming-data.html).
The following topics are best practices to help you manage and optimize your Step Functions workflows.
###### List of best practices
* [Optimizing costs using Express Workflows](#cost-opt-exp-workflows)
* [Tagging state machines and activities in Step Functions](#concepts-tagging)
* [Using timeouts to avoid stuck Step Functions workflow executions](#sfn-stuck-execution)
* [Using Amazon S3 ARNs instead of passing large payloads in Step Functions](#avoid-exec-failures)
* [Starting new executions to avoid reaching the history quota in Step Functions](#bp-history-limit)
* [Handle transient Lambda service exceptions](#bp-lambda-serviceexception)
* [Avoiding latency when polling for activity tasks](#bp-activity-pollers)
* [Avoiding CloudWatch resource policy size limits](#bp-cwl)
## Optimizing costs using Express Workflows
Step Functions determines pricing for Standard and Express workflows based on the workflow type you
use to build your state machines. To optimize the cost of your serverless workflows, you can
follow either or both of the following recommendations:
For information about how choosing a Standard or Express workflow type affects billing, see [AWS Step Functions Pricing](https://aws.amazon.com/step-functions/pricing/).
### Nest Express workflows inside Standard workflows
Step Functions runs workflows that have a finite duration and number of steps. Some workflows
may complete execution within a short period of time. Others may require a combination
of both long-running and high-event-rate workflows. With Step Functions, you can build large,
complex workflows out of multiple smaller, simpler workflows.
For example, to build an order processing workflow, you can include all non-idempotent
actions into a Standard workflow. This could include actions, such as approving order
through human interaction and processing payments. You can then combine a series of
idempotent actions, such as sending payment notifications and updating product
inventory, in an Express workflow. You can nest this Express workflow within the
Standard workflow. In this example, the Standard workflow is known as the
*parent state machine*. The nested Express workflow is known as a
*child state machine*.
### Migrate Standard workflows to Express workflows
You should consider migrating your Standard workflows to Express workflows if they meet
the following requirements:
* Your workflow must complete execution within five minutes.
* Your workflow conforms to an *at-least-once* execution
model, which means each step in the workflow may run more than exactly
once.
* Your workflow does **not** use the `[.waitForTaskToken](./connect-to-resource.html#connect-wait-token)` or `[.sync](./connect-to-resource.html#connect-sync)` service integration patterns.
###### Important
Express workflows use Amazon CloudWatch Logs to record execution histories. You will incur
additional costs when using CloudWatch Logs.
###### To migrate a Standard workflow to an Express workflow using the console
1. Open the [Step Functions console](https://console.aws.amazon.com/states/home?region=us-east-1#/).
2. On the **State machines** page, choose a Standard type state machine to open it.
###### Tip
From the **Any type** dropdown list, choose **Standard** to filter the state machines list and view only Standard workflows.
3. Choose **Copy to new**.
Workflow Studio opens in [Design mode](./workflow-studio.html#wfs-interface-design-mode) displaying workflow of the state machine you selected.
4. (Optional) Update the workflow design.
5. Specify a name for your state machine. To do this, choose the edit icon next to the default state machine name of **MyStateMachine**. Then, in **State machine configuration**, specify a name in the **State machine name** box.
6. (Optional) In **State machine configuration**, specify other workflow settings, such as state machine type and its execution role.
Make sure that for **Type**, you choose **Express**. Keep all the other default selections on **State machine settings**.
###### Note
If you're migrating a Standard workflow previously defined in [AWS CDK](https://docs.aws.amazon.com/cdk/api/latest/docs/aws-stepfunctions-readme.html) or AWS SAM, you must change the value of `Type` and
`Resource` name.
7. In the **Confirm role creation** dialog box, choose **Confirm** to continue.
You can also choose **View role settings** to go back to **State machine configuration**.
###### Note
If you delete the IAM role that Step Functions creates, Step Functions can't
recreate it later. Similarly, if you modify the role (for example, by
removing Step Functions from the principals in the IAM policy), Step Functions can't
restore its original settings later.
For more information about best practices and guidelines when you manage
cost-optimization for your workflows, see [Building
cost-effective AWS Step Functions workflows](https://aws.amazon.com/blogs/compute/building-cost-effective-aws-step-functions-workflows/).
## Tagging state machines and activities in Step Functions
AWS Step Functions supports tagging state machines (both Standard and Express) and activities.
Tags can help you track and manage your resources and provide
better security in your AWS Identity and Access Management (IAM) policies. After tagging Step Functions resources, you can
manage them with AWS Resource Groups. To learn how, see the [AWS Resource Groups User Guide](https://docs.aws.amazon.com/ARG/latest/userguide/).
For tag-based authorization, state machine execution resources as shown in the following example inherit the tags associated with a state machine.
```
`arn:`partition`:states:`region`:`account-id`:execution:`&lt;StateMachineName&gt;:&lt;ExecutionId&gt;``
```
When you call [DescribeExecution](https://docs.aws.amazon.com/step-functions/latest/apireference/API_DescribeExecution.html) or other APIs in which you specify the execution resource ARN, Step Functions uses tags associated with the state machine to accept or deny the request while performing tag-based authorization. This helps you allow or deny access to state machine executions at the state machine level.
To review the restrictions related to resource tagging, see [Restrictions related to tagging](./service-quotas.html#sfn-limits-tagging).
### Tagging for Cost Allocation
You can use cost
allocation tags to identify the purpose of a state machine and reflect that organization in
your AWS bill. Sign up to get your AWS account bill to include the tag keys and values. See [Setting Up a
Monthly Cost Allocation Report](https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/configurecostallocreport.html#allocation-report) in the
*AWS Billing User Guide* for details on setting up reports.
For example, you could add tags that represent your cost center and purpose of your
Step Functions resources, as follows.
|Resource|Key|Value|
|`StateMachine1`|`Cost Center`|`34567`|
|`Application`|`Image processing`|
|`StateMachine2`|`Cost Center`|`34567`|
|`Application`|`Rekognition processing`|
### Tagging for Security
IAM supports controlling access to resources based on tags. To control access based
on tags, provide information about your resource tags in the condition element of an
IAM policy.
For example, you could restrict access to all Step Functions resources that include a tag with
the key `environment` and the value `production`.
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Effect": "Deny",
"Action": [
"states:TagResource",
"states:DeleteActivity",
"states:DeleteStateMachine",
"states:StopExecution"
],
"Resource": "\*",
"Condition": {
"StringEquals": {"aws:ResourceTag/environment": "production"}
}
}
]
}`
`
```
For more information, see [Controlling
Access Using Tags](https://docs.aws.amazon.com/IAM/latest/UserGuide/access_tags.html) in the IAM User Guide.
### Managing tags in the Step Functions console
You can view and manage tags for your state machines in the Step Functions
console. From the **Details** page of a state machine, select
**Tags**.
### Managing tags with Step Functions API Actions
To manage tags using the Step Functions API, use the following API actions:
* [`ListTagsForResource`](https://docs.aws.amazon.com/step-functions/latest/apireference/API_ListTagsForResource.html)
* [`TagResource`](https://docs.aws.amazon.com/step-functions/latest/apireference/API_TagResource.html)
* [`UntagResource`](https://docs.aws.amazon.com/step-functions/latest/apireference/API_UntagResource.html)
## Using timeouts to avoid stuck Step Functions workflow executions
By default, the Amazon States Language doesn't specify timeouts for state machine definitions. Without an explicit timeout, Step Functions often relies solely on a response from an activity worker to know that
a task is complete. If something goes wrong and the `TimeoutSeconds` field isn't specified for an `Activity` or `Task` state, an execution is stuck
waiting for a response that will never come.
To avoid this situation, specify a reasonable timeout when you create a `Task` in your state machine. For example:
```
`"ActivityState": {
"Type": "Task",
"Resource": "arn:aws:states:`region`:`account-id`:activity:HelloWorld",
"TimeoutSeconds": 300,
"Next": "NextState"
}`
```
If you use a [callback with a task token
(.waitForTaskToken)](./connect-to-resource.html#connect-wait-token), we recommend that you use heartbeats and add the `HeartbeatSeconds` field in your `Task` state definition. You can set `HeartbeatSeconds` to be less than the task timeout so if your workflow fails with a heartbeat error then you know it's because of the task failure instead of the task taking a long time to complete.
```
`{
"StartAt": "Push to SQS",
"States": {
"Push to SQS": {
"Type": "Task",
"Resource": "arn:aws:states:::sqs:sendMessage.waitForTaskToken",
"HeartbeatSeconds": 600,
"Parameters": {
"MessageBody": { "myTaskToken.$": "$$.Task.Token" },
"QueueUrl": "https://sqs.us-east-1.amazonaws.com/`account-id`/push-based-queue"
},
"ResultPath": "$.SQS",
"End": true
}
}
}`
```
For more information, see [Task workflow state](./state-task.html) in the Amazon States Language documentation.
###### Note
You can set a timeout for your state machine using the `TimeoutSeconds` field in your Amazon States Language definition. For more information, see [State machine structure in Amazon States Language for Step Functions workflows](./statemachine-structure.html).
## Using Amazon S3 ARNs instead of passing large payloads in Step Functions
Executions that pass large payloads of data between states can be terminated. If the data you are passing between states might grow to over 256 KiB, use Amazon Simple Storage Service (Amazon S3) to store
the data, and parse the Amazon Resource Name (ARN) of the bucket in the `Payload` parameter to get the bucket name and key value. Alternatively, adjust your implementation so that you pass
smaller payloads in your executions.
In the following example, a state machine passes input to an AWS Lambda function, which processes a JSON file in an Amazon S3 bucket. After you run this state machine, the Lambda function
reads the contents of the JSON file, and returns the file contents as output.
###### Create the Lambda function
The following Lambda function named ``pass-large-payload`` reads the contents of a JSON file stored in a specific Amazon S3 bucket.
###### Note
After you create this Lambda function, make sure you provide its IAM role the appropriate permission to read from an Amazon S3 bucket. For example, attach the
**AmazonS3ReadOnlyAccess** permission to the Lambda function's role.
```
`import json
import boto3
import io
import os
s3 = boto3.client('s3')
def lambda\_handler(event, context):
event = event['Input']
final\_json = str()
s3 = boto3.resource('s3')
bucket = event['bucket'].split(':')[-1]
filename = event['key']
directory = "/tmp/{}".format(filename)
s3.Bucket(bucket).download\_file(filename, directory)
with open(directory, "r") as jsonfile:
final\_json = json.load(jsonfile)
os.popen("rm -rf /tmp")
return final\_json`
```
###### Create the state machine
The following state machine invokes the Lambda function you previously created.
```
`{
"StartAt":"Invoke Lambda function",
"States":{
"Invoke Lambda function":{
"Type":"Task",
"Resource":"arn:aws:states:::lambda:invoke",
"Parameters":{
"FunctionName":"arn:aws:lambda:us-east-2:123456789012:function:`pass-large-payload`",
"Payload":{
"Input.$":"$"
}
},
"OutputPath": "$.Payload",
"End":true
}
}
}
`
```
Rather than pass a large amount of data in the input, you could save that data in an Amazon S3 bucket, and pass the Amazon Resource Name (ARN) of the bucket in the `Payload` parameter to
get the bucket name and key value. Your Lambda function can then use that ARN to access the data directly. The following is example input for the state machine execution, where the data is
stored in `data.json` in an Amazon S3 bucket named ``amzn-s3-demo-large-payload-json``.
```
`{
"key": "data.json",
"bucket": "`arn:aws:s3:::amzn-s3-demo-large-payload-json`"
}`
```
## Starting new executions to avoid reaching the history quota in Step Functions
AWS Step Functions has a hard quota of 25,000 entries in the execution event history. When an execution reaches 24,999 events, it waits for the next event to happen.
* If the event number 25,000 is `ExecutionSucceeded`, the execution finishes successfully.
* If the event number 25,000 isn't `ExecutionSucceeded`, the `ExecutionFailed` event is logged and the state machine execution fails because of reaching the history limit
To avoid reaching this quota for long-running executions, you can try one of the following workarounds:
* [Use the Map state in Distributed mode](./state-map-distributed.html). In this mode, the `Map` state runs each iteration as a child workflow execution, which enables high concurrency of up to 10,000 parallel child workflow executions. Each child workflow execution has its own, separate execution history from that of the parent workflow.
* Start a new state
machine execution directly from the `Task` state of a running execution. To start such nested workflow executions, use Step Functions'
`[StartExecution](https://docs.aws.amazon.com/step-functions/latest/apireference/API_StartExecution.html)` API action in the parent state machine along with the necessary parameters. For more
information about using nested workflows, see [Start workflow executions from a task state in Step Functions](./concepts-nested-workflows.html) or
[Using a Step Functions API action to continue a new execution](./tutorial-continue-new.html) tutorial.
###### Tip
To deploy an example nested workflow, see [Optimizing costs](https://catalog.workshops.aws/stepfunctions/nested-workflow) in *The AWS Step Functions Workshop*.
* Implement a pattern that uses an AWS Lambda function that can start a new execution of your state machine to split ongoing work across multiple workflow
executions. For more information, see the [Using a Lambda function to continue a new execution in Step Functions](./tutorial-use-lambda-cont-exec.html) tutorial.
## Handle transient Lambda service exceptions
AWS Lambda can occasionally experience transient service errors. In this case, invoking
Lambda results in a 500 error, such as `ClientExecutionTimeoutException`, `ServiceException`,
`AWSLambdaException`, or `SdkClientException`. As a best practice,
proactively handle these exceptions in your state machine to `Retry` invoking your
Lambda function, or to `Catch` the error.
Lambda errors are reported as `Lambda.`ErrorName``. To
retry a Lambda service exception error, you could use the following `Retry`
code.
```
`"Retry": [ {
"ErrorEquals": [ "Lambda.ClientExecutionTimeoutException", "Lambda.ServiceException", "Lambda.AWSLambdaException", "Lambda.SdkClientException"],
"IntervalSeconds": 2,
"MaxAttempts": 6,
"BackoffRate": 2
} ]`
```
###### Note
Unhandled errors in Lambda runtimes were historically reported only as `Lambda.Unknown`. In newer runtimes, timeouts are reported as `Sandbox.Timedout` in the
error output.
When Lambda exceeds the maximum number of invocations, the reported error will be `Lambda.TooManyRequestsException`.
Match on `Lambda.Unknown`, `Sandbox.Timedout`, and `States.TaskFailed` to handle possible errors. You can also use `States.ALL`, but it must be alone and at the end of the list.
For more information about Lambda `Handled` and `Unhandled` errors, see `FunctionError` in the [AWS Lambda Developer Guide](https://docs.aws.amazon.com/lambda/latest/dg/API_Invoke.html#API_Invoke_ResponseSyntax).
For more information, see the following:
* [Retrying after an error](./concepts-error-handling.html#error-handling-retrying-after-an-error)
* [Handling error conditions in a Step Functions
state machine](./tutorial-handling-error-conditions.html)
* [Lambda Invoke
Errors](https://docs.aws.amazon.com/lambda/latest/dg/API_Invoke.html#API_Invoke_Errors)
## Avoiding latency when polling for activity tasks
The `[GetActivityTask](https://docs.aws.amazon.com/step-functions/latest/apireference/API_GetActivityTask.html)` API is designed to provide a [`taskToken`](https://docs.aws.amazon.com/step-functions/latest/apireference/API_GetActivityTask.html#StepFunctions-GetActivityTask-response-taskToken)
*exactly once*. If a `taskToken` is dropped while communicating
with an activity worker, a number of `GetActivityTask` requests can be blocked for
60 seconds waiting for a response until `GetActivityTask` times out.
If you only have a small number of polls waiting for a response, it's possible that all
requests will queue up behind the blocked request and stop. However, if you have a large
number of outstanding polls for each activity Amazon Resource Name (ARN), and some percentage of your
requests are stuck waiting, there will be many more that can still get a
`taskToken` and begin to process work.
For production systems, we recommend at least 100 open polls per activity ARN's at each
point in time. If one poll gets blocked, and a portion of those polls queue up behind it,
there are still many more requests that will receive a `taskToken` to process work
while the `GetActivityTask` request is blocked.
To avoid these kinds of latency problems when polling for tasks:
* Implement your pollers as separate threads from the work in your activity worker
implementation.
* Have at least 100 open polls per activity ARN at each point in time.
###### Note
Scaling to 100 open polls per ARN can be expensive. For example, 100 Lambda functions
polling per ARN is 100 times more expensive than having a single Lambda function with 100
polling threads. To both reduce latency *and* minimize cost, use a
language that has asynchronous I/O, and implement multiple polling threads per worker.
For an example activity worker where the poller threads are separate from the work
threads, see [Example: Activity Worker in Ruby](./concepts-activities.html#example-ruby-activity-worker).
For more information on activities and activity workers see [Learn about Activities in Step Functions](./concepts-activities.html).
## Avoiding CloudWatch resource policy size limits
When you create a state machine with logging, or update an existing state machine to enable logging, Step Functions must update your CloudWatch Logs resource policy with the log group that you specify. CloudWatch Logs resource policies are limited to 5,120 characters.
When CloudWatch Logs detects that a policy approaches the size limit, CloudWatch Logs automatically enables logging for log groups that start with `/aws/vendedlogs/`.
You can prefix your CloudWatch Logs log group names with `/aws/vendedlogs/` to avoid the CloudWatch Logs resource policy size limit. If you create a log group in the Step Functions console, the suggested log group name will already be prefixed with `/aws/vendedlogs/states`.
CloudWatch Logs also has a quota of ten resource policies per region, per account. If you try to enable logging on a state machine that already has ten CloudWatch Logs resource policies in a region for an account, the state machine will not be created or updated. For more information about logging quotas, see [CloudWatch Logs quotas](https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/cloudwatch_limits_cwl.html).
If you are having trouble sending logs to CloudWatch Logs, see [Troubleshooting state machine logging to CloudWatch Logs](./cw-logs.html#troubleshooting-logging-to-cloudwatch).
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Troubleshooting
Service quotas
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.