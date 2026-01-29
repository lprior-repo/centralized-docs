---
url: https://docs.aws.amazon.com/step-functions/latest/dg/connect-emr-serverless.html
title: Create and manage Amazon EMR Serverless applications with Step Functions
word_count: 2699
filtered: true
elements_removed: 0
density_score: 0.80
---

Create and manage Amazon EMR Serverless applications with Step Functions - AWS Step Functions
Create and manage Amazon EMR Serverless applications with Step Functions - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#connect-emr-serverless)
[Service integration APIs](#connect-emr-serverless-custom-apis)[Integration use cases](#connect-emr-serverless-use-cases)[IAM policies](#emr-serverless-iam)
# Create and manage Amazon EMR Serverless applications with Step Functions
Learn how to create, start, stop, and delete applications on EMR Serverless using Step Functions. This page lists the supported APIs and provides
example `Task` states to perform common use cases.
To learn about integrating with AWS services in Step Functions, see [Integrating services](./integrate-services.html) and [Passing parameters to a service API in Step Functions](./connect-parameters.html).
###### Key features of Optimized EMR Serverless integration
* The Optimized EMR Serverless service integration has a
customized set of [APIs](#connect-emr-serverless-custom-apis)
that wrap the underlying EMR Serverless APIs. Because of this
customization, the optimized EMR Serverless integration differs
significantly from the AWS SDK service integration.
* In addition, the optimized EMR Serverless integration supports
[Run a Job (.sync)](./connect-to-resource.html#connect-sync) integration
pattern.
* The [Wait for a Callback with Task Token](./connect-to-resource.html#connect-wait-token)
integration pattern is **not** supported.
## EMR Serverless service integration APIs
To integrate AWS Step Functions with EMR Serverless, you can use the following six EMR Serverless service integration APIs. These service integration APIs are similar to the corresponding EMR Serverless APIs, with some differences in the fields that are passed and in the responses that are returned.
The following table describes the differences between each EMR Serverless service integration API and its corresponding EMR Serverless API.
|EMR Serverless service integration API|Corresponding EMR Serverless API|Differences|
|
*createApplication*
Creates an application.
EMR Serverless is linked to a unique type of IAM role known as a service-linked role. For `createApplication` and `createApplication.sync` to work, you must have configured the necessary permissions to create the service-linked role `AWSServiceRoleForAmazonEMRServerless`. For more information about this, including a statement you can add to your IAM permissions policy, see [Using service-linked roles for EMR Serverless](https://docs.aws.amazon.com/emr/latest/EMR-Serverless-UserGuide/using-service-linked-roles.html).
|[CreateApplication](https://docs.aws.amazon.com/emr-serverless/latest/APIReference/API_CreateApplication.html)|None|
|
*createApplication.sync*
Creates an application.
|[CreateApplication](https://docs.aws.amazon.com/emr-serverless/latest/APIReference/API_CreateApplication.html)|
No differences between the requests and responses of the EMR Serverless API and EMR Serverless service integration API. However, *createApplication.sync* waits for the application to reach the `CREATED` state.
|
|
*startApplication*
Starts a specified application and initializes the application's initial capacity if configured.
|[StartApplication](https://docs.aws.amazon.com/emr-serverless/latest/APIReference/API_StartApplication.html)|
The EMR Serverless API response doesn't contain any data, but the EMR Serverless service integration API response includes the following data.
```
`{
"ApplicationId": "string"
}`
```
|
|
*startApplication.sync*
Starts a specified application and initializes the initial capacity if configured.
|[StartApplication](https://docs.aws.amazon.com/emr-serverless/latest/APIReference/API_StartApplication.html)|
The EMR Serverless API response doesn't contain any data, but the EMR Serverless service integration API response includes the following data.
```
`{
"ApplicationId": "string"
}`
```
Also, *startApplication.sync* waits for the application to reach the `STARTED` state.
|
|
*stopApplication*
Stops a specified application and releases initial capacity if configured. All scheduled and running jobs must be completed or cancelled before stopping an application.
|[StopApplication](https://docs.aws.amazon.com/emr-serverless/latest/APIReference/API_StopApplication.html)|
The EMR Serverless API response doesn't contain any data, but the EMR Serverless service integration API response includes the following data.
```
`{
"ApplicationId": "string"
}`
```
|
|
*stopApplication.sync*
Stops a specified application and releases initial capacity if configured. All scheduled and running jobs must be completed or cancelled before stopping an application.
|[StopApplication](https://docs.aws.amazon.com/emr-serverless/latest/APIReference/API_StopApplication.html)|
The EMR Serverless API response doesn't contain any data, but the EMR Serverless service integration API response includes the following data.
```
`{
"ApplicationId": "string"
}`
```
Also, *stopApplication.sync* waits for the application to reach the `STOPPED` state.
|
|
*deleteApplication*
Deletes an application. An application must be in the `STOPPED` or `CREATED` state in order to be deleted.
|[DeleteApplication](https://docs.aws.amazon.com/emr-serverless/latest/APIReference/API_DeleteApplication.html)|
The EMR Serverless API response doesn't contain any data, but the EMR Serverless service integration API response includes the following data.
```
`{
"ApplicationId": "string"
}`
```
|
|
*deleteApplication.sync*
Deletes an application. An application must be in the `STOPPED` or `CREATED` state in order to be deleted.
|[DeleteApplication](https://docs.aws.amazon.com/emr-serverless/latest/APIReference/API_DeleteApplication.html)|
The EMR Serverless API response doesn't contain any data, but the EMR Serverless service integration API response includes the following data.
```
`{
"ApplicationId": "string"
}`
```
Also, *stopApplication.sync* waits for the application to reach the `TERMINATED` state.
|
|
*startJobRun*
Starts a job run.
|[StartJobRun](https://docs.aws.amazon.com/emr-serverless/latest/APIReference/API_StartJobRun.html)|None|
|
*startJobRun.sync*
Starts a job run.
|[StartJobRun](https://docs.aws.amazon.com/emr-serverless/latest/APIReference/API_StartJobRun.html)|
No differences between the requests and responses of the EMR Serverless API and EMR Serverless service integration API. However, *startJobRun.sync* waits for the application to reach the `SUCCESS` state.
|
|
*cancelJobRun*
Cancels a job run.
|[CancelJobRun](https://docs.aws.amazon.com/emr-serverless/latest/APIReference/API_CancelJobRun.html)|None|
|
*cancelJobRun.sync*
Cancels a job run.
|[CancelJobRun](https://docs.aws.amazon.com/emr-serverless/latest/APIReference/API_CancelJobRun.html)|
No differences between the requests and responses of the EMR Serverless API and EMR Serverless service integration API. However, *cancelJobRun.sync* waits for the application to reach the `CANCELLED` state.
|
## EMR Serverless integration use cases
For the Optimized EMR Serverless service integration, we recommend that you create a single application, and then use that application to run multiple jobs. For example, in a single state machine, you can include multiple [startJobRun](https://docs.aws.amazon.com/emr-serverless/latest/APIReference/API_StartJobRun.html) requests, all of which use the same application. The following [Task workflow state](./state-task.html) state examples show use cases to integrate EMR Serverless APIs with Step Functions. For information about other use cases of EMR Serverless, see [What is Amazon EMR Serverless](https://docs.aws.amazon.com/emr/latest/EMR-Serverless-UserGuide/emr-serverless.html).
###### Tip
To deploy an example of a state machine that integrates with EMR Serverless for running multiple jobs;, see [Run an EMR Serverless job](./sample-emr-serverless-job.html).
* [Create an application](#connect-emr-serverless-task-state-createapp)
* [Start an application](#connect-emr-serverless-task-state-startapp)
* [Stop an application](#connect-emr-serverless-task-state-stopapp)
* [Delete an application](#connect-emr-serverless-task-state-deleteapp)
* [Start a job in an application](#connect-emr-serverless-task-state-startjobrun)
* [Cancel a job in an application](#connect-emr-serverless-task-state-canceljobrun)
To learn about configuring IAM permissions when using Step Functions with other AWS services, see [How Step Functions generates IAM policies for integrated
services](./service-integration-iam-templates.html).
In the examples shown in the following use cases, replace the `italicized` text with your resource-specific information. For example, replace `yourApplicationId` with the ID of your EMR Serverless application, such as `00yv7iv71inak893`.
### Create an application
The following Task state example creates an application using the *createApplication.sync* service integration API.
```
`"Create\_Application": {
"Type": "Task",
"Resource": "arn:aws:states:::emr-serverless:createApplication.sync",
"Arguments": {
"Name": "`MyApplication`",
"ReleaseLabel": "emr-6.9.0",
"Type": "SPARK"
},
"End": true
}`
```
### Start an application
The following Task state example starts an application using the *startApplication.sync* service integration API.
```
`"Start\_Application": {
"Type": "Task",
"Resource": "arn:aws:states:::emr-serverless:startApplication.sync",
"Arguments": {
"ApplicationId": "`yourApplicationId`"
},
"End": true
}`
```
### Stop an application
The following Task state example stops an application using the *stopApplication.sync* service integration API.
```
`"Stop\_Application": {
"Type": "Task",
"Resource": "arn:aws:states:::emr-serverless:stopApplication.sync",
"Arguments": {
"ApplicationId": "`yourApplicationId`"
},
"End": true
}`
```
### Delete an application
The following Task state example deletes an application using the *deleteApplication.sync* service integration API.
```
`"Delete\_Application": {
"Type": "Task",
"Resource": "arn:aws:states:::emr-serverless:deleteApplication.sync",
"Arguments": {
"ApplicationId": "`yourApplicationId`"
},
"End": true
}`
```
### Start a job in an application
The following Task state example starts a job in an application using the *startJobRun.sync* service integration API.
```
`"Start\_Job": {
"Type": "Task",
"Resource": "arn:aws:states:::emr-serverless:startJobRun.sync",
"Arguments": {
"ApplicationId": "`yourApplicationId`",
"ExecutionRoleArn": "arn:aws:iam::`account-id`:role/`myEMRServerless-execution-role`",
"JobDriver": {
"SparkSubmit": {
"EntryPoint": "s3://`&lt;amzn-s3-demo-bucket&gt;`/`sample.py`",
"EntryPointArguments": ["1"],
"SparkSubmitParameters": "--conf spark.executor.cores=4 --conf spark.executor.memory=4g --conf spark.driver.cores=2 --conf spark.driver.memory=4g --conf spark.executor.instances=1"
}
}
},
"End": true
}`
```
### Cancel a job in an application
The following Task state example cancels a job in an application using the *cancelJobRun.sync* service integration API.
```
`"Cancel\_Job": {
"Type": "Task",
"Resource": "arn:aws:states:::emr-serverless:cancelJobRun.sync",
"Arguments": {
"ApplicationId": "{% $states.input.ApplicationId %}",
"JobRunId": "{% $states.input.JobRunId %}"
},
"End": true
}`
```
## IAM policies for calling Amazon EMR Serverless
When you create a state machine using the console, Step Functions automatically creates an execution role for your state machine with the least privileges required. These automatically generated IAM roles are valid for the AWS Region in which you create the state machine.
The following example templates show how AWS Step Functions generates IAM policies based on the resources in your state machine definition. For more information, see [How Step Functions generates IAM policies for integrated
services](./service-integration-iam-templates.html) and [Discover service integration patterns in Step Functions](./connect-to-resource.html).
We recommend that when you create IAM policies, do not include wildcards in the policies. As a security best practice, you should scope your policies down as much as possible. You should use dynamic policies only when certain input parameters are not known during runtime.
Further, administrator users should be careful when granting non-administrator users execution roles for running the state machines. We recommend that you include passRole policies in the execution roles if you're creating policies on your own. We also recommend that you add the `aws:SourceARN` and `aws:SourceAccount` context keys in the execution roles.
### IAM policy examples for EMR Serverless integration with Step Functions
* [IAM policy example for CreateApplication](#emr-serverless-policy-createapp)
* [IAM policy example for StartApplication](#emr-serverless-policy-startapp)
* [IAM policy example for StopApplication](#emr-serverless-policy-stopapp)
* [IAM policy example for DeleteApplication](#emr-serverless-policy-deleteapp)
* [IAM policy example for StartJobRun](#emr-serverless-policy-startjobrun)
* [IAM policy example for CancelJobRun](#emr-serverless-policy-canceljobrun)
#### IAM policy example for CreateApplication
The following is an IAM policy example for a state machine with a CreateApplication [Task workflow state](./state-task.html) state.
###### Note
You need to specify the CreateServiceLinkedRole permissions in your IAM policies during the creation of the first ever application in your account. Thereafter, you need not add this permission. For information about CreateServiceLinkedRole, see [CreateServiceLinkedRole](https://docs.aws.amazon.com/IAM/latest/APIReference/API_CreateServiceLinkedRole.html) in the https://docs.aws.amazon.com/IAM/latest/APIReference/.
Static and dynamic resources for the following policies are the same.
Run a Job (.sync)
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Action": [
"emr-serverless:CreateApplication"
],
"Resource": [
"arn:aws:emr-serverless:`us-east-1`:`123456789012`:/\*"
]
},
{
"Effect": "Allow",
"Action": [
"emr-serverless:GetApplication",
"emr-serverless:DeleteApplication"
],
"Resource": [
"arn:aws:emr-serverless:`us-east-1`:`123456789012`:/applications/\*"
]
},
{
"Effect": "Allow",
"Action": [
"events:PutTargets",
"events:PutRule",
"events:DescribeRule"
],
"Resource": [
"arn:aws:events:`us-east-1`:`123456789012`:rule/StepFunctionsGetEventsForEMRServerlessApplicationRule"
]
},
{
"Effect": "Allow",
"Action": "iam:CreateServiceLinkedRole",
"Resource": "arn:aws:iam::`123456789012`:role/aws-service-role/ops.emr-serverless.amazonaws.com/`AWSServiceRoleForAmazonEMRServerless`\*",
"Condition": {
"StringLike": {
"iam:AWSServiceName": "ops.emr-serverless.amazonaws.com"
}
}
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
"emr-serverless:CreateApplication"
],
"Resource": [
"arn:aws:emr-serverless:`us-east-1`:`123456789012`:/\*"
]
},
{
"Effect": "Allow",
"Action": "iam:CreateServiceLinkedRole",
"Resource": "arn:aws:iam::`123456789012`:role/aws-service-role/ops.emr-serverless.amazonaws.com/`AWSServiceRoleForAmazonEMRServerless`\*",
"Condition": {
"StringLike": {
"iam:AWSServiceName": "ops.emr-serverless.amazonaws.com"
}
}
}
]
}`
`
```
###### Static resources
The following are IAM policy examples for static resources when you use a state machine with a StartApplication [Task workflow state](./state-task.html) state.
Run a Job (.sync)
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Action": [
"emr-serverless:StartApplication",
"emr-serverless:GetApplication",
"emr-serverless:StopApplication"
],
"Resource": [
"arn:aws:emr-serverless:`us-east-1`:`123456789012`:/applications/`applicationId`"
]
},
{
"Effect": "Allow",
"Action": [
"events:PutTargets",
"events:PutRule",
"events:DescribeRule"
],
"Resource": [
"arn:aws:events:`us-east-1`:`123456789012`:rule/`StepFunctionsGetEventsForEMRServerlessApplicationRule`"
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
"emr-serverless:StartApplication"
],
"Resource": [
"arn:aws:emr-serverless:`us-east-1`:`123456789012`:/applications/`applicationId`"
]
}
]
}`
`
```
###### Dynamic resources
The following are IAM policy examples for dynamic resources when you use a state machine with a StartApplication [Task workflow state](./state-task.html) state.
Run a Job (.sync)
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Action": [
"emr-serverless:StartApplication",
"emr-serverless:GetApplication",
"emr-serverless:StopApplication"
],
"Resource": [
"arn:aws:emr-serverless:`us-east-1`:`123456789012`:/applications/\*"
]
},
{
"Effect": "Allow",
"Action": [
"events:PutTargets",
"events:PutRule",
"events:DescribeRule"
],
"Resource": [
"arn:aws:events:`us-east-1`:`123456789012`:rule/`StepFunctionsGetEventsForEMRServerlessApplicationRule`"
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
"emr-serverless:StartApplication"
],
"Resource": [
"arn:aws:emr-serverless:`us-east-1`:`123456789012`:/applications/\*"
]
}
]
}`
`
```
###### Static resources
The following are IAM policy examples for static resources when you use a state machine with a StopApplication [Task workflow state](./state-task.html) state.
Run a Job (.sync)
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Action": [
"emr-serverless:StopApplication",
"emr-serverless:GetApplication"
],
"Resource": [
"arn:aws:emr-serverless:`us-east-1`:`123456789012`:/applications/`applicationId`"
]
},
{
"Effect": "Allow",
"Action": [
"events:PutTargets",
"events:PutRule",
"events:DescribeRule"
],
"Resource": [
"arn:aws:events:`us-east-1`:`123456789012`:rule/`StepFunctionsGetEventsForEMRServerlessApplicationRule`"
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
"emr-serverless:StopApplication"
],
"Resource": [
"arn:aws:emr-serverless:`us-east-1`:`123456789012`:/applications/`applicationId`"
]
}
]
}`
`
```
###### Dynamic resources
The following are IAM policy examples for dynamic resources when you use a state machine with a StopApplication [Task workflow state](./state-task.html) state.
Run a Job (.sync)
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Action": [
"emr-serverless:StopApplication",
"emr-serverless:GetApplication"
],
"Resource": [
"arn:aws:emr-serverless:`us-east-1`:`123456789012`:/applications/\*"
]
},
{
"Effect": "Allow",
"Action": [
"events:PutTargets",
"events:PutRule",
"events:DescribeRule"
],
"Resource": [
"arn:aws:events:`us-east-1`:`123456789012`:rule/`StepFunctionsGetEventsForEMRServerlessApplicationRule`"
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
"emr-serverless:StopApplication"
],
"Resource": [
"arn:aws:emr-serverless:`us-east-1`:`123456789012`:/applications/\*"
]
}
]
}`
`
```
###### Static resources
The following are IAM policy examples for static resources when you use a state machine with a DeleteApplication [Task workflow state](./state-task.html) state.
Run a Job (.sync)
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Action": [
"emr-serverless:DeleteApplication",
"emr-serverless:GetApplication"
],
"Resource": [
"arn:aws:emr-serverless:`us-east-1`:`123456789012`:/applications/`applicationId`"
]
},
{
"Effect": "Allow",
"Action": [
"events:PutTargets",
"events:PutRule",
"events:DescribeRule"
],
"Resource": [
"arn:aws:events:`us-east-1`:`123456789012`:rule/`StepFunctionsGetEventsForEMRServerlessApplicationRule`"
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
"emr-serverless:DeleteApplication"
],
"Resource": [
"arn:aws:emr-serverless:`us-east-1`:`123456789012`:/applications/`applicationId`"
]
}
]
}`
`
```
###### Dynamic resources
The following are IAM policy examples for dynamic resources when you use a state machine with a DeleteApplication [Task workflow state](./state-task.html) state.
Run a Job (.sync)
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Action": [
"emr-serverless:DeleteApplication",
"emr-serverless:GetApplication"
],
"Resource": [
"arn:aws:emr-serverless:`us-east-1`:`123456789012`:/applications/\*"
]
},
{
"Effect": "Allow",
"Action": [
"events:PutTargets",
"events:PutRule",
"events:DescribeRule"
],
"Resource": [
"arn:aws:events:`us-east-1`:`123456789012`:rule/`StepFunctionsGetEventsForEMRServerlessApplicationRule`"
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
"emr-serverless:DeleteApplication"
],
"Resource": [
"arn:aws:emr-serverless:`us-east-1`:`123456789012`:/applications/\*"
]
}
]
}`
`
```
###### Static resources
The following are IAM policy examples for static resources when you use a state machine with a StartJobRun [Task workflow state](./state-task.html) state.
Run a Job (.sync)
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Action": [
"emr-serverless:StartJobRun"
],
"Resource": [
"arn:aws:emr-serverless:`us-east-1`:`123456789012`:/applications/`applicationId`"
]
},
{
"Effect": "Allow",
"Action": "iam:PassRole",
"Resource": [
"arn:aws:iam::123456789012:role/`jobExecutionRoleArn`"
],
"Condition": {
"StringEquals": {
"iam:PassedToService": "emr-serverless.amazonaws.com"
}
}
},
{
"Effect": "Allow",
"Action": [
"emr-serverless:GetJobRun",
"emr-serverless:CancelJobRun"
],
"Resource": [
"arn:aws:emr-serverless:`us-east-1`:`123456789012`:/applications/`applicationId`/jobruns/\*"
]
},
{
"Effect": "Allow",
"Action": [
"events:PutTargets",
"events:PutRule",
"events:DescribeRule"
],
"Resource": [
"arn:aws:events:`us-east-1`:`123456789012`:rule/`StepFunctionsGetEventsForEMRServerlessJobRule`"
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
"emr-serverless:StartJobRun"
],
"Resource": [
"arn:aws:emr-serverless:`us-east-1`:`123456789012`:/applications/`applicationId`"
]
},
{
"Effect": "Allow",
"Action": "iam:PassRole",
"Resource": [
"arn:aws:iam::123456789012:role/`jobExecutionRoleArn`"
],
"Condition": {
"StringEquals": {
"iam:PassedToService": "emr-serverless.amazonaws.com"
}
}
}
]
}`
`
```
###### Dynamic resources
The following are IAM policy examples for dynamic resources when you use a state machine with a StartJobRun [Task workflow state](./state-task.html) state.
Run a Job (.sync)
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Action": [
"emr-serverless:StartJobRun",
"emr-serverless:GetJobRun",
"emr-serverless:CancelJobRun"
],
"Resource": [
"arn:aws:emr-serverless:`us-east-1`:`123456789012`:/applications/\*"
]
},
{
"Effect": "Allow",
"Action": "iam:PassRole",
"Resource": [
"arn:aws:iam::123456789012:role/`jobExecutionRoleArn`"
],
"Condition": {
"StringEquals": {
"iam:PassedToService": "emr-serverless.amazonaws.com"
}
}
},
{
"Effect": "Allow",
"Action": [
"events:PutTargets",
"events:PutRule",
"events:DescribeRule"
],
"Resource": [
"arn:aws:events:`us-east-1`:`123456789012`:rule/`StepFunctionsGetEventsForEMRServerlessJobRule`"
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
"emr-serverless:StartJobRun"
],
"Resource": [
"arn:aws:emr-serverless:`us-east-1`:`123456789012`:/applications/\*"
]
},
{
"Effect": "Allow",
"Action": "iam:PassRole",
"Resource": [
"arn:aws:iam::123456789012:role/`jobExecutionRoleArn`"
],
"Condition": {
"StringEquals": {
"iam:PassedToService": "emr-serverless.amazonaws.com"
}
}
}
]
}`
`
```
###### Static resources
The following are IAM policy examples for static resources when you use a state machine with a CancelJobRun [Task workflow state](./state-task.html) state.
Run a Job (.sync)
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Action": [
"emr-serverless:CancelJobRun",
"emr-serverless:GetJobRun"
],
"Resource": [
"arn:aws:emr-serverless:`us-east-1`:`123456789012`:/applications/`applicationId`/jobruns/`jobRunId`"
]
},
{
"Effect": "Allow",
"Action": [
"events:PutTargets",
"events:PutRule",
"events:DescribeRule"
],
"Resource": [
"arn:aws:events:`us-east-1`:`123456789012`:rule/`StepFunctionsGetEventsForEMRServerlessJobRule`"
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
"emr-serverless:CancelJobRun"
],
"Resource": [
"arn:aws:emr-serverless:`us-east-1`:`123456789012`:/applications/`applicationId`/jobruns/`jobRunId`"
]
}
]
}`
`
```
###### Dynamic resources
The following are IAM policy examples for dynamic resources when you use a state machine with a CancelJobRun [Task workflow state](./state-task.html) state.
Run a Job (.sync)
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Action": [
"emr-serverless:CancelJobRun",
"emr-serverless:GetJobRun"
],
"Resource": [
"arn:aws:emr-serverless:`us-east-1`:`123456789012`:/applications/\*"
]
},
{
"Effect": "Allow",
"Action": [
"events:PutTargets",
"events:PutRule",
"events:DescribeRule"
],
"Resource": [
"arn:aws:events:`us-east-1`:`123456789012`:rule/`StepFunctionsGetEventsForEMRServerlessJobRule`"
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
"emr-serverless:CancelJobRun"
],
"Resource": [
"arn:aws:emr-serverless:`us-east-1`:`123456789012`:/applications/\*"
]
}
]
}`
`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Amazon EMR on EKS
Amazon EventBridge
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.