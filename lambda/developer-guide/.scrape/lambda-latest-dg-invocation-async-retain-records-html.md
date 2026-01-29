---
url: https://docs.aws.amazon.com/lambda/latest/dg/invocation-async-retain-records.html
title: Capturing records of Lambda asynchronous invocations
word_count: 2228
filtered: true
elements_removed: 0
density_score: 0.86
---

Capturing records of Lambda asynchronous invocations - AWS Lambda
Capturing records of Lambda asynchronous invocations - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#invocation-async-retain-records)
[Destinations](#invocation-async-destinations)[Dead-letter queues](#invocation-dlq)
# Capturing records of Lambda asynchronous invocations
Lambda can send records of asynchronous invocations to one of the following AWS services.
* **Amazon SQS** – A standard SQS queue
* **Amazon SNS** – A standard SNS topic
* **Amazon S3** – An Amazon S3 bucket (on failure only)
* **AWS Lambda** – A Lambda function
* **Amazon EventBridge** – An EventBridge event bus
The invocation record contains details about the request and response in JSON format. You can configure
separate destinations for events that are processed successfully, and events that fail all processing attempts.
Alternatively, you can configure a standard Amazon SQS queue or standard Amazon SNS topic as a dead-letter
queue for discarded events. For dead-letter queues, Lambda only sends the content of the event, without
details about the response.
If Lambda can't send a record to a destination you have configured, it sends a `DestinationDeliveryFailures` metric to
Amazon CloudWatch. This can happen if your configuration includes an unsupported destination type, such as an Amazon SQS FIFO queue or an Amazon SNS FIFO
topic. Delivery errors can also occur due to permissions errors and size limits. For more information on Lambda invocation metrics,
see [Invocation metrics](./monitoring-metrics-types.html#invocation-metrics).
###### Note
To prevent a function from triggering, you can set the function's reserved concurrency to zero. When you set
reserved concurrency to zero for an asynchronously invoked function, Lambda begins sending new events to the
configured [dead-letter queue](#invocation-dlq) or the on-failure [event destination](#invocation-async-destinations), without any retries. To process events that
were sent while reserved concurrency was set to zero, you must consume the events from the dead-letter queue or
the on-failure event destination.
## Adding a destination
To retain records of asynchronous invocations, add a destination to your function. You can choose to send
either successful or failed invocations to a destination. Each function can have multiple destinations, so
you can configure separate destinations for successful and failed events. Each record sent to the
destination is a JSON document with details about the invocation. Like error handling settings, you can
configure destinations on a function, function version, or alias.
###### Tip
You can also retain records of failed invocations for the following event source mapping types:
[Amazon Kinesis](./kinesis-on-failure-destination.html#kinesis-on-failure-destination-console),
[Amazon DynamoDB](./services-dynamodb-errors.html), and
[Apache Kafka (Amazon MSK and self-managed Apache Kafka)](./kafka-on-failure.html#kafka-onfailure-destination).
The following table lists supported destinations for asynchronous invocation records. For Lambda to
successfully send records to your chosen destination, ensure that your function's
[execution role](./lambda-intro-execution-role.html) also contains the relevant permissions.
The table also describes how each destination type receives the JSON invocation record.
|Destination type|Required permission|Destination-specific JSON format|
|
Amazon SQS queue
|
[sqs:SendMessage](https://docs.aws.amazon.com/AWSSimpleQueueService/latest/APIReference/API_SendMessage.html)
|
Lambda passes the invocation record as the `Message` to the destination.
|
|
Amazon SNS topic
|
[sns:Publish](https://docs.aws.amazon.com/sns/latest/api/API_Publish.html)
|
Lambda passes the invocation record as the `Message` to the destination.
|
|
Amazon S3 bucket (on failure only)
|
[s3:PutObject](https://docs.aws.amazon.com/AmazonS3/latest/API/API_PutObject.html)
[s3:ListBucket](https://docs.aws.amazon.com/AmazonS3/latest/API/API_ListObjectsV2.html)
|
* Lambda stores the invocation record as a JSON object in the destination bucket.
* The S3 object name uses the following naming convention:
```
`aws/lambda/async/&lt;function-name&gt;/YYYY/MM/DD/YYYY-MM-DDTHH.MM.SS-&lt;Random UUID&gt;`
```
|
|
Lambda function
|
[lambda:InvokeFunction](https://docs.aws.amazon.com/lambda/latest/api/API_Invoke.html)
|
Lambda passes the invocation record as the payload to the function.
|
|
EventBridge
|
[events:PutEvents](https://docs.aws.amazon.com/eventbridge/latest/APIReference/API_PutEvents.html)
|
* Lambda passes the invocation record as the `detail` in the PutEvents call.
* The value for the `source` event field is `aws.lambda`.
* The value for the `detail-type` event field is either "Lambda Function
Invocation Result - Success" or "Lambda Function Invocation Result - Failure".
* The `resource` event field contains the function and destination
Amazon Resource Names (ARNs).
* For other event fields, see [Amazon EventBridge events](https://docs.aws.amazon.com/eventbridge/latest/userguide/eb-events.html).
|
###### Note
For Amazon S3 destinations, if you have enabled encryption on the bucket using a KMS key, your function also needs the
[kms:GenerateDataKey](https://docs.aws.amazon.com/kms/latest/APIReference/API_GenerateDataKey.html) permission.
###### Important
When using Amazon SNS as a destination, be aware that Amazon SNS has a maximum message size limit of 256 KB. If your async invocation payload approaches 1 MB, the invocation record (which includes the original payload plus additional metadata) may exceed the Amazon SNS limit and cause delivery failures. Consider using Amazon SQS or Amazon S3 destinations for larger payloads.
The following steps describe how to configure a destination for a function using the Lambda console and the AWS CLI.
Console
1. Open the [Functions page](https://console.aws.amazon.com/lambda/home#/functions) of the Lambda console.
2. Choose a function.
3. Under **Function overview**, choose **Add destination**.
4. For **Source**, choose **Asynchronous invocation**.
5. For **Condition**, choose from the following options:
* **On failure** – Send a record when the event fails all processing attempts or
exceeds the maximum age.
* **On success** – Send a record when the function successfully processes an
asynchronous invocation.
* For **Destination type**, choose the type of resource that receives the invocation
record.
* For **Destination**, choose a resource.
* Choose **Save**.
AWS CLI
To configure a destination using the AWS CLI, run the [update-function-event-invoke-config](https://awscli.amazonaws.com/v2/documentation/api/latest/reference/lambda/update-function-event-invoke-config.html) command. The following example configures Lambda to send a record to a standard
SQS queue named `destination` when an event can't be processed.
```
`aws lambda update-function-event-invoke-config \\
--function-name my-function \\
--destination-config '{"OnFailure":{"Destination": "`arn:aws:sqs:us-east-1:123456789012:``destination`"}}'`
```
### Security best practices for Amazon S3 destinations
Deleting an S3 bucket that's configured as a destination without removing the destination from your function's configuration can create a security risk. If another
user knows your destination bucket's name, they can recreate the bucket in their AWS account. Records of failed invocations will be sent to their bucket, potentially
exposing data from your function.
###### Warning
To ensure that invocation records from your function can't be sent to an S3 bucket in another AWS account, add a condition to your function's execution role
that limits `s3:PutObject` permissions to buckets in your account.
The following example shows an IAM policy that limits your function's `s3:PutObject` permissions to buckets in your account. This policy also gives Lambda
the `s3:ListBucket` permission it needs to use an S3 bucket as a destination.
```
`{
"Version": "2012-10-17",
"Statement": [
{
"Sid": "S3BucketResourceAccountWrite",
"Effect": "Allow",
"Action": [
"s3:PutObject",
"s3:ListBucket"
],
"Resource": [
"arn:aws:s3:::\*/\*",
"arn:aws:s3:::\*"
],
"Condition": {
"StringEquals": {
"s3:ResourceAccount": `"111122223333"`
}
}
}
]
}`
```
To add a permissions policy to your function's execution role using the AWS Management Console or AWS CLI, refer to the instructions in the following procedures:
Console
###### To add a permissions policy to a function's execution role (console)
1. Open the [Functions page](https://console.aws.amazon.com/lambda/home#/functions) of the Lambda console.
2. Select the Lambda function whose execution role you want to modify.
3. In the **Configuration** tab, select **Permissions**.
4. In the **Execution role** tab, select your function's **Role name** to open the role's IAM console page.
5. Add a permissions policy to the role by doing the following:
1. In the **Permissions policies** pane, choose **Add permissions** and select **Create inline policy**.
2. In **Policy editor**, select **JSON**.
3. Paste the policy you want to add into the editor (replacing the existing JSON), and then choose **Next**.
4. Under **Policy details**, enter a **Policy name**.
5. Choose **Create policy**.
AWS CLI
###### To add a permissions policy to a function's execution role (CLI)
1. Create a JSON policy document with the required permissions and save it in a local directory.
2. Use the IAM `put-role-policy` CLI command to add the permissions to your function's execution role. Run the following command from the
directory you saved your JSON policy document in and replace the role name, policy name, and policy document with your own values.
```
``aws iam put-role-policy \\
--role-name `my\_lambda\_role` \\
--policy-name LambdaS3DestinationPolicy \\
--policy-document file://`my\_policy.json```
```
### Example invocation record
When an invocation matches the condition, Lambda sends [a JSON document](#destinations-permissions) with details about the invocation to the destination. The following example shows an invocation record for an event that failed three processing attempts due to a function error.
```
`{
"version": "1.0",
"timestamp": "2019-11-14T18:16:05.568Z",
"requestContext": {
"requestId": "e4b46cbf-b738-xmpl-8880-a18cdf61200e",
"functionArn": "arn:aws:lambda:us-east-1:123456789012:function:my-function:$LATEST",
"condition": "RetriesExhausted",
"approximateInvokeCount": 3
},
"requestPayload": {
"ORDER\_IDS": [
"9e07af03-ce31-4ff3-xmpl-36dce652cb4f",
"637de236-e7b2-464e-xmpl-baf57f86bb53",
"a81ddca6-2c35-45c7-xmpl-c3a03a31ed15"
]
},
"responseContext": {
"statusCode": 200,
"executedVersion": "$LATEST",
"functionError": "Unhandled"
},
"responsePayload": {
"errorMessage": "RequestId: e4b46cbf-b738-xmpl-8880-a18cdf61200e Process exited before completing request"
}
}`
```
The invocation record contains details about the event, the response, and the reason that the record was
sent.
### Tracing requests to destinations
You can use AWS X-Ray
to see a connected view of each request as it's queued, processed by a Lambda function, and passed to the
destination service. When you activate X-Ray tracing for a function or a service that invokes a function, Lambda
adds an X-Ray header to the request and passes the header to the destination service. Traces from upstream
services are automatically linked to traces from downstream Lambda functions and destination services, creating
an end-to-end view of the entire application. For more information about tracing, see [Visualize Lambda function invocations using AWS X-Ray](./services-xray.html).
## Adding a dead-letter queue
As an alternative to an [on-failure destination](#invocation-async-destinations), you can
configure your function with a dead-letter queue to save discarded events for further processing. A dead-letter
queue acts the same as an on-failure destination in that it is used when an event fails all processing attempts or
expires without being processed. However, you can only add or remove a dead-letter queue at the function level. Function versions use the same dead-letter queue settings as the unpublished version ($LATEST). On-failure destinations also support additional
targets and include details about the function's response in the invocation record.
To reprocess events in a dead-letter queue, you can set it as an [event source](./invocation-eventsourcemapping.html) for your Lambda function.
Alternatively, you can manually retrieve the events.
You can choose an Amazon SQS standard queue or Amazon SNS standard topic for your dead-letter queue. FIFO queues and
Amazon SNS FIFO topics are not supported.
* [Amazon SQS queue](https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-create-queue.html) – A queue holds failed events
until they're retrieved. Choose an Amazon SQS standard queue if you expect a single entity, such as a Lambda function or CloudWatch
alarm, to process the failed event. For more information, see [Using Lambda with Amazon SQS](./with-sqs.html).
* [Amazon SNS topic](https://docs.aws.amazon.com/sns/latest/gsg/CreateTopic.html) – A topic relays failed events to
one or more destinations. Choose an Amazon SNS standard topic if you expect multiple entities to act on a failed event. For
example, you can configure a topic to send events to an email address, a Lambda function, and/or an HTTP
endpoint. For more information, see [Invoking Lambda functions with Amazon SNS notifications](./with-sns.html).
To send events to a queue or topic, your function needs additional permissions. Add a policy with the
[ required permissions](#destinations-permissions) to your function's
[execution role](./lambda-intro-execution-role.html). If the target queue or topic
is encrypted with a customer managed AWS KMS key, ensure that both your function's execution role and the key's
[resource-based policy](https://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html) contains the relevant permissions.
After creating the target and updating your function's execution role, add the dead-letter queue to your
function. You can configure multiple functions to send events to the same target.
Console
1. Open the [Functions page](https://console.aws.amazon.com/lambda/home#/functions) of the Lambda console.
2. Choose a function.
3. Choose **Configuration** and then choose **Asynchronous
invocation**.
4. Under **Asynchronous invocation**, choose **Edit**.
5. Set **Dead-letter queue service** to **Amazon SQS** or **Amazon SNS**.
6. Choose the target queue or topic.
7. Choose **Save**.
AWS CLI
To configure a dead-letter queue with the AWS CLI, use the [update-function-configuration](https://awscli.amazonaws.com/v2/documentation/api/latest/reference/lambda/update-function-configuration.html) command.
```
`aws lambda update-function-configuration \\
--function-name my-function \\
--dead-letter-config TargetArn=`arn:aws:sns:us-east-1:123456789012:my-topic``
```
Lambda sends the event to the dead-letter queue as-is, with additional information in attributes. You can use
this information to identify the error that the function returned, or to correlate the event with logs or an
AWS X-Ray trace.
###### Dead-letter queue message attributes
* **RequestID** (String) – The ID of the invocation request. Request IDs
appear in function logs. You can also use the X-Ray SDK to record the request ID on an attribute in the
trace. You can then search for traces by request ID in the X-Ray console.
* **ErrorCode** (Number) – The HTTP status code.
* **ErrorMessage** (String) – The first 1 KB of the error
message.
If Lambda can't send a message to the dead-letter queue, it deletes the event and emits the [DeadLetterErrors](./monitoring-metrics-types.html) metric. This can happen because of lack of permissions, or
if the total size of the message exceeds the limit for the target queue or topic. For example, say that an Amazon SNS
notification with a body close to 1 MB in size triggers a function that results in an error. In
that case, the event data that Amazon SNS adds, combined with the attributes that Lambda adds, can cause the message to
exceed the maximum size allowed in the dead-letter queue.
If you're using Amazon SQS as an event source, configure a dead-letter queue on the Amazon SQS queue itself and not on
the Lambda function. For more information, see [Using Lambda with Amazon SQS](./with-sqs.html).
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Configuration
Invocation
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.