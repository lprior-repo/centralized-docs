---
url: https://docs.aws.amazon.com/step-functions/latest/dg/tutorial-gather-s3-info.html
title: Gather Amazon S3 bucket info using AWS SDK service integrations
word_count: 1210
filtered: true
elements_removed: 0
density_score: 0.83
---

Gather Amazon S3 bucket info using AWS SDK service integrations - AWS Step Functions
Gather Amazon S3 bucket info using AWS SDK service integrations - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#tutorial-gather-s3-info)
[Step 1: Create the state machine](#aws-sdk-create-state-machine)[Step 2: Add the necessary IAM role permissions](#aws-sdk-add-iam-permissions)[Step 3: Run a Standard state machine execution](#aws-sdk-run-standard)[Step 4: Run an Express state machine execution](#aws-sdk-run-express)
# Gather Amazon S3 bucket info using AWS SDK service integrations
This tutorial shows you how to perform an [AWS SDK integration](./supported-services-awssdk.html) with Amazon Simple Storage Service. The state machine you create in this tutorial gathers
information about your Amazon S3 buckets, then list your buckets along with version information for
each bucket in the current region.
## Step 1: Create the state machine
Using the Step Functions console, you'll create a state machine that includes a `Task` state to list all the Amazon S3 buckets in the current account and region. Then, you'll add another `Task` state that invokes the `[HeadBucket](https://docs.aws.amazon.com/AmazonS3/latest/API/API_HeadBucket.html)` API to verify if the returned bucket is accessible in the current region. If the bucket isn't accessible, the `HeadBucket` API call returns the `S3.S3Exception` error. You'll include a `Catch` block to catch this exception and a `Pass` state as the fallback state.
1. Open the [Step Functions console](https://console.aws.amazon.com/states/home), choose **State machines** from the menu, then choose **Create state machine**.
2. Choose **Create from blank**.
3. Name your state machine, then choose **Continue** to edit your state machine in Workflow Studio.
4. For this tutorial, you'll write the [Amazon States Language](./concepts-amazon-states-language.html) (ASL) definition of your state machine in the [Code editor](./workflow-studio.html#wfs-interface-code-editor). To do this, choose **Code**.
5. Remove the existing boilerplate code and paste the following state machine definition.
```
`{
"Comment": "A description of my state machine",
"StartAt": "ListBuckets",
"States": {
"ListBuckets": {
"Type": "Task",
"Parameters": {},
"Resource": "arn:aws:states:::aws-sdk:s3:listBuckets",
"Next": "Map"
},
"Map": {
"Type": "Map",
"ItemsPath": "$.Buckets",
"ItemProcessor": {
"ProcessorConfig": {
"Mode": "INLINE"
},
"StartAt": "HeadBucket",
"States": {
"HeadBucket": {
"Type": "Task",
"ResultPath": null,
"Parameters": {
"Bucket.$": "$.Name"
},
"Resource": "arn:aws:states:::aws-sdk:s3:headBucket",
"Catch": [
{
"ErrorEquals": [
"S3.S3Exception"
],
"ResultPath": null,
"Next": "Pass"
}
],
"Next": "GetBucketVersioning"
},
"GetBucketVersioning": {
"Type": "Task",
"End": true,
"Parameters": {
"Bucket.$": "$.Name"
},
"ResultPath": "$.BucketVersioningInfo",
"Resource": "arn:aws:states:::aws-sdk:s3:getBucketVersioning"
},
"Pass": {
"Type": "Pass",
"End": true,
"Result": {
"Status": "Unknown"
},
"ResultPath": "$.BucketVersioningInfo"
}
}
},
"End": true
}
}
}`
```
6. Specify a name for your state machine. To do this, choose the edit icon next to the default state machine name of **MyStateMachine**. Then, in **State machine configuration**, specify a name in the **State machine name** box.
For this tutorial, enter the name `Gather-S3-Bucket-Info-Standard`.
7. (Optional) In **State machine configuration**, specify other workflow settings, such as state machine type and its execution role.
Keep all the default selections in **State machine settings**.
If you've [previously created an IAM role](./procedure-create-iam-role.html) with the correct permissions for your state machine and want to use it, in **Permissions**, select **Choose an existing role**, and then select a role from the list. Or select **Enter a role ARN** and then provide an ARN for that IAM role.
8. In the **Confirm role creation** dialog box, choose **Confirm** to continue.
You can also choose **View role settings** to go back to **State machine configuration**.
###### Note
If you delete the IAM role that Step Functions creates, Step Functions can't
recreate it later. Similarly, if you modify the role (for example, by
removing Step Functions from the principals in the IAM policy), Step Functions can't
restore its original settings later.
In [Step 2](#aws-sdk-add-iam-permissions), you'll add the missing permissions to the state machine role.
## Step 2: Add the necessary IAM role permissions
To gather information about the Amazon S3 buckets in your current region, you must provide your state machine the necessary permissions to access the Amazon S3 buckets.
1. On the state machine page, choose **IAM role ARN** to open the **Roles** page for the state machine role.
2. Choose **Add permissions** and then choose **Create inline policy.**
3. Choose the **JSON** tab, and then paste the following permissions into the JSON editor.
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Sid": "VisualEditor0",
"Effect": "Allow",
"Action": [
"s3:ListAllMyBuckets",
"s3:ListBucket",
"s3:GetBucketVersioning"
],
"Resource": "\*"
}
]
}`
`
```
4. Choose **Review policy**.
5. Under **Review policy**, for the policy **Name**, enter `s3-bucket-permissions`.
6. Choose **Create policy**.
## Step 3: Run a Standard state machine execution
1. On the **Gather-S3-Bucket-Info-Standard** page, choose **Start execution**.
2. In the **Start execution** dialog box, do the following:
1. (Optional) Enter a custom execution name to override the generated default.
###### Non-ASCII names and logging
Step Functions accepts names for state machines, executions, activities, and labels that contain non-ASCII characters. Because such characters will prevent Amazon CloudWatch from logging data, we recommend using only ASCII characters so you can track Step Functions metrics.
2. Choose **Start execution**.
3. The Step Functions console directs you to a page that's titled with your execution ID. This page is known as the *Execution Details* page. On this page, you can review the execution results as the execution progresses or after it's complete.
To review the execution results, choose individual states on the **Graph view**, and then choose the individual tabs on the [Step details](./concepts-view-execution-details.html#exec-details-intf-step-details) pane to view each state's details including input, output, and definition respectively. For details about the execution information you can view on the *Execution Details* page, see [Execution details overview](./concepts-view-execution-details.html#exec-details-interface-overview).
## Step 4: Run an Express state machine execution
1. Create an Express state machine using the state machine definition provided in [Step 1](#aws-sdk-create-state-machine). Make sure that you also include the necessary IAM role permissions as explained in [Step 2](#aws-sdk-add-iam-permissions).
###### Tip
To distinguish from the Standard machine you created earlier, name the Express state machine as `Gather-S3-Bucket-Info-Express`.
2. On the **Gather-S3-Bucket-Info-Standard** page, choose **Start execution**.
3. In the **Start execution** dialog box, do the following:
1. (Optional) Enter a custom execution name to override the generated default.
###### Non-ASCII names and logging
Step Functions accepts names for state machines, executions, activities, and labels that contain non-ASCII characters. Because such characters will prevent Amazon CloudWatch from logging data, we recommend using only ASCII characters so you can track Step Functions metrics.
2. Choose **Start execution**.
3. The Step Functions console directs you to a page that's titled with your execution ID. This page is known as the *Execution Details* page. On this page, you can review the execution results as the execution progresses or after it's complete.
To review the execution results, choose individual states on the **Graph view**, and then choose the individual tabs on the [Step details](./concepts-view-execution-details.html#exec-details-intf-step-details) pane to view each state's details including input, output, and definition respectively. For details about the execution information you can view on the *Execution Details* page, see [Execution details overview](./concepts-view-execution-details.html#exec-details-interface-overview).
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
View X-Ray traces
Continue long-running workflows using Step Functions API (recommended)
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.