---
url: https://docs.aws.amazon.com/lambda/latest/dg/with-sqs-cross-account-example.html
title: Tutorial: Using a cross-account Amazon SQS queue as an event
word_count: 1346
filtered: true
elements_removed: 0
density_score: 0.87
---

Tutorial: Using a cross-account Amazon SQS queue as an event source - AWS Lambda
Tutorial: Using a cross-account Amazon SQS queue as an event source - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#with-sqs-cross-account-example)
[Prerequisites](#with-sqs-cross-account-prepare)[Create the execution role (Account A)](#with-sqs-cross-account-create-execution-role)[Create the function (Account A)](#with-sqs-cross-account-create-function)[Test the function (Account A)](#with-sqs-cross-account-create-test-function)[Create an Amazon SQS queue (Account B)](#with-sqs-cross-account-configure-sqs)[Configure the event source (Account A)](#with-sqs-cross-account-event-source)[Test the setup](#with-sqs-final-integration-test-no-iam)[Clean up your resources](#cleanup)
# Tutorial: Using a cross-account Amazon SQS queue as an event
source
In this tutorial, you create a Lambda function that consumes messages from an Amazon Simple Queue Service (Amazon SQS) queue in a
different AWS account. This tutorial involves two AWS accounts: **Account A** refers to the
account that contains your Lambda function, and **Account B** refers to the account that contains
the Amazon SQS queue.
## Prerequisites
If you have not yet installed the AWS Command Line Interface, follow the steps at [Installing or updating the latest version of the AWS CLI](https://docs.aws.amazon.com/cli/latest/userguide/getting-started-install.html)
to install it.
The tutorial requires a command line terminal or shell to run commands. In Linux and macOS, use your preferred shell and package manager.
###### Note
In Windows, some Bash CLI commands that you commonly use with Lambda (such as `zip`) are not supported by the operating system's built-in terminals.
To get a Windows-integrated version of Ubuntu and Bash, [install the Windows Subsystem for Linux](https://docs.microsoft.com/en-us/windows/wsl/install-win10).
## Create the execution role (Account A)
In **Account A**, create an [execution role](./lambda-intro-execution-role.html)
that gives your function permission to access the required AWS resources.
###### To create an execution role
1. Open the [Roles page](https://console.aws.amazon.com/iam/home#/roles) in the AWS Identity and Access Management (IAM)
console.
2. Choose **Create role**.
3. Create a role with the following properties.
* **Trusted entity** –
**AWS Lambda**
* **Permissions** –
**AWSLambdaSQSQueueExecutionRole**
* **Role name** –
`cross-account-lambda-sqs-role`
The **AWSLambdaSQSQueueExecutionRole** policy has the permissions that the function needs to
read items from Amazon SQS and to write logs to Amazon CloudWatch Logs.
## Create the function (Account A)
In **Account A**, create a Lambda function that processes your Amazon SQS messages. The Lambda function and the Amazon SQS queue must be in the same AWS Region.
The following Node.js code example writes each message to a log in CloudWatch Logs.
###### Example index.mjs
```
`export const handler = async function(event, context) {
event.Records.forEach(record =&gt; {
const { body } = record;
console.log(body);
});
return {};
}`
```
###### Note
Following these steps creates a Node.js function. For other languages, the steps are similar, but
some details are different.
1. Save the code example as a file named `index.mjs`.
2. Create a deployment package.
```
``zip function.zip index.mjs``
```
3. Create the function using the `create-function` AWS Command Line Interface (AWS CLI) command. Replace `arn:aws:iam::111122223333:role/cross-account-lambda-sqs-role` with the ARN of the execution role that you created earlier.
```
``aws lambda create-function --function-name CrossAccountSQSExample \\
--zip-file fileb://function.zip --handler index.handler --runtime nodejs24.x \\
--role `arn:aws:iam::111122223333:role/cross-account-lambda-sqs-role```
```
## Test the function (Account A)
In **Account A**, test your Lambda function manually using the `invoke` AWS CLI
command and a sample Amazon SQS event.
If the handler returns normally without exceptions, Lambda considers the message to be successfully processed
and begins reading new messages in the queue. After successfully processing a message, Lambda automatically deletes
it from the queue. If the handler throws an exception, Lambda considers the batch of messages not successfully
processed, and Lambda invokes the function with the same batch of messages.
1. Save the following JSON as a file named `input.txt`.
```
`{
"Records": [
{
"messageId": "059f36b4-87a3-44ab-83d2-661975830a7d",
"receiptHandle": "AQEBwJnKyrHigUMZj6rYigCgxlaS3SLy0a...",
"body": "test",
"attributes": {
"ApproximateReceiveCount": "1",
"SentTimestamp": "1545082649183",
"SenderId": "AIDAIENQZJOLO23YVJ4VO",
"ApproximateFirstReceiveTimestamp": "1545082649185"
},
"messageAttributes": {},
"md5OfBody": "098f6bcd4621d373cade4e832627b4f6",
"eventSource": "aws:sqs",
"eventSourceARN": "arn:aws:sqs:us-east-1:111122223333:example-queue",
"awsRegion": "us-east-1"
}
]
}`
```
The preceding JSON simulates an event that Amazon SQS might send to your Lambda function, where
`"body"` contains the actual message from the queue.
2. Run the following `invoke` AWS CLI command.
```
``aws lambda invoke --function-name CrossAccountSQSExample \\
--cli-binary-format raw-in-base64-out \\
--payload file://input.txt outputfile.txt``
```
The **cli-binary-format** option is required if you're using AWS CLI version 2. To make this the default setting, run `aws configure set cli-binary-format raw-in-base64-out`. For more information, see [AWS CLI supported global command line options](https://docs.aws.amazon.com/cli/latest/userguide/cli-configure-options.html#cli-configure-options-list) in the *AWS Command Line Interface User Guide for Version 2*.
3. Verify the output in the file `outputfile.txt`.
## Create an Amazon SQS queue (Account B)
In **Account B**, create an Amazon SQS queue that the Lambda function in **Account
A** can use as an event source. The Lambda function and the Amazon SQS queue must be in the same AWS Region.
###### To create a queue
1. Open the [Amazon SQS console](https://console.aws.amazon.com/sqs).
2. Choose **Create queue**.
3. Create a queue with the following properties.
* **Type** – **Standard**
* **Name** – **LambdaCrossAccountQueue**
* **Configuration** – Keep the default settings.
* **Access policy** – Choose **Advanced**. Paste in the
following JSON policy. Replace the following values:
* `111122223333`: AWS account ID for **Account A**
* `444455556666`: AWS account ID for **Account B**
JSON
****
```
``{
"Version":"2012-10-17",
"Id": "Queue1\_Policy\_UUID",
"Statement": [
{
"Sid": "Queue1\_AllActions",
"Effect": "Allow",
"Principal": {
"AWS": [
"arn:aws:iam::`111122223333`:role/cross-account-lambda-sqs-role"
]
},
"Action": "sqs:\*",
"Resource": "arn:aws:sqs:us-east-1:`444455556666`:LambdaCrossAccountQueue"
}
]
}`
`
```
This policy grants the Lambda execution role in **Account A** permissions to consume
messages from this Amazon SQS queue.
* After creating the queue, record its Amazon Resource Name (ARN). You need this in the next step when you
associate the queue with your Lambda function.
## Configure the event source (Account A)
In **Account A**, create an event source mapping between the Amazon SQS queue in **Account
B** and your Lambda function by running the following `create-event-source-mapping` AWS CLI
command. Replace `arn:aws:sqs:us-east-1:444455556666:LambdaCrossAccountQueue` with the ARN of the Amazon SQS queue that you created in the previous step.
```
``aws lambda create-event-source-mapping --function-name CrossAccountSQSExample --batch-size 10 \\
--event-source-arn arn:aws:sqs:us-east-1:`444455556666`:LambdaCrossAccountQueue``
```
To get a list of your event source mappings, run the following command.
```
``aws lambda list-event-source-mappings --function-name CrossAccountSQSExample \\
--event-source-arn arn:aws:sqs:us-east-1:`444455556666`:LambdaCrossAccountQueue``
```
## Test the setup
You can now test the setup as follows:
1. In **Account B**, open the [Amazon SQS console](https://console.aws.amazon.com/sqs).
2. Choose **LambdaCrossAccountQueue**, which you created earlier.
3. Choose **Send and receive messages**.
4. Under **Message body**, enter a test message.
5. Choose **Send message**.
Your Lambda function in **Account A** should receive the message. Lambda will continue to poll
the queue for updates. When there is a new message, Lambda invokes your function with this new event data from the
queue. Your function runs and creates logs in Amazon CloudWatch. You can view the logs in the [CloudWatch console](https://console.aws.amazon.com/cloudwatch).
## Clean up your resources
You can now delete the resources that you created for this tutorial, unless you want to retain them. By deleting AWS resources that you're no longer using, you prevent unnecessary charges to your AWS account.
In **Account A**, clean up your execution role and Lambda function.
###### To delete the execution role
1. Open the [Roles page](https://console.aws.amazon.com/iam/home#/roles) of the IAM console.
2. Select the execution role that you created.
3. Choose **Delete**.
4. Enter the name of the role in the text input field and choose **Delete**.
###### To delete the Lambda function
1. Open the [Functions page](https://console.aws.amazon.com/lambda/home#/functions) of the Lambda console.
2. Select the function that you created.
3. Choose **Actions**, **Delete**.
4. Type `confirm` in the text input field and choose **Delete**.
In **Account B**, clean up the Amazon SQS queue.
###### To delete the Amazon SQS queue
1. Sign in to the AWS Management Console and open the Amazon SQS console at
[https://console.aws.amazon.com/sqs/](https://console.aws.amazon.com/sqs/).
2. Select the queue you created.
3. Choose **Delete**.
4. Enter `confirm` in the text input field.
5. Choose **Delete**.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Tutorial
Step Functions
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.