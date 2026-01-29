---
url: https://docs.aws.amazon.com/lambda/latest/dg/services-s3-batch.html
title: Invoke a Lambda function with Amazon S3 batch events
word_count: 666
filtered: true
elements_removed: 0
density_score: 0.84
---

Invoke a Lambda function with Amazon S3 batch events - AWS Lambda
Invoke a Lambda function with Amazon S3 batch events - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#services-s3-batch)
[Invoking Lambda functions from Amazon S3 batch operations ](#invoking)
# Invoke a Lambda function with Amazon S3 batch events
You can use Amazon S3 batch operations to invoke a Lambda function on a large set of Amazon S3 objects. Amazon S3
tracks the progress of batch operations, sends notifications, and stores a completion report that shows the status
of each action.
To run a batch operation, you create an Amazon S3 [batch operations job](https://docs.aws.amazon.com/AmazonS3/latest/dev/batch-ops-operations.html). When you create
the job, you provide a manifest (the list of objects) and configure the action to perform on those objects.
When the batch job starts, Amazon S3 invokes the Lambda function [synchronously](./invocation-sync.html) for each object in the manifest.
The event parameter includes the names of the bucket and the object.
The following example shows the event that Amazon S3 sends to the Lambda function for an object that is named
**customerImage1.jpg** in the **amzn-s3-demo-bucket**
bucket.
###### Example Amazon S3 batch request event
```
`
{
"invocationSchemaVersion": "1.0",
"invocationId": "YXNkbGZqYWRmaiBhc2RmdW9hZHNmZGpmaGFzbGtkaGZza2RmaAo",
"job": {
"id": "f3cc4f60-61f6-4a2b-8a21-d07600c373ce"
},
"tasks": [
{
"taskId": "dGFza2lkZ29lc2hlcmUK",
"s3Key": "customerImage1.jpg",
"s3VersionId": "1",
"s3BucketArn": "arn:aws:s3:::amzn-s3-demo-bucket"
}
]
}`
```
Your Lambda function must return a JSON object with the fields as shown in the following example. You can copy the
`invocationId` and `taskId` from the event parameter. You can return a string in the `resultString`.
Amazon S3 saves the `resultString` values in the completion report.
###### Example Amazon S3 batch request response
```
`
{
"invocationSchemaVersion": "1.0",
"treatMissingKeysAs" : "PermanentFailure",
"invocationId" : "YXNkbGZqYWRmaiBhc2RmdW9hZHNmZGpmaGFzbGtkaGZza2RmaAo",
"results": [
{
"taskId": "dGFza2lkZ29lc2hlcmUK",
"resultCode": "Succeeded",
"resultString": "[\\"Alice\\", \\"Bob\\"]"
}
]
}
`
```
## Invoking Lambda functions from Amazon S3 batch operations
You can invoke the Lambda function with an unqualified or qualified function ARN. If you want to use the same
function version for the entire batch job, configure a specific function version in the `FunctionARN`
parameter when you create your job. If you configure an alias or the $LATEST qualifier, the batch job immediately
starts calling the new version of the function if the alias or $LATEST is updated during the job execution.
Note that you can't reuse an existing Amazon S3 event-based function for batch operations. This is because the Amazon S3
batch operation passes a different event parameter to the Lambda function and expects a return message with a
specific JSON structure.
In the [resource-based policy](./access-control-resource-based.html) that you create for the Amazon S3
batch job, ensure that you set permission for the job to invoke your Lambda function.
In the execution role for the function, set a [trust policy for Amazon S3 to assume the role when it runs your
function](https://docs.aws.amazon.com/AmazonS3/latest/userguide/batch-ops-iam-role-policies.html).
If your function uses the AWS SDK to manage Amazon S3 resources, you need to add Amazon S3 permissions in the execution role.
When the job runs, Amazon S3 starts multiple function instances to process the Amazon S3 objects in parallel, up to
the [concurrency limit](./lambda-concurrency.html) of the function. Amazon S3 limits the initial ramp-up of instances
to avoid excess cost for smaller jobs.
If the Lambda function returns a `TemporaryFailure` response code, Amazon S3 retries the operation.
For more information about Amazon S3 batch operations, see [Performing batch operations](https://docs.aws.amazon.com/AmazonS3/latest/dev/batch-ops.html) in the
*Amazon S3 Developer Guide*.
For an example of how to use a Lambda function in Amazon S3 batch operations, see [Invoking a Lambda function from Amazon S3
batch operations](https://docs.aws.amazon.com/AmazonS3/latest/dev/batch-ops-invoke-lambda.html) in the *Amazon S3 Developer Guide*.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Step Functions
SNS
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.