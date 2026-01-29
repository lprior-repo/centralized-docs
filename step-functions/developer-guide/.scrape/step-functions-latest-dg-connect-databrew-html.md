---
url: https://docs.aws.amazon.com/step-functions/latest/dg/connect-databrew.html
title: Start AWS Glue DataBrew jobs with Step Functions
word_count: 357
filtered: true
elements_removed: 0
density_score: 0.80
---

Start AWS Glue DataBrew jobs with Step Functions - AWS Step Functions
Start AWS Glue DataBrew jobs with Step Functions - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#connect-databrew)
[Supported APIs](#connect-databrew-api)[IAM policies](#databrew-iam)
# Start AWS Glue DataBrew jobs with Step Functions
Learn how you can use the DataBrew integration to add data cleaning and
data normalization steps into your analytics and machine learning workflows with Step Functions.
To learn about integrating with AWS services in Step Functions, see [Integrating services](./integrate-services.html) and [Passing parameters to a service API in Step Functions](./connect-parameters.html).
The following includes a `Task` state that starts a request-response DataBrew
job.
```
`"DataBrew StartJobRun": {
"Type": "Task",
"Resource": "arn:aws:states:::databrew:startJobRun",
"Arguments": {
"Name": "sample-proj-job-1"
},
"Next": "NEXT\_STATE"
},
`
```
The following includes a `Task` state that starts a sync DataBrew job.
```
`"DataBrew StartJobRun": {
"Type": "Task",
"Resource": "arn:aws:states:::databrew:startJobRun.sync",
"Arguments": {
"Name": "sample-proj-job-1"
},
"Next": "NEXT\_STATE"
},
`
```
###### Parameters in Step Functions are expressed in PascalCase
Even if the native service API is in camelCase, for example the API action `startSyncExecution`, you specify parameters in PascalCase, such as: `StateMachineArn`.
## IAM policies for calling DataBrew
The following example templates show how AWS Step Functions generates IAM policies based on the resources in your state machine definition. For more information, see [How Step Functions generates IAM policies for integrated
services](./service-integration-iam-templates.html) and [Discover service integration patterns in Step Functions](./connect-to-resource.html).
Run a Job (.sync)
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Action": [
"databrew:startJobRun",
"databrew:listJobRuns",
"databrew:stopJobRun"
],
"Resource": [
"arn:aws:databrew:`us-east-1`:`123456789012`:job/\*"
]
}
]
}`
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
"databrew:startJobRun"
],
"Resource": [
"arn:aws:databrew:`us-east-1`:`123456789012`:job/\*"
]
}
]
}`
`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
AWS Glue
AWS Lambda
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.