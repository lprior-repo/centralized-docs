---
url: https://docs.aws.amazon.com/step-functions/latest/dg/connect-batch.html
title: Run AWS Batch workloads with Step Functions
word_count: 527
filtered: true
elements_removed: 0
density_score: 0.78
---

Run AWS Batch workloads with Step Functions - AWS Step Functions
Run AWS Batch workloads with Step Functions - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#connect-batch)
[Supported APIs](#connect-batch-api)[IAM policies](#batch-iam)
# Run AWS Batch workloads with Step Functions
You can integrate Step Functions with AWS Batch to run batch computing workloads in the AWS cloud. This page lists the supported
AWS Batch APIs and provides an example `Task` state to perform a batch-processing task.
To learn about integrating with AWS services in Step Functions, see [Integrating services](./integrate-services.html) and [Passing parameters to a service API in Step Functions](./connect-parameters.html).
###### Key features of Optimized AWS Batch integration
* The [Run a Job (.sync)](./connect-to-resource.html#connect-sync) integration pattern is available.
Note that there are no specific optimizations for the [Request Response](./connect-to-resource.html#connect-default) or [Wait for a Callback with Task Token](./connect-to-resource.html#connect-wait-token) integration patterns.
The following shows an example `Task` state that submits an AWS Batch job and waits
for it to complete. Many of the arguments shown are optional.
```
`"Submit Batch Job": {
"Type": "Task",
"Resource": "arn:aws:states:::batch:submitJob.sync",
"Arguments": {
"JobName": "`BATCH\_NAME`",
"JobQueue": "`BATCH\_QUEUE\_ARN`",
"JobDefinition": "`BATCH\_JOB\_DEFINITION\_ARN`",
"ArrayProperties": {
"Size": 10
},
"ContainerOverrides": {
"ResourceRequirements": [
{
"Type": "VCPU",
"Value": "4"
}
]
},
"DependsOn": [
{
"JobId": "myJobId",
"Type": "SEQUENTIAL"
}
],
"PropagateTags": true,
"Arguments": {
"Key1": "value1",
"Key2": 100
},
"RetryStrategy": {
"Attempts": 1
},
"Tags": {
"Tag": "`TAG`"
},
"Timeout": {
"AttemptDurationSeconds": 10
}
}
}`
```
###### Parameters in Step Functions are expressed in PascalCase
Even if the native service API is in camelCase, for example the API action `startSyncExecution`, you specify parameters in PascalCase, such as: `StateMachineArn`.
## IAM policies for calling AWS Batch
The following example templates show how AWS Step Functions generates IAM policies based on the resources in your state machine definition. For more information, see [How Step Functions generates IAM policies for integrated
services](./service-integration-iam-templates.html) and [Discover service integration patterns in Step Functions](./connect-to-resource.html).
Because job ids for `SubmitJob` and `TerminateJob` are generated and therefore only known at runtime, you cannot create a policy that restricts access based on a specific resource.
###### Tip for fine grained access
To add fine grained access to `SubmitJob` and `TerminateJob`, consider using tags for jobs and creating a policy that limits access based on your tags. In addition, the job queue, definition, and consumable resources can be restricted for `SubmitJob` using known resources.
Run a Job (.sync)
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Action": [
"batch:SubmitJob",
"batch:DescribeJobs",
"batch:TerminateJob"
],
"Resource": "\*"
},
{
"Effect": "Allow",
"Action": [
"events:PutTargets",
"events:PutRule",
"events:DescribeRule"
],
"Resource": [
"arn:aws:events:`us-east-1`:`123456789012`:rule/StepFunctionsGetEventsForBatchJobsRule"
]
}
]
}
`
`
```
Request Response
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Action": [
"batch:SubmitJob"
],
"Resource": "\*"
}
]
}`
`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Amazon Athena
Amazon Bedrock
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.