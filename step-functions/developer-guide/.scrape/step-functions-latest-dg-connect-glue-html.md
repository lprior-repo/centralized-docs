---
url: https://docs.aws.amazon.com/step-functions/latest/dg/connect-glue.html
title: Start an AWS Glue job with Step Functions
word_count: 380
filtered: true
elements_removed: 0
density_score: 0.85
---

Start an AWS Glue job with Step Functions - AWS Step Functions
Start an AWS Glue job with Step Functions - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#connect-glue)
[Supported APIs](#connect-glue-api)[IAM policies](#glue-iam)
# Start an AWS Glue job with Step Functions
Learn to use Step Functions to start a job run on AWS Glue. This page lists the supported API actions and provides an
example `Task` state to start a AWS Glue job.
To learn about integrating with AWS services in Step Functions, see [Integrating services](./integrate-services.html) and [Passing parameters to a service API in Step Functions](./connect-parameters.html).
###### Key features of Optimized AWS Glue integration
* The [Run a Job (.sync)](./connect-to-resource.html#connect-sync) integration pattern is available.
* The `JobName` field is extracted from the request and inserted into the response, which normally only contains `JobRunID`.
The following includes a `Task` state that starts an AWS Glue job.
```
`"Glue StartJobRun": {
"Type": "Task",
"Resource": "arn:aws:states:::glue:startJobRun.sync",
"Arguments": {
"JobName": "GlueJob-JTrRO5l98qMG"
},
"Next": "ValidateOutput"
},
`
```
###### Parameters in Step Functions are expressed in PascalCase
Even if the native service API is in camelCase, for example the API action `startSyncExecution`, you specify parameters in PascalCase, such as: `StateMachineArn`.
## IAM policies for calling AWS Glue
The following example templates show how AWS Step Functions generates IAM policies based on the resources in your state machine definition. For more information, see [How Step Functions generates IAM policies for integrated
services](./service-integration-iam-templates.html) and [Discover service integration patterns in Step Functions](./connect-to-resource.html).
AWS Glue does not have resource-based control.
Run a Job (.sync)
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Action": [
"glue:StartJobRun",
"glue:GetJobRun",
"glue:GetJobRuns",
"glue:BatchStopJobRun"
],
"Resource": "\*"
}
]
}
`
`
```
Request Response and Callback (.waitForTaskToken)
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Action": [
"glue:StartJobRun"
],
"Resource": "\*"
}
]
}`
`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Amazon EventBridge
AWS Glue DataBrew
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.